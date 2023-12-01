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

// Placeholder data stored in the `program_auth_pda` indicating which Solana
// programs are authenticated Move programs and allowed to use the storage
// program.
const MOVE_PROGRAM_AUTH_KEY: u8 = 0xff;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let cmd: Command = BorshDeserialize::try_from_slice(instruction_data)
        .expect("deserialize command");

    cmd.run(program_id, accounts)?;

    Ok(())
}

#[extension_trait]
impl CommandExt for Command {
    fn run(
        &self,
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        match self {
            Command::Init(cmd) => cmd.run(program_id, accounts),
            Command::Authorize(cmd) => cmd.run(program_id, accounts),
            Command::MoveTo(cmd) => {
                let accounts = Accounts::from_slice(accounts)?;
                accounts.authenticate_caller(program_id, &cmd.calling_program_id);
                cmd.run(program_id, &accounts)
            },
            Command::MoveFrom(cmd) => {
                let accounts = Accounts::from_slice(accounts)?;
                accounts.authenticate_caller(program_id, &cmd.calling_program_id);
                cmd.run(program_id, &accounts)
            },
            Command::BorrowGlobalMutCommit(cmd) => {
                let accounts = Accounts::from_slice(accounts)?;
                accounts.authenticate_caller(program_id, &cmd.calling_program_id);
                cmd.run(program_id, &accounts)
            },
        }
    }
}

#[extension_trait]
impl InitCommandExt for InitCommand {
    fn run(
        &self,
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();

        let system_program = next_account_info(account_iter)?;
        let payer = next_account_info(account_iter)?;
        let admin_pda = next_account_info(account_iter)?;

        assert!(next_account_info(account_iter).is_err());
        
        assert_eq!(system_program.key, &system_program::ID);
        assert!(payer.is_signer);
        assert!(payer.is_writable);
        assert!(admin_pda.is_writable);

        // Don't allow reinitialization.
        assert_eq!(admin_pda.owner, &system_program::ID);
        assert_eq!(admin_pda.data.borrow().len(), 0);
        assert_eq!(**admin_pda.lamports.borrow(), 0);

        let (
            admin_pda_derived,
            admin_pda_bump_seed,
        ) = Pubkey::find_program_address(
            &[
                b"admin",
            ],
            program_id,
        );
        assert_eq!(admin_pda.key, &admin_pda_derived);

        let admin_account_bytes = self.admin_account.to_bytes();

        let rent = Rent::get().expect("rent")
            .minimum_balance(admin_account_bytes.len());

        invoke_signed(
            &system_instruction::create_account(
                payer.key,
                admin_pda.key,
                rent,
                admin_account_bytes.len() as u64,
                program_id, // owned by the storage program
            ),
            &[payer.clone(), admin_pda.clone()],
            &[
                // The admin_pda derivation
                &[b"admin", &[admin_pda_bump_seed]],
            ],
        )?;

        // Write the admin account into the admin_pda
        let mut data = admin_pda.data.borrow_mut();
        assert_eq!(data.len(), admin_account_bytes.len());
        data.copy_from_slice(&admin_account_bytes);

        Ok(())
    }
}

#[extension_trait]
impl AuthorizeCommandExt for AuthorizeCommand {
    fn run(
        &self,
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();

        let system_program = next_account_info(account_iter)?;
        let payer = next_account_info(account_iter)?;
        let admin = next_account_info(account_iter)?;
        let admin_pda = next_account_info(account_iter)?;
        let program_auth_pda = next_account_info(account_iter)?;

        assert!(next_account_info(account_iter).is_err());
        
        assert_eq!(system_program.key, &system_program::ID);
        assert!(payer.is_signer);
        assert!(payer.is_writable);
        assert!(admin.is_signer);
        assert!(program_auth_pda.is_writable);

        // Verify the admin
        {
            let (
                admin_pda_derived,
                _admin_pda_bump_seed,
            ) = Pubkey::find_program_address(
                &[
                    b"admin",
                ],
                program_id,
            );

            assert_eq!(admin_pda.key, &admin_pda_derived);

            let admin_bytes = admin.key.to_bytes();
            let admin_pda_data = admin_pda.data.borrow();
            assert_eq!(admin_bytes, *admin_pda_data);
        }

        // Validate program properties
        {
            // fixme:
            // - check if authorized_program_id is finalized
            // - check if the authorized_program_id is executable?
        }

        // Write authorization to program_auth_pda
        {
            let (
                program_auth_pda_derived,
                program_auth_pda_bump_seed,
            ) = Pubkey::find_program_address(
                &[
                    b"program-auth",
                ],
                program_id,
            );

            // The provided account is the program_auth_pda
            assert_eq!(program_auth_pda.key, &program_auth_pda_derived);

            // Don't allow reinitialization.
            assert_eq!(program_auth_pda.owner, &system_program::ID);

            let rent = Rent::get().expect("rent")
                .minimum_balance(1);

            invoke_signed(
                &system_instruction::create_account(
                    payer.key,
                    program_auth_pda.key,
                    rent,
                    1,
                    program_id, // owned by the storage program
                ),
                &[payer.clone(), program_auth_pda.clone()],
                &[
                    // The program_auth_pda derivation
                    &[b"program_auth", &[program_auth_pda_bump_seed]],
                ],
            )?;
            
            let mut data = program_auth_pda.data.borrow_mut();
            assert_eq!(data.len(), 1);
            data.copy_from_slice(&[MOVE_PROGRAM_AUTH_KEY]);
        }

        Ok(())
    }
}

/// Accounts used by all instructions issued by Move programs.
///
/// Admin instructions use different accounts.
struct Accounts<'a, 'b> {
    system_program: &'a AccountInfo<'b>,
    /// The account that will pay for any system invocations,
    /// probably the end-user that invoked the tx.
    /// Also receives refunds when storage is freed.
    payer: &'a AccountInfo<'b>,
    /// The account that holds admin authorization for the caller to use the storage program,
    /// owned by the storage program.
    program_auth_pda: &'a AccountInfo<'b>,
    /// Signed by the caller, to authorize storage access.
    caller_auth_pda: &'a AccountInfo<'b>,
    /// The storage account operated on, owned by the storage program,
    /// and signed by the storage program when invoking system programs.
    storage_pda: &'a AccountInfo<'b>,
}

impl<'a, 'b> Accounts<'a, 'b> {
    fn from_slice(accounts: &'a [AccountInfo<'b>]) -> Result<Accounts<'a, 'b>, ProgramError> {
        let iter = &mut accounts.iter();

        let accounts = Accounts {
            system_program: next_account_info(iter)?,
            payer: next_account_info(iter)?,
            program_auth_pda: next_account_info(iter)?,
            caller_auth_pda: next_account_info(iter)?,
            storage_pda: next_account_info(iter)?,
        };

        assert!(next_account_info(iter).is_err());

        assert_eq!(accounts.system_program.key, &system_program::ID);
        // This doesn't need to be true for MoveFromCommand
        //assert!(accounts.payer.is_signer);
        assert!(accounts.payer.is_writable);
        assert!(accounts.storage_pda.is_writable);

        Ok(accounts)
    }

    fn authenticate_caller(
        &self,
        program_id: &Pubkey,
        calling_program_id: &Pubkey,
    ) {
        // Check that the caller has been authorized by admin
        {
            let (
                program_auth_pda,
                _program_auth_pda_bump_seed,
            ) = Pubkey::find_program_address(
                &[
                    b"program-auth",
                ],
                program_id,
            );

            // The provided account is the program_auth_pda
            assert_eq!(&program_auth_pda, self.program_auth_pda.key);
            // The storage program owns the program auth account
            assert_eq!(self.program_auth_pda.owner, program_id);
            // The auth account contains the expected auth data
            assert_eq!(self.program_auth_pda.data.borrow().as_ref(), &[MOVE_PROGRAM_AUTH_KEY]);
        }

        // Check that the caller is authorizing writing to its storage.
        let (
            caller_auth_pda,
            _caller_auth_pda_bump_seed,
        ) = Pubkey::find_program_address(
            &[
                b"caller-auth",
            ],
            calling_program_id,
        );

        // The provided account is the caller_auth_pda
        assert_eq!(&caller_auth_pda, self.caller_auth_pda.key);
        // That account has been signed by the caller
        assert!(self.caller_auth_pda.is_signer);
    }
}

#[extension_trait]
impl MoveToCommandExt for MoveToCommand {
    fn run(
        &self,
        program_id: &Pubkey,
        accounts: &Accounts
    ) -> ProgramResult {
        // Two cases: the storage cell doesn't exist or does exist.
        if accounts.storage_pda.owner == &system_program::ID {
            // sanity checks
            assert_eq!(accounts.storage_pda.data.borrow().len(), 0);
            assert_eq!(**accounts.storage_pda.lamports.borrow(), 0);

            // Create the account
            let (
                storage_pda,
                storage_pda_bump_seed,
            ) = Pubkey::find_program_address(
                &[
                    &self.address.0,
                    &self.type_hash.0,
                ],
                program_id,
            );
            assert_eq!(&storage_pda, accounts.storage_pda.key);

            let rent = Rent::get().expect("rent")
                .minimum_balance(self.value.len());

            invoke_signed(
                &system_instruction::create_account(
                    accounts.payer.key,
                    accounts.storage_pda.key,
                    rent,
                    self.value.len() as u64,
                    program_id, // owned by the storage program
                ),
                &[accounts.payer.clone(), accounts.storage_pda.clone()],
                &[
                    // The storage_pda derivation
                    &[&self.address.0, &self.type_hash.0, &[storage_pda_bump_seed]],
                ],
            )?;
        } else {
            assert_eq!(accounts.storage_pda.owner, program_id);

            // Resize the account to fit the new data
            accounts.storage_pda.realloc(
                self.value.len(),
                false,
            )?;
        }

        let mut data = accounts.storage_pda.data.borrow_mut();
        assert_eq!(data.len(), self.value.len());
        data.copy_from_slice(&self.value);

        Ok(())
    }
}

#[extension_trait]
impl MoveFromCommandExt for MoveFromCommand {
    fn run(
        &self,
        program_id: &Pubkey,
        accounts: &Accounts
    ) -> ProgramResult {
        // All there is to do in this case is deallocate the storage;
        // The caller will read it through its own AccountInfo.

        let (
            storage_pda,
            storage_pda_bump_seed,
        ) = Pubkey::find_program_address(
            &[
                &self.address.0,
                &self.type_hash.0,
            ],
            program_id,
        );
        assert_eq!(&storage_pda, accounts.storage_pda.key);

        // First transfer the lamports to make it un-rent-exempt,
        // so it will be deleted after the next block.
        invoke_signed(
            &system_instruction::transfer(
                accounts.storage_pda.key,
                accounts.payer.key,
                **accounts.storage_pda.lamports.borrow(),
            ),
            &[accounts.storage_pda.clone()],
            &[
                // The storage_pda derivation
                &[&self.address.0, &self.type_hash.0, &[storage_pda_bump_seed]],
            ],
        )?;

        // These next two steps may be necessary if the account gets
        // recreated in the same block / transaction, before it is deleted.

        // Reallocate to zero -
        // it is expected by MoveTo when creating an account.
        accounts.storage_pda.realloc(0, false)?;

        // Finally assign ownership back to the system program.
        invoke_signed(
            &system_instruction::assign(
                accounts.storage_pda.key,
                &system_program::ID,
            ),
            &[accounts.storage_pda.clone()],
            &[
                // The storage_pda derivation
                &[&self.address.0, &self.type_hash.0, &[storage_pda_bump_seed]],
            ]
        )?;
        
        Ok(())
    }
}

#[extension_trait]
impl BorrowGlobalMutCommitCommandExt for BorrowGlobalMutCommitCommand {
    fn run(
        &self,
        program_id: &Pubkey,
        accounts: &Accounts
    ) -> ProgramResult {
        assert_eq!(accounts.storage_pda.owner, program_id);

        // Resize the account to fit the new data
        accounts.storage_pda.realloc(
            self.value.len(),
            false,
        )?;

        let mut data = accounts.storage_pda.data.borrow_mut();
        assert_eq!(data.len(), self.value.len());
        data.copy_from_slice(&self.value);

        Ok(())
    }
}
