pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub const COUNTER_SEED: &str = "smart-counter";
pub const SETTINGS_SEED: &str = "settings";

solana_program::declare_id!("HnT6HNu2ppYAvotmL1Z4Tc2XB89r1LgM4i2uaZTE5xGr");


// program: HnT6HNu2ppYAvotmL1Z4Tc2XB89r1LgM4i2uaZTE5xGr
// admin: 6GVwyLvuW8mW4UZRUbUgauZeTBMPq3sgGgYpunUAd9cQ - wallet1.json
// user: B6n7uLSPK6m8mwPxkJ2t8a1sGaXqocF2fabZ6XPBdSQ9 - wallet2.json
// settings: EndezmND2wTgMp7CsAUZtGwQbdGweL4gxuAZ6i77aFfg
// counter: 3ojoCDtMrAygHWZ4uDW9E9B7VkHst2CiH3fbLmaBLvtq