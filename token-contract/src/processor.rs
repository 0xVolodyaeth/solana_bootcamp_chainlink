use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::TokenInstruction;
use crate::state::{Token, TokenAccount};

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = TokenInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        let accounts_iter = &mut accounts.iter();
        msg!("Instruction: {:?}", instruction);

        match instruction {
            TokenInstruction::CreateToken => {
                //get account info for master token account
                let token_master_account = next_account_info(accounts_iter)?;
                let token_authority = next_account_info(accounts_iter)?;
                let mut token = Token::load_unchecked(token_master_account)?;

                //set default values and save master token account
                token.authority = *token_authority.key;
                token.supply = 0;
                token.save(token_master_account)?
            }
            TokenInstruction::CreateTokenAccount => {
                msg!("Instruction: Create Token Account");
            }
            TokenInstruction::Mint { amount } => {
                msg!("Instruction: Mint");
            }
            TokenInstruction::Transfer { amount } => {
                msg!("Instruction: Transfer");
            }
        }
        Ok(())
    }
}
