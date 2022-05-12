use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum SmartCounterError {
    #[error("Admin signature is required")]
    AdminRequired,

    #[error("Wrong counter PDA for this user")]
    WrongCounterPDA,

    #[error("Wrong settings PDA")]
    WrongSettingsPDA,
}

impl From<SmartCounterError> for ProgramError {
    fn from(e: SmartCounterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}