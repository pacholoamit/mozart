mod shared;
pub use shared::Shared;

mod conversion;
pub use conversion::{prost_to_serde, serde_to_prost};
