#![allow(unused)]

extern crate std;

use std::vec::Vec;

use crate::rt_types::*;
use crate::vector::*;

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

pub unsafe fn exists<'mv>(
    program_id: &SolanaPubkey,
    storage_program_id: &SolanaPubkey,
    accounts: &MoveBorrowedRustVec<'mv, SolanaAccountInfo>,
    address: &MoveAddress,
    type_: &MoveType,
) -> bool {
    todo!()
}
