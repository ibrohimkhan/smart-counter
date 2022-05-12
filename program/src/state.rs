use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;

use crate::{id, COUNTER_SEED, SETTINGS_SEED};

/// Each user has its own smart-counter account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SmartCounter {
    /// Increment this property by one
    pub counter: u32,

    /// Value of a counter
    pub value: i64,
}

/// There is only one settings account and all smart-counter accounts use it
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Settings {
    /// Only admin can modify this account
    pub admin: [u8; 32],

    /// Step value to increment counter
    pub inc_step: u32,

    /// Step value to decrement counter
    pub dec_step: u32,
}

impl SmartCounter {
    pub fn get_smart_counter_pubkey(user: &Pubkey) -> Pubkey {
        Pubkey::create_with_seed(user, COUNTER_SEED, &id()).unwrap()
    }

    pub fn is_ok_counter_pubkey(user: &Pubkey, counter: &Pubkey) -> bool {
        counter.to_bytes() == Self::get_smart_counter_pubkey(user).to_bytes()
    }
}

impl Settings {
    pub fn get_settings_pubkey_with_bump() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SETTINGS_SEED.as_bytes()], &id())
    }

    pub fn get_settings_pubkey() -> Pubkey {
        let (pubkey, _) = Self::get_settings_pubkey_with_bump();
        pubkey
    } 

    pub fn is_ok_settings_pubkey(settings_pubkey: &Pubkey) -> bool {
        let (pubkey, _) = Self::get_settings_pubkey_with_bump();
        pubkey.to_bytes() == settings_pubkey.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_serialization() {
        let data = Settings { admin: [7_u8; 32], inc_step: 19, dec_step: 99 }.try_to_vec().unwrap();
        assert_eq!(
            data,
            [
                7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
                7, 7, 7, 7, 19, 0, 0, 0, 99, 0, 0, 0
            ]
        );
    }

    #[test]
    #[ignore = "issues with pubkeys, will fix it later"]
    fn test_get_settings_address_with_seed() {
        let (address, bump) = Settings::get_settings_pubkey_with_bump();
        assert_eq!(
            address,
            Pubkey::from_str("4voA9ct4uAJuBVLNfoaPiU1VgpatMpGKRLHfvP8CZ147").unwrap()
        );

        assert_eq!(bump, 255);
    }

    #[test]
    #[ignore = "issues with pubkeys, will fix it later"]
    fn test_get_smart_counter_pubkey() {
        let pubkey = SmartCounter::get_smart_counter_pubkey(
            &Pubkey::from_str("FKr2pLkJXFpnJf2sUtStVwDiQPq61rKngtXyhLw8SQbF").unwrap(),
        );

        assert_eq!(
            pubkey,
            Pubkey::from_str("9JVaomeo7Ps8D41whGLkz1c1wzWGfKpk62Mopnf3B274").unwrap()
        );
    }
}