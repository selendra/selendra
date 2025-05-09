use std::{collections::HashSet, marker::PhantomData, mem::size_of};

use log::{debug, warn};
use parity_scale_codec::{Decode, Encode, Error as CodecError, Input as CodecInput};
use static_assertions::const_assert;

use crate::{
    selendra_primitives::MAX_BLOCK_SIZE,
    block::{Block, Header, Justification, UnverifiedHeader, UnverifiedHeaderFor},
    network::GossipNetwork,
    sync::{PeerId, LOG_TARGET},
    BlockId, Version,
};

/// The representation of the database state to be sent to other nodes.
#[derive(Clone, Debug, Encode, Decode)]
pub struct State<J: Justification> {
    top_justification: J::Unverified,
    favourite_block: UnverifiedHeaderFor<J>,
}

impl<J: Justification> State<J> {
    pub fn new(top_justification: J::Unverified, favourite_block: UnverifiedHeaderFor<J>) -> Self {
        State {
            top_justification,
            favourite_block,
        }
    }

    pub fn top_justification(&self) -> J::Unverified {
        self.top_justification.clone()
    }

    pub fn favourite_block(&self) -> UnverifiedHeaderFor<J> {
        self.favourite_block.clone()
    }
}

/// Represents one of the possible response_items we are sending over the network.
#[derive(Clone, Debug, Encode, Decode)]
pub enum ResponseItem<B, J>
where
    J: Justification,
    B: Block<UnverifiedHeader = UnverifiedHeaderFor<J>>,
{
    Justification(J::Unverified),
    Header(UnverifiedHeaderFor<J>),
    Block(B),
}

/// Things we send over the network as a response to the request.
pub type ResponseItems<B, J> = Vec<ResponseItem<B, J>>;

/// Additional information about the branch connecting the top finalized block
/// with a given one. All the variants are exhaustive and exclusive due to the
/// properties of the `Forest` structure.
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub enum BranchKnowledge {
    /// ID of the oldest known ancestor if none of them are imported.
    /// It must be different from the, imported by definition, root.
    LowestId(BlockId),
    /// ID of the top imported ancestor if any of them is imported.
    /// Since imported vertices are connected to the root, the oldest known
    /// ancestor is, implicitly, the root.
    TopImported(BlockId),
}

/// Request content, current version.
#[derive(Clone, Debug, Encode, Decode)]
pub struct Request<J: Justification> {
    target: UnverifiedHeaderFor<J>,
    branch_knowledge: BranchKnowledge,
    state: State<J>,
}

impl<J: Justification> Request<J> {
    pub fn new(
        target: UnverifiedHeaderFor<J>,
        branch_knowledge: BranchKnowledge,
        state: State<J>,
    ) -> Self {
        Request {
            target,
            branch_knowledge,
            state,
        }
    }
}

impl<J: Justification> Request<J> {
    pub fn state(&self) -> &State<J> {
        &self.state
    }
    pub fn target(&self) -> &UnverifiedHeaderFor<J> {
        &self.target
    }
    pub fn branch_knowledge(&self) -> &BranchKnowledge {
        &self.branch_knowledge
    }
}

/// Data that can be used to generate a request given our state.
pub struct PreRequest<UH: UnverifiedHeader, I: PeerId> {
    header: UH,
    branch_knowledge: BranchKnowledge,
    know_most: HashSet<I>,
}

impl<UH: UnverifiedHeader, I: PeerId> PreRequest<UH, I> {
    pub fn new(header: UH, branch_knowledge: BranchKnowledge, know_most: HashSet<I>) -> Self {
        PreRequest {
            header,
            branch_knowledge,
            know_most,
        }
    }

    /// Convert to a request and recipients given a state.
    pub fn with_state<J>(self, state: State<J>) -> (Request<J>, HashSet<I>)
    where
        J: Justification,
        J::Header: Header<Unverified = UH>,
    {
        let PreRequest {
            header,
            branch_knowledge,
            know_most,
        } = self;
        (Request::new(header, branch_knowledge, state), know_most)
    }
}

/// Data to be sent over the network, current version.
#[derive(Clone, Debug, Encode, Decode)]
pub enum NetworkData<B: Block, J: Justification>
where
    J: Justification,
    B: Block<UnverifiedHeader = UnverifiedHeaderFor<J>>,
{
    /// A periodic state broadcast, so that neighbouring nodes can request what they are missing,
    /// send what we are missing, and sometimes just use the justifications to update their own
    /// state.
    StateBroadcast(State<J>),
    /// Response to a state broadcast. Contains at most two justifications that the peer will
    /// understand.
    StateBroadcastResponse(J::Unverified, Option<J::Unverified>),
    /// An explicit request for data, potentially a lot of it.
    Request(Request<J>),
    /// Response to the request for data.
    RequestResponse(ResponseItems<B, J>),
    /// A request for a chain extension.
    ChainExtensionRequest(State<J>),
}

/// Version wrapper around the network data.
#[derive(Clone, Debug)]
pub enum VersionedNetworkData<B: Block, J: Justification>
where
    J: Justification,
    B: Block<UnverifiedHeader = UnverifiedHeaderFor<J>>,
{
    // Most likely from the future.
    Other(Version, Vec<u8>),
    V4(NetworkData<B, J>),
}

// We need 32 bits, since blocks can be quite sizeable.
type ByteCount = u32;

// We agreed to 15mb + some wiggle room for sync message.
// Maximum block size is 5mb so we have spare for at least 3 blocks.
pub const MAX_SYNC_MESSAGE_SIZE: u32 = 15 * 1024 * 1024 + 1024;
const_assert!(MAX_SYNC_MESSAGE_SIZE > 3 * MAX_BLOCK_SIZE);
/// The toal maximal message size that should be accepted, slighly
/// larger than the above to include the version plus some wiggle-room.
pub const MAX_MESSAGE_SIZE: u64 = 16 * 1024 * 1024;
const_assert!(MAX_MESSAGE_SIZE > MAX_SYNC_MESSAGE_SIZE as u64 + 128);

fn encode_with_version(version: Version, payload: &[u8]) -> Vec<u8> {
    let size = payload.len().try_into().unwrap_or(ByteCount::MAX);

    if size > MAX_SYNC_MESSAGE_SIZE {
        warn!(
            target: LOG_TARGET,
            "Versioned sync message v{:?} too big during Encode. Size is {:?}. Should be {:?} at max.",
            version,
            payload.len(),
            MAX_SYNC_MESSAGE_SIZE
        );
    }

    let mut result = Vec::with_capacity(version.size_hint() + size.size_hint() + payload.len());

    version.encode_to(&mut result);
    size.encode_to(&mut result);
    result.extend_from_slice(payload);

    result
}

impl<B, J> Encode for VersionedNetworkData<B, J>
where
    J: Justification,
    B: Block<UnverifiedHeader = UnverifiedHeaderFor<J>>,
{
    fn size_hint(&self) -> usize {
        use VersionedNetworkData::*;
        let version_size = size_of::<Version>();
        let byte_count_size = size_of::<ByteCount>();
        version_size
            + byte_count_size
            + match self {
                Other(_, payload) => payload.len(),
                V4(data) => data.size_hint(),
            }
    }

    fn encode(&self) -> Vec<u8> {
        use VersionedNetworkData::*;
        match self {
            Other(version, payload) => encode_with_version(*version, payload),
            V4(data) => encode_with_version(Version(4), &data.encode()),
        }
    }
}

impl<B, J> Decode for VersionedNetworkData<B, J>
where
    J: Justification,
    B: Block<UnverifiedHeader = UnverifiedHeaderFor<J>>,
{
    fn decode<I: CodecInput>(input: &mut I) -> Result<Self, CodecError> {
        use VersionedNetworkData::*;
        let version = Version::decode(input)?;
        let num_bytes = ByteCount::decode(input)?;
        match version {
            Version(4) => Ok(V4(NetworkData::decode(input)?)),
            _ => {
                if num_bytes > MAX_SYNC_MESSAGE_SIZE {
                    Err("Sync message has unknown version and is encoded as more than the maximum size.")?;
                };
                let mut payload = vec![0; num_bytes as usize];
                input.read(payload.as_mut_slice())?;
                Ok(Other(version, payload))
            }
        }
    }
}

/// Wrap around a network to avoid thinking about versioning.
pub struct VersionWrapper<B, J, N>
where
    N: GossipNetwork<VersionedNetworkData<B, J>>,
    J: Justification,
    B: Block<UnverifiedHeader = UnverifiedHeaderFor<J>>,
{
    inner: N,
    _phantom: PhantomData<(B, J)>,
}

impl<B, J, N> VersionWrapper<B, J, N>
where
    N: GossipNetwork<VersionedNetworkData<B, J>>,
    J: Justification,
    B: Block<UnverifiedHeader = UnverifiedHeaderFor<J>>,
{
    /// Wrap the inner network.
    pub fn new(inner: N) -> Self {
        VersionWrapper {
            inner,
            _phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<B, J, N> GossipNetwork<NetworkData<B, J>> for VersionWrapper<B, J, N>
where
    N: GossipNetwork<VersionedNetworkData<B, J>>,
    J: Justification,
    B: Block<UnverifiedHeader = UnverifiedHeaderFor<J>>,
{
    type Error = N::Error;
    type PeerId = N::PeerId;

    fn send_to(
        &mut self,
        data: NetworkData<B, J>,
        peer_id: Self::PeerId,
    ) -> Result<(), Self::Error> {
        self.inner.send_to(VersionedNetworkData::V4(data), peer_id)
    }

    fn send_to_random(
        &mut self,
        data: NetworkData<B, J>,
        peer_ids: HashSet<Self::PeerId>,
    ) -> Result<(), Self::Error> {
        self.inner
            .send_to_random(VersionedNetworkData::V4(data), peer_ids)
    }

    fn broadcast(&mut self, data: NetworkData<B, J>) -> Result<(), Self::Error> {
        self.inner.broadcast(VersionedNetworkData::V4(data))
    }

    /// Retrieves next message from the network.
    ///
    /// # Cancel safety
    ///
    /// This method is cancellation safe.
    async fn next(&mut self) -> Result<(NetworkData<B, J>, Self::PeerId), Self::Error> {
        loop {
            match self.inner.next().await? {
                (VersionedNetworkData::Other(version, _), _) => {
                    debug!(
                        target: LOG_TARGET,
                        "Received sync data of unsupported version {:?}.", version
                    )
                }
                (VersionedNetworkData::V4(data), peer_id) => return Ok((data, peer_id)),
            }
        }
    }
}
