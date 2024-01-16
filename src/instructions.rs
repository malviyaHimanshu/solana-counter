use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}

// #[derive(Debug, BorshDeserialize, BorshSerialize)]
// pub struct IncArgs {
//     pub value: u32,
// }

// #[derive(Debug, BorshDeserialize, BorshSerialize)]
// pub struct DecArgs {
//     pub value: u32,
// }

#[derive(Debug)]
pub enum CounterInstructions {
    Increment(UpdateArgs),
    Decrement(UpdateArgs),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        println!("this is input: {:?}", input);
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        println!("this is rest {:?}", rest);
        Ok(match variant {
            0 => Self::Increment(UpdateArgs::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(UpdateArgs::try_from_slice(rest).unwrap()),
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}