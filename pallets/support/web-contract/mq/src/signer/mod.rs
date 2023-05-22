use alloc::vec::Vec;

use crate::SignedMessage;

pub trait MessageSigner {
    fn sign(&self, data: &[u8]) -> Vec<u8>;
}
pub trait MessageVerifier {
    fn verify(&self, message: &SignedMessage) -> bool;
}

#[cfg(feature = "signers")]
pub mod signers {
    use super::MessageSigner;
    use alloc::vec::Vec;
    use sp_core::{crypto::Pair as PairTrait, sr25519};
    use serde::{Serialize, Deserialize};
    use serde_more as more;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Sr25519Signer {
        #[serde(with = "more::key_bytes")]
        key: sr25519::Pair,
    }

    impl MessageSigner for Sr25519Signer {
        fn sign(&self, data: &[u8]) -> Vec<u8> {
            self.key.sign(data).0.to_vec()
        }
    }

    impl From<sr25519::Pair> for Sr25519Signer {
        fn from(key: sr25519::Pair) -> Self {
            Self { key }
        }
    }
}
