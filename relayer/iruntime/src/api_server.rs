use std::str;

use indratory_api::irpc::indratory_api_server::IndratoryAPIMethod;
use rocket::{
	data::{ByteUnit, Data, Limits, ToByteUnit},
	fs::{FileServer, Options},
	get,
	http::{ContentType, Method, Status},
	post,
	response::status::Custom,
	routes,
	serde::json::{json, Json, Value as JsonValue},
	Build, Phase, Rocket,
};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tracing::{error, info, instrument};

use indranet_rocket_middleware::{RequestTracer, ResponseSigner, TimeMeter, TraceId};
use indratory_api::{actions, irpc};

use crate::runtime;

#[derive(Serialize, Deserialize)]
struct ContractInput {
	input: Map<String, Value>,
	nonce: Map<String, Value>,
}

macro_rules! do_ecall_handle {
    ($id: expr, $num: expr, $content: expr) => {{
        match runtime::ecall_handle($id, $num, $content) {
            Ok(data) => {
                let output_value: serde_json::value::Value = serde_json::from_slice(&data).unwrap();
                json!(output_value)
            },
            Err(err) => {
                error!("Call ecall_handle failed: {:?}!", err);
                json!({
                    "status": "error",
                    "payload": format!("{:?}!", err)
                })
            }
        }
    }};
}

macro_rules! proxy_post {
	($rpc: literal, $name: ident, $num: expr) => {
		#[instrument(target="irpc", fields(%id), skip_all)]
		#[post($rpc, format = "json", data = "<contract_input>")]
		fn $name(id: TraceId, contract_input: Json<ContractInput>) -> JsonValue {
			let input_string = serde_json::to_string(&*contract_input).unwrap();
			do_ecall_handle!(id.id(), $num, input_string.as_bytes())
		}
	};
}

macro_rules! proxy_get {
	($rpc: literal, $name: ident, $num: expr) => {
		#[instrument(target="irpc", fields(%id), skip_all)]
		#[get($rpc)]
		fn $name(id: TraceId) -> JsonValue {
			let input_string = r#"{ "input": {} }"#.to_string();
			do_ecall_handle!(id.id(), $num, input_string.as_bytes())
		}
	};
}

macro_rules! proxy {
	(post, $rpc: literal, $name: ident, $num: expr) => {
		proxy_post!($rpc, $name, $num)
	};
	(get, $rpc: literal, $name: ident, $num: expr) => {
		proxy_get!($rpc, $name, $num)
	};
}

enum ReadData {
	Ok(Vec<u8>),
	IoError,
	PayloadTooLarge,
}

async fn read_data(data: Data<'_>, limit: ByteUnit) -> ReadData {
	let stream = data.open(limit);
	let data = match stream.into_bytes().await {
		Ok(data) => data,
		Err(_) => return ReadData::IoError,
	};
	if !data.is_complete() {
		return ReadData::PayloadTooLarge
	}
	ReadData::Ok(data.into_inner())
}

macro_rules! proxy_bin {
    ($rpc: literal, $name: ident, $num: expr) => {
        #[instrument(target="irpc", fields(%id), skip_all)]
        #[post($rpc, data = "<data>")]
        async fn $name(id: TraceId, data: Data<'_>, limits: &Limits) -> JsonValue {
            let limit = limits.get(stringify!($name)).unwrap_or(100.mebibytes());
            let data = match read_data(data, limit).await {
                ReadData::Ok(data) => data,
                ReadData::IoError => {
                    return json!({
                        "status": "error",
                        "payload": "Io error: Read input data failed"
                    })
                }
                ReadData::PayloadTooLarge => {
                    return json!({
                        "status": "error",
                        "payload": "Entity too large"
                    })
                }
            };
            do_ecall_handle!(id.id(), $num, &data)
        }
    };
}

macro_rules! proxy_routes {
    ($(($m: ident, $rpc: literal, $name: ident, $num: expr),)+) => {{
        $(proxy!($m, $rpc, $name, $num);)+
        routes![$($name),+]
    }};
}

macro_rules! proxy_bin_routes {
    ($(($rpc: literal, $name: ident, $num: expr),)+) => {{
        $(proxy_bin!($rpc, $name, $num);)+
        routes![$($name),+]
    }};
}

#[post("/kick")]
fn kick() -> String {
	info!("Kicked by the operator");
	std::process::exit(0);
}

#[instrument(target="irpc", fields(id=%_id), skip_all)]
#[get("/info")]
fn getinfo(_id: TraceId) -> String {
	runtime::ecall_getinfo()
}

#[get("/help")]
fn help() -> String {
	indratory_api::irpc::PROTO_DEF.to_string()
}

enum RpcType {
	Public,
	Private,
}

impl RpcType {
	fn is_public(&self) -> bool {
		match self {
			RpcType::Public => true,
			RpcType::Private => false,
		}
	}
}

fn rpc_type(method: &str) -> RpcType {
	use IndratoryAPIMethod::*;
	use RpcType::*;
	match IndratoryAPIMethod::from_str(method) {
		None => Private,
		Some(method) => match method {
			SyncHeader => Private,
			SyncParaHeader => Private,
			SyncCombinedHeaders => Private,
			DispatchBlocks => Private,
			InitRuntime => Private,
			GetRuntimeInfo => Private,
			GetEgressMessages => Private,
			GetWorkerState => Private,
			AddEndpoint => Private,
			RefreshEndpointSigningTime => Private,
			GetEndpointInfo => Private,
			SignEndpointInfo => Private,
			DeriveIndranetI2pKey => Private,
			Echo => Private,
			HandoverCreateChallenge => Private,
			HandoverStart => Private,
			HandoverAcceptChallenge => Private,
			HandoverReceive => Private,
			ConfigNetwork => Private,
			HttpFetch => Private,
			GetNetworkConfig => Private,
			LoadChainState => Private,
			Stop => Private,
			LoadStorageProof => Private,
			TakeCheckpoint => Private,

			GetInfo => Public,
			ContractQuery => Public,
			GetContractInfo => Public,
			GetClusterInfo => Public,
			UploadSidevmCode => Public,
			CalculateContractId => Public,
			Statistics => Public,

			GenerateClusterStateRequest => Private,
			SaveClusterState => Public,
			LoadClusterState => Private,
		},
	}
}

fn default_payload_limit_for_method(method: IndratoryAPIMethod) -> ByteUnit {
	use IndratoryAPIMethod::*;

	match method {
		GetInfo => 1.kibibytes(),
		SyncHeader => 100.mebibytes(),
		SyncParaHeader => 100.mebibytes(),
		SyncCombinedHeaders => 100.mebibytes(),
		DispatchBlocks => 100.mebibytes(),
		InitRuntime => 10.mebibytes(),
		GetRuntimeInfo => 1.kibibytes(),
		GetEgressMessages => 1.kibibytes(),
		ContractQuery => 500.kibibytes(),
		GetWorkerState => 1.kibibytes(),
		AddEndpoint => 10.kibibytes(),
		RefreshEndpointSigningTime => 10.kibibytes(),
		GetEndpointInfo => 1.kibibytes(),
		DeriveIndranetI2pKey => 10.kibibytes(),
		Echo => 1.mebibytes(),
		HandoverCreateChallenge => 10.kibibytes(),
		HandoverStart => 10.kibibytes(),
		HandoverAcceptChallenge => 10.kibibytes(),
		HandoverReceive => 10.kibibytes(),
		SignEndpointInfo => 32.kibibytes(),
		ConfigNetwork => 10.kibibytes(),
		HttpFetch => 100.mebibytes(),
		GetContractInfo => 100.kibibytes(),
		GetClusterInfo => 1.kibibytes(),
		UploadSidevmCode => 32.mebibytes(),
		CalculateContractId => 1.kibibytes(),
		GetNetworkConfig => 1.kibibytes(),
		LoadChainState => 500.mebibytes(),
		Stop => 1.kibibytes(),
		LoadStorageProof => 10.mebibytes(),
		TakeCheckpoint => 1.kibibytes(),
		Statistics => 100.kibibytes(),

		GenerateClusterStateRequest => 1.kibibytes(),
		SaveClusterState => 1.kibibytes(),
		LoadClusterState => 1.kibibytes(),
	}
}

fn limit_for_method(method: &str, limits: &Limits) -> ByteUnit {
	if let Some(v) = limits.get(method) {
		return v
	}
	match IndratoryAPIMethod::from_str(method) {
		None => 1.mebibytes(),
		Some(method) => default_payload_limit_for_method(method),
	}
}

#[instrument(target="irpc", name="irpc", fields(%id), skip_all)]
#[post("/<method>?<json>", data = "<data>")]
async fn irpc_proxy(
	id: TraceId,
	method: String,
	data: Data<'_>,
	limits: &Limits,
	content_type: Option<&ContentType>,
	json: bool,
) -> Custom<Vec<u8>> {
	irpc_proxy_inner(id.id(), method, data, limits, content_type, json).await
}

async fn irpc_proxy_inner(
	id: u64,
	method: String,
	data: Data<'_>,
	limits: &Limits,
	content_type: Option<&ContentType>,
	json: bool,
) -> Custom<Vec<u8>> {
	let limit = limit_for_method(&method, limits);
	let data = match read_data(data, limit).await {
		ReadData::Ok(data) => data,
		ReadData::IoError =>
			return Custom(Status::ServiceUnavailable, b"Read body failed".to_vec()),
		ReadData::PayloadTooLarge =>
			return Custom(Status::PayloadTooLarge, b"Entity too large".to_vec()),
	};
	let json = json || content_type.map(|t| t.is_json()).unwrap_or(false);
	info!("Payload size: {}", data.len());
	irpc_call(id, method, &data, json).await
}

async fn irpc_call(id: u64, method: String, data: &[u8], json: bool) -> Custom<Vec<u8>> {
	let (status_code, output) = runtime::ecall_irpc_request(id, method, data, json).await;
	if let Some(status) = Status::from_code(status_code) {
		Custom(status, output)
	} else {
		error!(status_code, "irpc: Invalid status code!");
		Custom(Status::ServiceUnavailable, vec![])
	}
}

#[instrument(target="irpc", name="irpc", fields(%id), skip_all)]
#[post("/<method>?<json>", data = "<data>")]
async fn irpc_proxy_acl(
	id: TraceId,
	method: String,
	data: Data<'_>,
	limits: &Limits,
	content_type: Option<&ContentType>,
	json: bool,
) -> Custom<Vec<u8>> {
	info!(method, "irpc enter");
	if !rpc_type(&method).is_public() {
		error!("irpc_acl: access denied");
		return Custom(Status::Forbidden, vec![])
	}
	irpc_proxy_inner(id.id(), method, data, limits, content_type, json).await
}

#[instrument(target="irpc", name="irpc", fields(%id), skip_all)]
#[get("/<method>")]
async fn irpc_proxy_get_acl(id: TraceId, method: String) -> Custom<Vec<u8>> {
	info!(method, "irpc_acl get enter");
	if !rpc_type(&method).is_public() {
		error!("irpc_acl: access denied");
		return Custom(Status::Forbidden, vec![])
	}
	irpc_call(id.id(), method, b"", true).await
}

#[instrument(target="irpc", name="irpc", fields(%id), skip_all)]
#[get("/<method>")]
async fn irpc_proxy_get(id: TraceId, method: String) -> Custom<Vec<u8>> {
	irpc_call(id.id(), method, b"", true).await
}

fn cors_options() -> CorsOptions {
	let allowed_origins = AllowedOrigins::all();
	let allowed_methods: AllowedMethods =
		vec![Method::Get, Method::Post].into_iter().map(From::from).collect();

	// You can also deserialize this
	rocket_cors::CorsOptions {
		allowed_origins,
		allowed_methods,
		allowed_headers: AllowedHeaders::all(),
		allow_credentials: true,
		..Default::default()
	}
}

fn print_rpc_methods(prefix: &str, methods: &[&str]) {
	info!("Methods under {}:", prefix);
	for method in methods {
		info!("    {}", format!("{prefix}/{method}"));
	}
}

fn mount_static_file_server(builer: Rocket<Build>, storage_path: &str) -> Rocket<Build> {
	let data_dir = indratory::public_data_dir(storage_path);
	builer.mount("/download", FileServer::new(data_dir, Options::Missing))
}

pub(super) fn rocket(args: &super::Args, storage_path: &str) -> rocket::Rocket<impl Phase> {
	let mut server = rocket::build()
		.mount(
			"/",
			proxy_routes![
				(get, "/get_info", get_info, actions::ACTION_GET_INFO),
				(post, "/get_info", get_info_post, actions::ACTION_GET_INFO),
			],
		)
		.mount(
			"/bin_api",
			proxy_bin_routes![
				("/sync_header", sync_header, actions::BIN_ACTION_SYNC_HEADER),
				("/dispatch_block", dispatch_block, actions::BIN_ACTION_DISPATCH_BLOCK),
				(
					"/sync_combined_headers",
					sync_combined_headers,
					actions::BIN_ACTION_SYNC_COMBINED_HEADERS
				),
			],
		)
		.mount("/", routes![getinfo, help]);

	if args.enable_kick_api {
		info!("ENABLE `kick` API");

		server = server.mount("/", routes![kick]);
	}

	server = server.mount("/irpc", routes![irpc_proxy, irpc_proxy_get]);
	print_rpc_methods("/irpc", irpc::indratory_api_server::supported_methods());

	if args.allow_cors {
		info!("Allow CORS");
		server = server
			.mount("/", rocket_cors::catch_all_options_routes()) // mount the catch all routes
			.attach(cors_options().to_cors().expect("To not fail"))
			.manage(cors_options().to_cors().expect("To not fail"));
	}
	server = server.attach(TimeMeter).attach(RequestTracer);
	server = mount_static_file_server(server, storage_path);
	server
}

/// api endpoint with access control, will be exposed to the public
pub(super) fn rocket_acl(
	args: &super::Args,
	storage_path: &str,
) -> Option<rocket::Rocket<impl Phase>> {
	let public_port: u16 = if args.public_port.is_some() {
		args.public_port.expect("public_port should be set")
	} else {
		return None
	};

	let figment = rocket::Config::figment()
		.merge(("address", "0.0.0.0"))
		.merge(("port", public_port))
		.merge(("limits", Limits::new().limit("json", 100.mebibytes())));

	let mut server_acl = rocket::custom(figment).mount("/", routes![getinfo, help]);

	server_acl = server_acl.mount("/irpc", routes![irpc_proxy_acl, irpc_proxy_get_acl]);

	if args.allow_cors {
		info!("Allow CORS");

		server_acl = server_acl
			.mount("/", rocket_cors::catch_all_options_routes()) // mount the catch all routes
			.attach(cors_options().to_cors().expect("To not fail"))
			.manage(cors_options().to_cors().expect("To not fail"));
	}

	let signer = ResponseSigner::new(1024 * 1024 * 10, runtime::ecall_sign_http_response);
	server_acl = server_acl.attach(signer).attach(RequestTracer).attach(TimeMeter);
	server_acl = mount_static_file_server(server_acl, storage_path);
	Some(server_acl)
}
