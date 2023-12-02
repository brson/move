use solana_program::pubkey::Pubkey;

pub const ACCOUNT_ADDRESS_LENGTH: usize = 32;

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct MoveAddress(pub [u8; ACCOUNT_ADDRESS_LENGTH]);

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct MoveTypeHash(pub [u8; 32]);

#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub enum Command {
    /// Initilaize the storage program.
    Init(InitCommand),
    /// Authorize another (trusted) program to use the storage program.
    Authorize(AuthorizeCommand),
    /// The Move `move_to` instruction.
    MoveTo(MoveToCommand),
    /// The Move `move_from` instruction.
    MoveFrom(MoveFromCommand),
    /// The Move `borrow_global_mut` instruction.
    BorrowGlobalMutCommit(BorrowGlobalMutCommitCommand),
}

/// Initialize the storage program.
///
/// # Accounts
///
/// - system_program
/// - payer - signer, writable
/// - admin_pda - writable
///   - derivation: (["admin"], program_id)
#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct InitCommand {
    pub admin_account: Pubkey,
}

/// Authorize another program to use the storage program.
///
/// # Accounts
///
/// - system_program
/// - payer - signer, writable
/// - admin - signer
/// - admin_pda
///   - derivation: (["admin"], program_id)
/// - program_auth_pda - writable
///   - derivation: (["program-auth"], program_id)
#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct AuthorizeCommand {
    pub authorized_program_id: Pubkey,
}

/// # Accounts
///
/// - system_program
/// - payer - signer, writable
/// - program_auth_pda
///   - derivation: (["program-auth"], program_id)
/// - caller_auth_pda - signer
///   - derivation: (["caller-auth"], self.calling_program_id)
/// - storage_pda - writable
///   - derivation: ([self.address, self.type_hash], program_id)
#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct MoveToCommand {
    pub calling_program_id: Pubkey,
    pub address: MoveAddress,
    pub type_hash: MoveTypeHash,
    pub value: Vec<u8>,
}

/// # Accounts
///
/// - system_program
/// - payer - writable
/// - program_auth_pda
///   - derivation: (["program-auth"], program_id)
/// - caller_auth_pda - signer
///   - derivation: (["caller-auth"], self.calling_program_id)
/// - storage_pda - writable
///   - derivation: ([self.address, self.type_hash], program_id)
#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct MoveFromCommand {
    pub calling_program_id: Pubkey,
    pub address: MoveAddress,
    pub type_hash: MoveTypeHash,
}

/// # Accounts
///
/// - system_program
/// - payer - signer, writable
/// - program_auth_pda
///   - derivation: (["program-auth"], program_id)
/// - caller_auth_pda - signer
///   - derivation: (["caller-auth"], self.calling_program_id)
/// - storage_pda - writable
///   - derivation: ([self.address, self.type_hash], program_id)
#[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
pub struct BorrowGlobalMutCommitCommand {
    pub calling_program_id: Pubkey,
    pub address: MoveAddress,
    pub type_hash: MoveTypeHash,
    pub value: Vec<u8>,
}

pub mod test {
    use super::MoveAddress;

    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub enum Command {
        MoveIntTo(MoveIntToCommand),
        MoveIntFrom(MoveIntFromCommand),
        MoveIntFromTo(MoveIntFromToCommand),
        BorrowMutateInt(BorrowIntIncrementCommand),
    }

    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct MoveIntToCommand {
        pub to_address: MoveAddress,
        pub value: u32,
    }

    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct MoveIntFromCommand {
        pub from_address: MoveAddress,
    }

    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct MoveIntFromToCommand {
        pub from_address: MoveAddress,
        pub to_address: MoveAddress,
    }

    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct BorrowIntIncrementCommand {
        pub address: MoveAddress,
    }
}
