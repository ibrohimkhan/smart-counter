use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

use crate::id;
use crate::state::{Settings, SmartCounter};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum SmartCounterInstruction {
    /// Increment a counter.
    /// Accounts:
    /// 0. `[signer]` owner of a counter
    /// 1. `[writable]` counter_account, PDA
    /// 2. `[]` settings_account, PDA
    Inc,

    /// Decrement a counter.
    /// Accounts:
    /// 0. `[signer]` owner of a counter
    /// 1. `[writable]` counter_account, PDA
    /// 2. `[]` settings_account, PDA
    Dec,

    /// Update settings for counters. Only admin can do it.
    /// Accounts:
    /// 0. `[signer, writable]` Admin of counters
    /// 1. `[writable]` settings_account, PDA
    /// 2. `[]` Rent sysvar
    /// 3. `[]` System program
    UpdateSettings {
        admin: [u8; 32],
        inc_step: u32,
        dec_step: u32,
    },
}

impl SmartCounterInstruction {
    pub fn inc(user: &Pubkey) -> Instruction {
        let counter_pubkey = SmartCounter::get_smart_counter_pubkey(user);
        let (settings_pubkey, _) = Settings::get_settings_pubkey_with_bump();

        Instruction::new_with_borsh(
            id(),
            &SmartCounterInstruction::Inc,
            vec![
                AccountMeta::new_readonly(*user, true),
                AccountMeta::new(counter_pubkey, false),
                AccountMeta::new(settings_pubkey, false),
            ],
        )
    }

    pub fn dec(user: &Pubkey) -> Instruction {
        let counter_pubkey = SmartCounter::get_smart_counter_pubkey(user);
        let (settings_pubkey, _) = Settings::get_settings_pubkey_with_bump();

        Instruction::new_with_borsh(
            id(),
            &SmartCounterInstruction::Dec,
            vec![
                AccountMeta::new_readonly(*user, true),
                AccountMeta::new(counter_pubkey, false),
                AccountMeta::new(settings_pubkey, false),
            ],
        )
    }

    pub fn update_settings(
        admin: &Pubkey,
        new_admin: [u8; 32],
        inc_step: u32,
        dec_step: u32,
    ) -> Instruction {
        let (settings_pubkey, _) = Settings::get_settings_pubkey_with_bump();

        Instruction::new_with_borsh(
            id(),
            &SmartCounterInstruction::UpdateSettings {
                admin: new_admin,
                inc_step,
                dec_step,
            },
            vec![
                AccountMeta::new(*admin, true),
                AccountMeta::new(settings_pubkey, false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_serialization() {
        let data = SmartCounterInstruction::UpdateSettings {
            admin: Pubkey::from_str("6GVwyLvuW8mW4UZRUbUgauZeTBMPq3sgGgYpunUAd9cQ")
                .unwrap()
                .to_bytes(),
            inc_step: 19,
            dec_step: 99,
        }
        .try_to_vec()
        .unwrap();

        assert_eq!(
            data,
            [
                2, 78, 66, 224, 205, 49, 170, 169, 123, 253, 137, 85, 42, 136, 62, 85, 129, 235,
                240, 23, 107, 80, 42, 142, 119, 154, 129, 192, 157, 221, 14, 144, 117, 19, 0, 0, 0,
                99, 0, 0, 0
            ]
        );
    }
}
