use parity_scale_codec::Encode;
use indranet_types::messaging::SignedMessage;
use subxt::{tx::TxPayload, utils::Encoded};

pub struct EncodedPayload {
    pallet_name: &'static str,
    call_name: &'static str,
    call_data: Encoded,
}

impl EncodedPayload {
    pub fn new(pallet_name: &'static str, call_name: &'static str, call_data: Vec<u8>) -> Self {
        Self {
            pallet_name,
            call_name,
            call_data: Encoded(call_data),
        }
    }
}

impl TxPayload for EncodedPayload {
    fn encode_call_data_to(
        &self,
        metadata: &subxt::Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), subxt::Error> {
        let pallet = metadata.pallet(self.pallet_name)?;
        let call = pallet.call(self.call_name)?;

        let pallet_index = pallet.index();
        let call_index = call.index();

        pallet_index.encode_to(out);
        call_index.encode_to(out);
        self.call_data.encode_to(out);
        Ok(())
    }
}

pub fn register_worker(iruntime_info: Vec<u8>, attestation: Vec<u8>, v2: bool) -> EncodedPayload {
    let call_name = if v2 {
        "register_worker_v2"
    } else {
        "register_worker"
    };
    EncodedPayload::new(
        "IndranetRegistry",
        call_name,
        (Encoded(iruntime_info), Encoded(attestation)).encode(),
    )
}

pub fn update_worker_endpoint(signed_endpoint: Vec<u8>, signature: Vec<u8>) -> EncodedPayload {
    let args = (Encoded(signed_endpoint), signature).encode();
    EncodedPayload::new("IndranetRegistry", "update_worker_endpoint", args)
}

pub fn sync_offchain_message(message: SignedMessage) -> EncodedPayload {
    let args = message.encode();
    EncodedPayload::new("IndranetMq", "sync_offchain_message", args)
}
