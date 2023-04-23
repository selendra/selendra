//! Implementations and definitions of traits used in legacy & current abft

use std::{cmp::Ordering, fmt::Debug, hash::Hash as StdHash, marker::PhantomData, pin::Pin};

use codec::{Codec, Decode, Encode};
use futures::{channel::oneshot, Future, TryFutureExt};
use sc_service::SpawnTaskHandle;
use sp_api::BlockT;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Hash as SpHash;

use crate::data_io::{DataProvider, OrderedDataInterpreter, SelendraData};

/// A convenience trait for gathering all of the desired hash characteristics.
pub trait Hash: AsRef<[u8]> + StdHash + Eq + Clone + Codec + Debug + Send + Sync {}

impl<T: AsRef<[u8]> + StdHash + Eq + Clone + Codec + Debug + Send + Sync> Hash for T {}

#[async_trait::async_trait]
impl<B: BlockT> selendra_bft::DataProvider<SelendraData<B>> for DataProvider<B> {
	async fn get_data(&mut self) -> Option<SelendraData<B>> {
		DataProvider::get_data(self).await
	}
}

#[async_trait::async_trait]
impl<B: BlockT> legacy_selendra_bft::DataProvider<SelendraData<B>> for DataProvider<B> {
	async fn get_data(&mut self) -> Option<SelendraData<B>> {
		DataProvider::get_data(self).await
	}
}

impl<B: BlockT, C: HeaderBackend<B> + Send + 'static>
	selendra_bft::FinalizationHandler<SelendraData<B>> for OrderedDataInterpreter<B, C>
{
	fn data_finalized(&mut self, data: SelendraData<B>) {
		OrderedDataInterpreter::data_finalized(self, data)
	}
}

impl<B: BlockT, C: HeaderBackend<B> + Send + 'static>
	legacy_selendra_bft::FinalizationHandler<SelendraData<B>> for OrderedDataInterpreter<B, C>
{
	fn data_finalized(&mut self, data: SelendraData<B>) {
		OrderedDataInterpreter::data_finalized(self, data)
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Wrapper<H: SpHash> {
	phantom: PhantomData<H>,
}

/// SelendraBFT requires an order on hashes and `SpHash` does not have one, so we wrap it to add it.
#[derive(Debug, PartialEq, Eq, Clone, Copy, StdHash, Encode, Decode)]
pub struct OrdForHash<O: Eq + Copy + Clone + Send + Debug + StdHash + Encode + Decode + AsRef<[u8]>>
{
	inner: O,
}

impl<O: Eq + Copy + Clone + Send + Sync + Debug + StdHash + Encode + Decode + AsRef<[u8]>>
	PartialOrd for OrdForHash<O>
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<O: Eq + Copy + Clone + Send + Sync + Debug + StdHash + Encode + Decode + AsRef<[u8]>> Ord
	for OrdForHash<O>
{
	fn cmp(&self, other: &Self) -> Ordering {
		self.inner.as_ref().cmp(other.inner.as_ref())
	}
}

impl<O: Eq + Copy + Clone + Send + Sync + Debug + StdHash + Encode + Decode + AsRef<[u8]>>
	AsRef<[u8]> for OrdForHash<O>
{
	fn as_ref(&self) -> &[u8] {
		self.inner.as_ref()
	}
}

impl<H: SpHash> Wrapper<H> {
	fn hash(s: &[u8]) -> OrdForHash<H::Output> {
		OrdForHash { inner: <H as SpHash>::hash(s) }
	}
}

impl<H: SpHash> selendra_bft::Hasher for Wrapper<H> {
	type Hash = OrdForHash<H::Output>;

	fn hash(s: &[u8]) -> Self::Hash {
		Wrapper::<H>::hash(s)
	}
}

impl<H: SpHash> legacy_selendra_bft::Hasher for Wrapper<H> {
	type Hash = OrdForHash<H::Output>;

	fn hash(s: &[u8]) -> Self::Hash {
		Wrapper::<H>::hash(s)
	}
}

/// A wrapper for spawning tasks in a way compatible with SelendraBFT.
#[derive(Clone)]
pub struct SpawnHandle(SpawnTaskHandle);

impl SpawnHandle {
	pub fn spawn_essential_with_result(
		&self,
		name: &'static str,
		task: impl Future<Output = Result<(), ()>> + Send + 'static,
	) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send>> {
		let (tx, rx) = oneshot::channel();
		let wrapped_task = async move {
			let result = task.await;
			let _ = tx.send(result);
		};
		let result = <Self as SpawnHandleT>::spawn_essential(self, name, wrapped_task);
		let wrapped_result = async move {
			let main_result = result.await;
			if main_result.is_err() {
				return Err(())
			}
			let rx_result = rx.await;
			rx_result.unwrap_or(Err(()))
		};
		Box::pin(wrapped_result)
	}
}

impl From<SpawnTaskHandle> for SpawnHandle {
	fn from(sth: SpawnTaskHandle) -> Self {
		SpawnHandle(sth)
	}
}

/// Trait abstracting spawning tasks
pub trait SpawnHandleT {
	/// Run task
	fn spawn(&self, name: &'static str, task: impl Future<Output = ()> + Send + 'static);

	/// Run an essential task
	fn spawn_essential(
		&self,
		name: &'static str,
		task: impl Future<Output = ()> + Send + 'static,
	) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send>>;
}

impl SpawnHandleT for SpawnHandle {
	fn spawn(&self, name: &'static str, task: impl Future<Output = ()> + Send + 'static) {
		self.0.spawn(name, None, task)
	}

	fn spawn_essential(
		&self,
		name: &'static str,
		task: impl Future<Output = ()> + Send + 'static,
	) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send>> {
		let (tx, rx) = oneshot::channel();
		self.spawn(name, async move {
			task.await;
			let _ = tx.send(());
		});
		Box::pin(rx.map_err(|_| ()))
	}
}

impl selendra_bft::SpawnHandle for SpawnHandle {
	fn spawn(&self, name: &'static str, task: impl Future<Output = ()> + Send + 'static) {
		SpawnHandleT::spawn(self, name, task)
	}

	fn spawn_essential(
		&self,
		name: &'static str,
		task: impl Future<Output = ()> + Send + 'static,
	) -> selendra_bft::TaskHandle {
		SpawnHandleT::spawn_essential(self, name, task)
	}
}

impl legacy_selendra_bft::SpawnHandle for SpawnHandle {
	fn spawn(&self, name: &'static str, task: impl Future<Output = ()> + Send + 'static) {
		SpawnHandleT::spawn(self, name, task)
	}

	fn spawn_essential(
		&self,
		name: &'static str,
		task: impl Future<Output = ()> + Send + 'static,
	) -> legacy_selendra_bft::TaskHandle {
		SpawnHandleT::spawn_essential(self, name, task)
	}
}
