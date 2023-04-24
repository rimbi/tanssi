//! Crate containing various traits used by moondance crates allowing to connect pallet
//! with each other or with mocks.

#![cfg_attr(not(feature = "std"), no_std)]

pub use cumulus_primitives_core::{relay_chain::Slot, ParaId};
use sp_std::vec::Vec;

/// Get the current list of container chains parachain ids.
pub trait GetCurrentContainerChains {
    fn current_container_chains() -> Vec<ParaId>;
}

/// Get the list of container chains parachain ids at given
/// session index.
pub trait GetSessionContainerChains<SessionIndex> {
    fn session_container_chains(session_index: SessionIndex) -> Vec<ParaId>;
}

/// Returns author for a parachain id for the given slot.
pub trait GetContainerChainAuthor<AccountId> {
    fn author_for_slot(slot: Slot, para_id: ParaId) -> Option<AccountId>;
}

/// Returns the host configuration composed of the amount of collators assigned
/// to the orchestrator chain, and how many collators are assigned per container chain.
pub trait GetHostConfiguration<SessionIndex> {
    fn collators_for_orchestrator(session_index: SessionIndex) -> u32;
    fn collators_per_container(session_index: SessionIndex) -> u32;
}

/// Returns current session index.
pub trait GetSessionIndex<SessionIndex> {
    fn session_index() -> SessionIndex;
}
