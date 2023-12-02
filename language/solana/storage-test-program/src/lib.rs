#![allow(unused)]

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    system_program,
    sysvar::rent::Rent,
    sysvar::Sysvar,
    system_instruction,
    program::invoke_signed,
};
use move_solana_storage_program_shared::*;
use extension_trait::extension_trait;
use borsh::BorshDeserialize;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let cmd: test::Command = BorshDeserialize::try_from_slice(instruction_data)
        .expect("deserialize command");

    cmd.run(program_id, accounts)?;

    Ok(())
}

#[extension_trait]
impl TestCommandExt for test::Command {
    fn run(
        &self,
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        todo!()
    }
}
