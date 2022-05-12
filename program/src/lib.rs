pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub const COUNTER_SEED: &str = "smart-counter";
pub const SETTINGS_SEED: &str = "settings";

solana_program::declare_id!("HnT6HNu2ppYAvotmL1Z4Tc2XB89r1LgM4i2uaZTE5xGr");