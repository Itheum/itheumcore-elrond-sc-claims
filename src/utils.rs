multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage::{self, ClaimType};

#[multiversx_sc::module]
pub trait UtilsModule: storage::StorageModule {}
