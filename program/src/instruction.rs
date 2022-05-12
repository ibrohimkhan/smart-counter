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
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_serialization() {
        let data = SmartCounterInstruction::UpdateSettings {
            admin: Pubkey::from_str("EG7uy9FCe4AxL9AavEA1nXDfo2AoBo1ZtBCV224hmoub")
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
                2, 197, 7, 117, 129, 90, 151, 178, 48, 248, 208, 199, 5, 17, 134, 51, 183, 155,
                153, 209, 86, 177, 138, 127, 133, 1, 191, 178, 128, 179, 23, 157, 98, 19, 0, 0, 0,
                99, 0, 0, 0
            ]
        );
    }
}
