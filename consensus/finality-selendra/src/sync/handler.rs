use std::fmt::{Display, Error as FmtError, Formatter};

use crate::{
	session::{last_block_of_session, session_id_from_block_num, SessionId, SessionPeriod},
	sync::{
		data::{NetworkData, Request, State},
		forest::{Error as ForestError, Forest, Interest},
		BlockIdFor, BlockIdentifier, ChainStatus, Finalizer, Header, Justification, PeerId,
		Verifier,
	},
};

/// How many justifications we will send at most in response to an explicit query.
const MAX_JUSTIFICATION_BATCH: usize = 100;

/// Handler for data incoming from the network.
pub struct Handler<I: PeerId, J: Justification, CS: ChainStatus<J>, V: Verifier<J>, F: Finalizer<J>>
{
	chain_status: CS,
	verifier: V,
	finalizer: F,
	forest: Forest<I, J>,
	period: SessionPeriod,
}

/// What actions can the handler recommend as a reaction to some data.
#[derive(Clone, Debug)]
pub enum SyncAction<J: Justification> {
	/// A response for the peer that sent us the data.
	Response(NetworkData<J>),
	/// A task that should be performed periodically. At the moment these are only requests for blocks,
	/// so it always contains the id of the block.
	Task(BlockIdFor<J>),
	/// Do nothing.
	Noop,
}

impl<J: Justification> SyncAction<J> {
	fn noop() -> Self {
		SyncAction::Noop
	}

	fn state_broadcast_response(
		justification: J::Unverified,
		other_justification: Option<J::Unverified>,
	) -> Self {
		SyncAction::Response(NetworkData::StateBroadcastResponse(
			justification,
			other_justification,
		))
	}

	fn request_response(justifications: Vec<J::Unverified>) -> Self {
		SyncAction::Response(NetworkData::RequestResponse(justifications))
	}

	fn task(id: BlockIdFor<J>) -> Self {
		SyncAction::Task(id)
	}
}

/// What can go wrong when handling a piece of data.
#[derive(Clone, Debug)]
pub enum Error<J: Justification, CS: ChainStatus<J>, V: Verifier<J>, F: Finalizer<J>> {
	Verifier(V::Error),
	ChainStatus(CS::Error),
	Finalizer(F::Error),
	Forest(ForestError),
	MissingJustification,
}

impl<J: Justification, CS: ChainStatus<J>, V: Verifier<J>, F: Finalizer<J>> Display
	for Error<J, CS, V, F>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
		use Error::*;
		match self {
			Verifier(e) => write!(f, "verifier error: {}", e),
			ChainStatus(e) => write!(f, "chain status error: {}", e),
			Finalizer(e) => write!(f, "finalized error: {}", e),
			Forest(e) => write!(f, "forest error: {}", e),
			MissingJustification =>
				write!(f, "justification for the last block of a past session missing"),
		}
	}
}

impl<J: Justification, CS: ChainStatus<J>, V: Verifier<J>, F: Finalizer<J>> From<ForestError>
	for Error<J, CS, V, F>
{
	fn from(e: ForestError) -> Self {
		Error::Forest(e)
	}
}

impl<I: PeerId, J: Justification, CS: ChainStatus<J>, V: Verifier<J>, F: Finalizer<J>>
	Handler<I, J, CS, V, F>
{
	/// New handler with the provided chain interfaces.
	pub fn new(
		chain_status: CS,
		verifier: V,
		finalizer: F,
		period: SessionPeriod,
	) -> Result<Self, Error<J, CS, V, F>> {
		let top_finalized = chain_status.top_finalized().map_err(Error::ChainStatus)?.header().id();
		let forest = Forest::new(top_finalized);
		Ok(Handler { chain_status, verifier, finalizer, forest, period })
	}

	fn try_finalize(&mut self) -> Result<(), Error<J, CS, V, F>> {
		let mut number = self
			.chain_status
			.top_finalized()
			.map_err(Error::ChainStatus)?
			.header()
			.id()
			.number() + 1;
		loop {
			while let Some(justification) = self.forest.try_finalize(&number) {
				self.finalizer.finalize(justification).map_err(Error::Finalizer)?;
				number += 1;
			}
			number =
				last_block_of_session(session_id_from_block_num(number, self.period), self.period);
			match self.forest.try_finalize(&number) {
				Some(justification) => {
					self.finalizer.finalize(justification).map_err(Error::Finalizer)?;
					number += 1;
				},
				None => return Ok(()),
			};
		}
	}

	fn handle_verified_justification(
		&mut self,
		justification: J,
		peer: I,
	) -> Result<Option<BlockIdFor<J>>, Error<J, CS, V, F>> {
		let id = justification.header().id();
		let maybe_id = match self.forest.update_justification(justification, Some(peer))? {
			true => Some(id),
			false => None,
		};
		self.try_finalize()?;
		Ok(maybe_id)
	}

	/// Inform the handler that a block has been imported.
	pub fn block_imported(&mut self, header: J::Header) -> Result<(), Error<J, CS, V, F>> {
		self.forest.update_body(&header)?;
		self.try_finalize()
	}

	/// Handle a request for potentially substantial amounts of data.
	///
	/// Currently ignores the requested id, it will only become important once we can request
	/// blocks.
	pub fn handle_request(
		&mut self,
		request: Request<J>,
	) -> Result<SyncAction<J>, Error<J, CS, V, F>> {
		let mut number = request.state().top_justification().id().number() + 1;
		let mut justifications = vec![];
		while justifications.len() < MAX_JUSTIFICATION_BATCH {
			match self.chain_status.finalized_at(number).map_err(Error::ChainStatus)? {
				Some(justification) => {
					justifications.push(justification.into_unverified());
					number += 1;
				},
				None => {
					number = last_block_of_session(
						session_id_from_block_num(number, self.period),
						self.period,
					);
					match self.chain_status.finalized_at(number).map_err(Error::ChainStatus)? {
						Some(justification) => {
							justifications.push(justification.into_unverified());
							number += 1;
						},
						None => break,
					};
				},
			}
		}
		Ok(SyncAction::request_response(justifications))
	}

	/// Handle a single justification.
	pub fn handle_justification(
		&mut self,
		justification: J::Unverified,
		peer: I,
	) -> Result<Option<BlockIdFor<J>>, Error<J, CS, V, F>> {
		let justification = self.verifier.verify(justification).map_err(Error::Verifier)?;
		self.handle_verified_justification(justification, peer)
	}

	fn last_justification_unverified(
		&self,
		session: SessionId,
	) -> Result<J::Unverified, Error<J, CS, V, F>> {
		use Error::*;
		Ok(self
			.chain_status
			.finalized_at(last_block_of_session(session, self.period))
			.map_err(ChainStatus)?
			.ok_or(MissingJustification)?
			.into_unverified())
	}

	/// Handle a state broadcast returning the actions we should take in response.
	pub fn handle_state(
		&mut self,
		state: State<J>,
		peer: I,
	) -> Result<SyncAction<J>, Error<J, CS, V, F>> {
		use Error::*;
		let remote_top_number = state.top_justification().id().number();
		let local_top = self.chain_status.top_finalized().map_err(ChainStatus)?;
		let local_top_number = local_top.header().id().number();
		let remote_session = session_id_from_block_num(remote_top_number, self.period);
		let local_session = session_id_from_block_num(local_top_number, self.period);
		match local_session.0.checked_sub(remote_session.0) {
			// remote session number larger than ours, we can try to import the justification
			None => Ok(self
				.handle_justification(state.top_justification(), peer)?
				.map(SyncAction::Task)
				.unwrap_or(SyncAction::Noop)),
			// same session
			Some(0) => match remote_top_number >= local_top_number {
				// remote top justification higher than ours, we can import the justification
				true => Ok(self
					.handle_justification(state.top_justification(), peer)?
					.map(SyncAction::Task)
					.unwrap_or(SyncAction::Noop)),
				// remote top justification lower than ours, we can send a response
				false =>
					Ok(SyncAction::state_broadcast_response(local_top.into_unverified(), None)),
			},
			// remote lags one session behind
			Some(1) => Ok(SyncAction::state_broadcast_response(
				self.last_justification_unverified(remote_session)?,
				Some(local_top.into_unverified()),
			)),
			// remote lags multiple sessions behind
			Some(2..) => Ok(SyncAction::state_broadcast_response(
				self.last_justification_unverified(remote_session)?,
				Some(self.last_justification_unverified(SessionId(remote_session.0 + 1))?),
			)),
		}
	}

	/// The current state of our database.
	pub fn state(&self) -> Result<State<J>, Error<J, CS, V, F>> {
		let top_justification =
			self.chain_status.top_finalized().map_err(Error::ChainStatus)?.into_unverified();
		Ok(State::new(top_justification))
	}

	/// The state of the identified block, whether we are interested in it and how much.
	pub fn block_state(&mut self, block_id: &BlockIdFor<J>) -> Interest<I, J> {
		self.forest.state(block_id)
	}
}

#[cfg(test)]
mod tests {
	use super::{Handler, SyncAction};
	use crate::{
		sync::{
			data::{BranchKnowledge::*, NetworkData, Request},
			mock::{Backend, MockHeader, MockJustification, MockPeerId, MockVerifier},
			BlockIdentifier, ChainStatus, Header, Justification,
		},
		SessionPeriod,
	};

	type MockHandler = Handler<MockPeerId, MockJustification, Backend, MockVerifier, Backend>;

	const SESSION_PERIOD: usize = 20;

	fn setup() -> (MockHandler, Backend, impl Send) {
		let (backend, _keep) = Backend::setup(SESSION_PERIOD);
		let verifier = MockVerifier {};
		let handler = Handler::new(backend.clone(), verifier, backend.clone(), SessionPeriod(20))
			.expect("mock backend works");
		(handler, backend, _keep)
	}

	fn import_branch(backend: &Backend, branch_length: usize) -> Vec<MockHeader> {
		let result: Vec<_> = backend
			.best_block()
			.expect("mock backend works")
			.random_branch()
			.take(branch_length)
			.collect();
		for header in &result {
			backend.import(header.clone());
		}
		result
	}

	#[test]
	fn finalizes_imported_and_justified() {
		let (mut handler, backend, _keep) = setup();
		let header = import_branch(&backend, 1)[0].clone();
		handler.block_imported(header.clone()).expect("importing in order");
		let justification = MockJustification::for_header(header);
		let peer = rand::random();
		assert!(matches!(
			handler
				.handle_justification(justification.clone().into_unverified(), peer)
				.expect("correct justification"),
			None
		));
		assert_eq!(backend.top_finalized().expect("mock backend works"), justification);
	}

	#[test]
	fn finalizes_justified_and_imported() {
		let (mut handler, backend, _keep) = setup();
		let header = import_branch(&backend, 1)[0].clone();
		let justification = MockJustification::for_header(header.clone());
		let peer = rand::random();
		match handler
			.handle_justification(justification.clone().into_unverified(), peer)
			.expect("correct justification")
		{
			Some(id) => assert_eq!(id, header.id()),
			None => panic!("expected an id, got nothing"),
		}
		handler.block_imported(header).expect("importing in order");
		assert_eq!(backend.top_finalized().expect("mock backend works"), justification);
	}

	#[test]
	fn handles_state_with_large_difference() {
		let (mut handler, backend, _keep) = setup();
		let initial_state = handler.state().expect("state works");
		let peer = rand::random();
		let justifications: Vec<MockJustification> = import_branch(&backend, 43)
			.into_iter()
			.map(MockJustification::for_header)
			.collect();
		let last_from_first_session = justifications[18].clone().into_unverified();
		let last_from_second_session = justifications[38].clone().into_unverified();
		for justification in justifications.into_iter() {
			handler
				.block_imported(justification.header().clone())
				.expect("importing in order");
			handler
				.handle_justification(justification.clone().into_unverified(), peer)
				.expect("correct justification");
		}
		match handler.handle_state(initial_state, peer).expect("correct justification") {
			SyncAction::Response(NetworkData::StateBroadcastResponse(
				justification,
				maybe_justification,
			)) => {
				assert_eq!(justification, last_from_first_session);
				assert_eq!(maybe_justification, Some(last_from_second_session));
			},
			other_action =>
				panic!("expected a response with justifications, got {:?}", other_action),
		}
	}

	#[test]
	fn handles_state_with_medium_difference() {
		let (mut handler, backend, _keep) = setup();
		let initial_state = handler.state().expect("state works");
		let peer = rand::random();
		let justifications: Vec<MockJustification> = import_branch(&backend, 23)
			.into_iter()
			.map(MockJustification::for_header)
			.collect();
		let last_from_first_session = justifications[18].clone().into_unverified();
		let top = justifications[22].clone().into_unverified();
		for justification in justifications.into_iter() {
			handler
				.block_imported(justification.header().clone())
				.expect("importing in order");
			handler
				.handle_justification(justification.clone().into_unverified(), peer)
				.expect("correct justification");
		}
		match handler.handle_state(initial_state, peer).expect("correct justification") {
			SyncAction::Response(NetworkData::StateBroadcastResponse(
				justification,
				maybe_justification,
			)) => {
				assert_eq!(justification, last_from_first_session);
				assert_eq!(maybe_justification, Some(top));
			},
			other_action =>
				panic!("expected a response with justifications, got {:?}", other_action),
		}
	}

	#[test]
	fn handles_state_with_small_difference() {
		let (mut handler, backend, _keep) = setup();
		let initial_state = handler.state().expect("state works");
		let peer = rand::random();
		let justifications: Vec<MockJustification> = import_branch(&backend, 13)
			.into_iter()
			.map(MockJustification::for_header)
			.collect();
		let top = justifications[12].clone().into_unverified();
		for justification in justifications.into_iter() {
			handler
				.block_imported(justification.header().clone())
				.expect("importing in order");
			handler
				.handle_justification(justification.clone().into_unverified(), peer)
				.expect("correct justification");
		}
		match handler.handle_state(initial_state, peer).expect("correct justification") {
			SyncAction::Response(NetworkData::StateBroadcastResponse(
				justification,
				maybe_justification,
			)) => {
				assert_eq!(justification, top);
				assert!(maybe_justification.is_none());
			},
			other_action =>
				panic!("expected a response with justifications, got {:?}", other_action),
		}
	}

	#[test]
	fn handles_request() {
		let (mut handler, backend, _keep) = setup();
		let initial_state = handler.state().expect("state works");
		let peer = rand::random();
		let mut justifications: Vec<_> = import_branch(&backend, 500)
			.into_iter()
			.map(MockJustification::for_header)
			.collect();
		for justification in &justifications {
			let number = justification.header().id().number();
			handler
				.block_imported(justification.header().clone())
				.expect("importing in order");
			// skip some justifications, but always keep the last of the session
			// justifications right before the last will be skipped in response
			if number % 20 < 10 || number % 20 > 14 {
				handler
					.handle_justification(justification.clone().into_unverified(), peer)
					.expect("correct justification");
			}
		}
		// currently ignored, so picking a random one
		let requested_id = justifications[43].header().id();
		let request = Request::new(requested_id.clone(), LowestId(requested_id), initial_state);
		// filter justifications, these are supposed to be included in the response
		justifications.retain(|j| {
			let number = j.header().id().number();
			number % 20 < 10 || number % 20 == 19
		});
		match handler.handle_request(request).expect("correct request") {
			SyncAction::Response(NetworkData::RequestResponse(sent_justifications)) => {
				assert_eq!(sent_justifications.len(), 100);
				for (sent_justification, justification) in
					sent_justifications.iter().zip(justifications)
				{
					assert_eq!(sent_justification.header().id(), justification.header().id());
				}
			},
			other_action =>
				panic!("expected a response with justifications, got {:?}", other_action),
		}
	}
}
