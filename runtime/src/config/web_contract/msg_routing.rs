use crate::{pallet_mq, PhalaComputation, PhalaPhatContracts, PhalaRegistry};
use codec::Decode;
use frame_support::dispatch::{DispatchError, DispatchResult};
use web_contract_types::messaging::{BindTopic, DecodedMessage, Message};

pub struct MessageRouteConfig;

fn try_dispatch<Msg, Func>(func: Func, message: &Message) -> DispatchResult
where
	Msg: Decode + BindTopic,
	Func: Fn(DecodedMessage<Msg>) -> DispatchResult,
{
	if message.destination.path() == &Msg::topic() {
		let msg: DecodedMessage<Msg> =
			message.decode().ok_or(DispatchError::Other("MessageCodecError"))?;
		return (func)(msg)
	}
	Ok(())
}

impl pallet_mq::QueueNotifyConfig for MessageRouteConfig {
	/// Handles an incoming message
	fn on_message_received(message: &Message) -> DispatchResult {
		macro_rules! route_handlers {
            ($($handler: path,)+) => {
                $(try_dispatch($handler, message)?;)+
            }
        }

		route_handlers! {
			PhalaRegistry::on_message_received,
			PhalaRegistry::on_gk_message_received,
			PhalaComputation::on_gk_message_received,
			PhalaComputation::on_working_message_received,
			PhalaPhatContracts::on_worker_cluster_message_received,
			PhalaPhatContracts::on_cluster_message_received,
			PhalaPhatContracts::on_contract_message_received,
			// BridgeTransfer::on_message_received,
		};
		Ok(())
	}
}
