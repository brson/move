#![allow(unused)]

extern crate std;

use std::vec::Vec;

use crate::rt_types::*;
use crate::vector::*;

use self::solana_storage_interface::*;

mod solana_storage_interface {
    use crate::rt_types::*;
    use super::Vec;

    pub enum Command {
        MoveFrom(MoveFromCommand),
        MoveTo(MoveToCommand),
        BorrowGlobalMutCommit(BorrowGlobalMutCommitCommand),
    }

    pub struct MoveFromCommand {
        calling_program_id: SolanaPubkey,
        address: MoveAddress,
        type_: MoveType,
    }

    pub struct MoveToCommand {
        calling_program_id: SolanaPubkey,
        address: MoveAddress,
        type_: MoveType,
        value: Vec<u8>,
    }

    pub struct BorrowGlobalMutCommitCommand {
        calling_program_id: SolanaPubkey,
        address: MoveAddress,
        type_: MoveType,
        value: Vec<u8>,
    }
}

pub unsafe fn move_from<'mv>(
    program_id: &SolanaPubkey,
    storage_program_id: &SolanaPubkey,
    accounts: &MoveBorrowedRustVec<'mv, SolanaAccountInfo>,
    address: &MoveAddress,
    type_: &MoveType,
    outptr: *mut AnyValue,
) {
    todo!()
}

pub unsafe fn move_to<'mv>(
    program_id: &SolanaPubkey,
    storage_program_id: &SolanaPubkey,
    accounts: &MoveBorrowedRustVec<'mv, SolanaAccountInfo>,
    address: &MoveAddress,
    type_: &MoveType,
    ptr: *mut AnyValue,
) {
    todo!()
}

pub unsafe fn borrow_global<'mv>(
    program_id: &SolanaPubkey,
    storage_program_id: &SolanaPubkey,
    accounts: &MoveBorrowedRustVec<'mv, SolanaAccountInfo>,
    address: &MoveAddress,
    type_: &MoveType,
    outptr: *mut AnyValue,
) {
    todo!()
}

pub unsafe fn borrow_global_mut<'mv>(
    program_id: &SolanaPubkey,
    storage_program_id: &SolanaPubkey,
    accounts: &MoveBorrowedRustVec<'mv, SolanaAccountInfo>,
    address: &MoveAddress,
    type_: &MoveType,
    outptr: *mut AnyValue,
) {
    todo!()
}

pub unsafe fn borrow_global_mut_commit<'mv>(
    program_id: &SolanaPubkey,
    storage_program_id: &SolanaPubkey,
    accounts: &MoveBorrowedRustVec<'mv, SolanaAccountInfo>,
    address: &MoveAddress,
    type_: &MoveType,
    ptr: *mut AnyValue,
) {
    todo!()
}
