use crate::{
	crypto::{AuthorityPen, AuthorityVerifier, Signature},
	NodeCount, NodeIndex, SignatureSet,
};

/// Keychain combines an AuthorityPen and AuthorityVerifier into one object implementing the SelendraBFT
/// MultiKeychain trait.
#[derive(Clone)]
pub struct Keychain {
	id: NodeIndex,
	authority_pen: AuthorityPen,
	authority_verifier: AuthorityVerifier,
}

impl Keychain {
	/// Constructs a new keychain from a signing contraption and verifier, with the specified node
	/// index.
	pub fn new(
		id: NodeIndex,
		authority_verifier: AuthorityVerifier,
		authority_pen: AuthorityPen,
	) -> Self {
		Keychain { id, authority_pen, authority_verifier }
	}

	fn index(&self) -> NodeIndex {
		self.id
	}

	fn node_count(&self) -> NodeCount {
		self.authority_verifier.node_count()
	}

	async fn sign(&self, msg: &[u8]) -> Signature {
		self.authority_pen.sign(msg).await
	}

	fn verify<I: Into<NodeIndex>>(&self, msg: &[u8], sgn: &Signature, index: I) -> bool {
		self.authority_verifier.verify(msg, sgn, index.into())
	}

	fn is_complete(&self, msg: &[u8], partial: &SignatureSet<Signature>) -> bool {
		self.authority_verifier.is_complete(msg, partial)
	}
}

impl current_selendra_bft::Index for Keychain {
	fn index(&self) -> current_selendra_bft::NodeIndex {
		Keychain::index(self).into()
	}
}

impl legacy_selendra_bft::Index for Keychain {
	fn index(&self) -> legacy_selendra_bft::NodeIndex {
		Keychain::index(self).into()
	}
}

#[async_trait::async_trait]
impl current_selendra_bft::Keychain for Keychain {
	type Signature = Signature;

	fn node_count(&self) -> current_selendra_bft::NodeCount {
		Keychain::node_count(self).into()
	}

	async fn sign(&self, msg: &[u8]) -> Signature {
		Keychain::sign(self, msg).await
	}

	fn verify(&self, msg: &[u8], sgn: &Signature, index: current_selendra_bft::NodeIndex) -> bool {
		Keychain::verify(self, msg, sgn, index)
	}
}

#[async_trait::async_trait]
impl legacy_selendra_bft::Keychain for Keychain {
	type Signature = Signature;

	fn node_count(&self) -> legacy_selendra_bft::NodeCount {
		Keychain::node_count(self).into()
	}

	async fn sign(&self, msg: &[u8]) -> Signature {
		Keychain::sign(self, msg).await
	}

	fn verify(&self, msg: &[u8], sgn: &Signature, index: legacy_selendra_bft::NodeIndex) -> bool {
		Keychain::verify(self, msg, sgn, index)
	}
}

impl current_selendra_bft::MultiKeychain for Keychain {
	// Using `SignatureSet` is slow, but Substrate has not yet implemented aggregation.
	// We probably should do this for them at some point.
	type PartialMultisignature = SignatureSet<Signature>;

	fn bootstrap_multi(
		&self,
		signature: &Signature,
		index: current_selendra_bft::NodeIndex,
	) -> Self::PartialMultisignature {
		current_selendra_bft::PartialMultisignature::add_signature(
			SignatureSet(legacy_selendra_bft::SignatureSet::with_size(
				legacy_selendra_bft::Keychain::node_count(self),
			)),
			signature,
			index,
		)
	}

	fn is_complete(&self, msg: &[u8], partial: &Self::PartialMultisignature) -> bool {
		Keychain::is_complete(self, msg, partial)
	}
}

impl legacy_selendra_bft::MultiKeychain for Keychain {
	// Using `SignatureSet` is slow, but Substrate has not yet implemented aggregation.
	// We probably should do this for them at some point.
	type PartialMultisignature = SignatureSet<Signature>;

	fn bootstrap_multi(
		&self,
		signature: &Signature,
		index: legacy_selendra_bft::NodeIndex,
	) -> Self::PartialMultisignature {
		legacy_selendra_bft::PartialMultisignature::add_signature(
			SignatureSet(legacy_selendra_bft::SignatureSet::with_size(
				legacy_selendra_bft::Keychain::node_count(self),
			)),
			signature,
			index,
		)
	}

	fn is_complete(&self, msg: &[u8], partial: &Self::PartialMultisignature) -> bool {
		Keychain::is_complete(self, msg, partial)
	}
}
