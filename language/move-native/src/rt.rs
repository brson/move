// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::rt_types::{AnyValue, MoveType, MoveUntypedVector};

#[export_name = "move_rt_abort"]
fn abort(code: u64) -> ! {
    crate::target_defs::abort(code);
}

#[export_name = "move_rt_vec_destroy"]
unsafe fn vec_destroy(type_ve: &MoveType, v: MoveUntypedVector) {
    assert_eq!(0, v.length, "can't destroy vectors with elements yet");
    crate::vector::destroy_empty(type_ve, v);
}

#[export_name = "move_rt_vec_empty"]
unsafe fn vec_empty(type_ve: &MoveType) -> MoveUntypedVector {
    crate::vector::empty(type_ve)
}

#[export_name = "move_rt_vec_copy"]
unsafe fn vec_copy(type_ve: &MoveType, dstv: &mut MoveUntypedVector, srcv: &MoveUntypedVector) {
    crate::vector::copy(type_ve, dstv, srcv)
}

#[export_name = "move_rt_vec_cmp_eq"]
unsafe fn vec_cmp_eq(type_ve: &MoveType, v1: &MoveUntypedVector, v2: &MoveUntypedVector) -> bool {
    crate::vector::cmp_eq(type_ve, v1, v2)
}

#[export_name = "move_rt_struct_cmp_eq"]
pub unsafe fn struct_cmp_eq(type_ve: &MoveType, s1: &AnyValue, s2: &AnyValue) -> bool {
    use crate::conv::walk_struct_fields;
    use crate::conv::{BorrowedTypedMoveValue as BTMV, borrow_move_value_as_rust_value};

    let st_info = (*(type_ve.type_info)).struct_;
    let fields1 = walk_struct_fields(&st_info, s1);
    let fields2 = walk_struct_fields(&st_info, s2);
    for ((fld_ty1, fld_ref1, _fld_name1), (fld_ty2, fld_ref2, _fld_name2)) in Iterator::zip(fields1, fields2) {
        let rv1 = borrow_move_value_as_rust_value(fld_ty1, fld_ref1);
        let rv2 = borrow_move_value_as_rust_value(fld_ty2, fld_ref2);

        let is_eq = match (rv1, rv2) {
            (BTMV::Bool(val1), BTMV::Bool(val2)) => { val1 == val2  }
            (BTMV::U8(val1), BTMV::U8(val2)) => { val1 == val2  }
            (BTMV::U16(val1), BTMV::U16(val2)) => { val1 == val2  }
            (BTMV::U32(val1), BTMV::U32(val2)) => { val1 == val2  }
            (BTMV::U64(val1), BTMV::U64(val2)) => { val1 == val2  }
            (BTMV::U128(val1), BTMV::U128(val2)) => { val1 == val2  }
            (BTMV::U256(val1), BTMV::U256(val2)) => { val1 == val2  }
            (BTMV::Address(val1), BTMV::Address(val2)) => { val1 == val2  }
            (BTMV::Signer(val1), BTMV::Signer(val2)) => { val1 == val2  }
            (BTMV::Vector(t1, utv1), BTMV::Vector(_t2, utv2)) => {
                vec_cmp_eq(&t1, utv1, utv2)
            }
            (BTMV::Struct(t1, anyv1), BTMV::Struct(_t2, anyv2)) => {
                struct_cmp_eq(&t1, anyv1, anyv2)
            }
            (BTMV::Reference(_, _), BTMV::Reference(_, _)) => {
                unreachable!("reference in struct field impossible")
            }
            _ => { unreachable!("struct_cmp_eq unexpected value combination") }
        };

        if !is_eq { return false }
    }
    true
}
