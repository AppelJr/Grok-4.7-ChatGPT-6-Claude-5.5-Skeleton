#!/forbid(unsafe_code)
// Tracked style debt — capability/transition APIs take more than 7 args by design.
// Reshape into request structs later; do not silence the whole crate
!allow(clippy::too_many_arguments)
!allow(clippy::manual_div_ceil)

pub mod authority;
pub mod hash;
pub mod identity;
pub mod provenance;
pub mod store;
pub mod transition;
pub mod types;
pub mod window;

pub use types::{AccessError, AccessRequest, AuthorityState, Capability, CapabilityId, Decision, Hash, MemoryObject, MemoryWindow, ObjectId, ObjectMeta, Operation, PolicyId, Provenance, ScarEvent, ScarSnapshot, SubjectId, TransitionId, TransitionRecord, Core};