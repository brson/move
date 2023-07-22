// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{conv::*, rt_types::*};
use alloc::vec::Vec;
use core::{mem, ops::{Deref, DerefMut}, ptr};
use core::marker::PhantomData;

pub enum TypedMoveBorrowedRustVec<'mv> {
    Bool(MoveBorrowedRustVec<'mv, bool>),
    U8(MoveBorrowedRustVec<'mv, u8>),
    U16(MoveBorrowedRustVec<'mv, u16>),
    U32(MoveBorrowedRustVec<'mv, u32>),
    U64(MoveBorrowedRustVec<'mv, u64>),
    U128(MoveBorrowedRustVec<'mv, u128>),
    U256(MoveBorrowedRustVec<'mv, U256>),
    Address(MoveBorrowedRustVec<'mv, MoveAddress>),
    Signer(MoveBorrowedRustVec<'mv, MoveSigner>),
    Vector(MoveType, MoveBorrowedRustVec<'mv, MoveUntypedVector>),
    Struct(MoveBorrowedRustVecOfStruct<'mv>),
    Reference(MoveType, MoveBorrowedRustVec<'mv, MoveUntypedReference>),
    // todo
}

#[derive(Debug)]
pub enum TypedMoveBorrowedRustVecMut<'mv> {
    Bool(MoveBorrowedRustVecMut<'mv, bool>),
    U8(MoveBorrowedRustVecMut<'mv, u8>),
    U16(MoveBorrowedRustVecMut<'mv, u16>),
    U32(MoveBorrowedRustVecMut<'mv, u32>),
    U64(MoveBorrowedRustVecMut<'mv, u64>),
    U128(MoveBorrowedRustVecMut<'mv, u128>),
    U256(MoveBorrowedRustVecMut<'mv, U256>),
    Address(MoveBorrowedRustVecMut<'mv, MoveAddress>),
    Signer(MoveBorrowedRustVecMut<'mv, MoveSigner>),
    Vector(MoveType, MoveBorrowedRustVecMut<'mv, MoveUntypedVector>),
    Struct(MoveBorrowedRustVecOfStructMut<'mv>),
    Reference(MoveType, MoveBorrowedRustVecMut<'mv, MoveUntypedReference>),
    // todo
}

pub struct MoveBorrowedRustVec<'mv, T> {
    inner: Vec<T>,
    _lifetime: PhantomData<&'mv ()>,
}

#[derive(Debug)]
pub struct MoveBorrowedRustVecMut<'mv, T> {
    inner: Vec<T>,
    original: &'mv mut MoveUntypedVector,
}

/// A vector of Move structs.
///
/// Since we can't instantiate Move structs as Rust structs, this is a
/// container that unsafely implements exactly the ops needed to deal with
/// Move's `vector<T>`.
#[derive(Debug)]
pub struct MoveBorrowedRustVecOfStruct<'mv> {
    inner: &'mv MoveUntypedVector,
    type_: &'mv StructTypeInfo,
    full_type: &'mv MoveType,
}

#[derive(Debug)]
pub struct MoveBorrowedRustVecOfStructMut<'mv> {
    inner: &'mv mut MoveUntypedVector,
    type_: &'mv StructTypeInfo,
}

impl<'mv> TypedMoveBorrowedRustVec<'mv> {
    pub unsafe fn new(
        type_: &'mv MoveType,
        mv: &'mv MoveUntypedVector,
    ) -> TypedMoveBorrowedRustVec<'mv> {
        match type_.type_desc {
            TypeDesc::Bool => TypedMoveBorrowedRustVec::Bool(MoveBorrowedRustVec::new(mv)),
            TypeDesc::U8 => TypedMoveBorrowedRustVec::U8(MoveBorrowedRustVec::new(mv)),
            TypeDesc::U16 => TypedMoveBorrowedRustVec::U16(MoveBorrowedRustVec::new(mv)),
            TypeDesc::U32 => TypedMoveBorrowedRustVec::U32(MoveBorrowedRustVec::new(mv)),
            TypeDesc::U64 => TypedMoveBorrowedRustVec::U64(MoveBorrowedRustVec::new(mv)),
            TypeDesc::U128 => TypedMoveBorrowedRustVec::U128(MoveBorrowedRustVec::new(mv)),
            TypeDesc::U256 => TypedMoveBorrowedRustVec::U256(MoveBorrowedRustVec::new(mv)),
            TypeDesc::Address => {
                TypedMoveBorrowedRustVec::Address(MoveBorrowedRustVec::new(mv))
            }
            TypeDesc::Signer => {
                TypedMoveBorrowedRustVec::Signer(MoveBorrowedRustVec::new(mv))
            }
            TypeDesc::Vector => TypedMoveBorrowedRustVec::Vector(
                *(*type_.type_info).vector.element_type,
                MoveBorrowedRustVec::new(mv),
            ),
            TypeDesc::Struct => TypedMoveBorrowedRustVec::Struct(
                MoveBorrowedRustVecOfStruct::new(type_, mv)
            ),
            TypeDesc::Reference => TypedMoveBorrowedRustVec::Reference(
                *(*type_.type_info).reference.element_type,
                MoveBorrowedRustVec::new(mv),
            ),
        }
    }
}

impl<'mv> TypedMoveBorrowedRustVecMut<'mv> {
    pub unsafe fn new(
        type_: &'mv MoveType,
        mv: &'mv mut MoveUntypedVector,
    ) -> TypedMoveBorrowedRustVecMut<'mv> {
        match type_.type_desc {
            TypeDesc::Bool => {
                TypedMoveBorrowedRustVecMut::Bool(MoveBorrowedRustVecMut::new(mv))
            }
            TypeDesc::U8 => TypedMoveBorrowedRustVecMut::U8(MoveBorrowedRustVecMut::new(mv)),
            TypeDesc::U16 => {
                TypedMoveBorrowedRustVecMut::U16(MoveBorrowedRustVecMut::new(mv))
            }
            TypeDesc::U32 => {
                TypedMoveBorrowedRustVecMut::U32(MoveBorrowedRustVecMut::new(mv))
            }
            TypeDesc::U64 => {
                TypedMoveBorrowedRustVecMut::U64(MoveBorrowedRustVecMut::new(mv))
            }
            TypeDesc::U128 => {
                TypedMoveBorrowedRustVecMut::U128(MoveBorrowedRustVecMut::new(mv))
            }
            TypeDesc::U256 => {
                TypedMoveBorrowedRustVecMut::U256(MoveBorrowedRustVecMut::new(mv))
            }
            TypeDesc::Address => {
                TypedMoveBorrowedRustVecMut::Address(MoveBorrowedRustVecMut::new(mv))
            }
            TypeDesc::Signer => {
                TypedMoveBorrowedRustVecMut::Signer(MoveBorrowedRustVecMut::new(mv))
            }
            TypeDesc::Vector => TypedMoveBorrowedRustVecMut::Vector(
                *(*type_.type_info).vector.element_type,
                MoveBorrowedRustVecMut::new(mv),
            ),
            TypeDesc::Struct => TypedMoveBorrowedRustVecMut::Struct(
                MoveBorrowedRustVecOfStructMut::new(type_, mv)
            ),
            TypeDesc::Reference => TypedMoveBorrowedRustVecMut::Reference(
                *(*type_.type_info).reference.element_type,
                MoveBorrowedRustVecMut::new(mv),
            ),
        }
    }
}

impl<'mv, T> MoveBorrowedRustVec<'mv, T> {
    pub unsafe fn new(mv: &MoveUntypedVector) -> MoveBorrowedRustVec<'_, T> {
        let rv = Vec::from_raw_parts(
            mv.ptr as *mut T,
            usize::try_from(mv.length).expect("overflow"),
            usize::try_from(mv.capacity).expect("overflow"),
        );
        MoveBorrowedRustVec {
            inner: rv,
            _lifetime: PhantomData,
        }
    }
}

impl<'mv, T> MoveBorrowedRustVecMut<'mv, T> {
    pub unsafe fn new(
        mv: &mut MoveUntypedVector,
    ) -> MoveBorrowedRustVecMut<'_, T> {
        let rv = Vec::from_raw_parts(
            mv.ptr as *mut T,
            usize::try_from(mv.length).expect("overflow"),
            usize::try_from(mv.capacity).expect("overflow"),
        );
        MoveBorrowedRustVecMut {
            inner: rv,
            original: mv,
        }
    }
}

impl<'mv, T> Drop for MoveBorrowedRustVec<'mv, T> {
    fn drop(&mut self) {
        let rv = mem::take(&mut self.inner);
        mem::forget(rv);
    }
}

impl<'mv, T> Drop for MoveBorrowedRustVecMut<'mv, T> {
    fn drop(&mut self) {
        let mut rv = mem::take(&mut self.inner);

        self.original.length = u64::try_from(rv.len()).expect("overflow");
        self.original.capacity = u64::try_from(rv.capacity()).expect("overflow");
        self.original.ptr = rv.as_mut_ptr() as *mut u8;

        mem::forget(rv);
    }
}

impl<'mv, T> Deref for MoveBorrowedRustVec<'mv, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'mv, T> Deref for MoveBorrowedRustVecMut<'mv, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'mv, T> DerefMut for MoveBorrowedRustVecMut<'mv, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'mv> MoveBorrowedRustVecOfStruct<'mv> {
    unsafe fn new(ty: &'mv MoveType, mv: &'mv MoveUntypedVector) -> MoveBorrowedRustVecOfStruct<'mv> {
        assert_eq!(ty.type_desc, TypeDesc::Struct);
        MoveBorrowedRustVecOfStruct {
            inner: mv,
            type_: &(*ty.type_info).struct_,
            full_type: ty
        }
    }
}

impl<'mv> MoveBorrowedRustVecOfStructMut<'mv> {
    unsafe fn new(ty: &'mv MoveType, mv: &'mv mut MoveUntypedVector) -> MoveBorrowedRustVecOfStructMut<'mv> {
        assert_eq!(ty.type_desc, TypeDesc::Struct);
        MoveBorrowedRustVecOfStructMut {
            inner: mv,
            type_: &(*ty.type_info).struct_,
        }
    }
}

pub fn empty(type_r: &MoveType) -> MoveUntypedVector {
    match type_r.type_desc {
        TypeDesc::Bool => rust_vec_to_move_vec::<bool>(Vec::new()),
        TypeDesc::U8 => rust_vec_to_move_vec::<u8>(Vec::new()),
        TypeDesc::U16 => rust_vec_to_move_vec::<u16>(Vec::new()),
        TypeDesc::U32 => rust_vec_to_move_vec::<u32>(Vec::new()),
        TypeDesc::U64 => rust_vec_to_move_vec::<u64>(Vec::new()),
        TypeDesc::U128 => rust_vec_to_move_vec::<u128>(Vec::new()),
        TypeDesc::U256 => rust_vec_to_move_vec::<U256>(Vec::new()),
        TypeDesc::Address => rust_vec_to_move_vec::<MoveAddress>(Vec::new()),
        TypeDesc::Signer => rust_vec_to_move_vec::<MoveSigner>(Vec::new()),
        TypeDesc::Vector => {
            // Safety: need correct alignment for the internal vector
            // pointer of the outer vector, which is non-null even for
            // an unallocated vector. `MoveUntypedVector` has the same
            // size and alignment regardless of the type it contains, so
            // no need to interpret the vector type.
            rust_vec_to_move_vec::<MoveUntypedVector>(Vec::new())
        }
        TypeDesc::Struct => unsafe {
            // Safety: this gets pretty sketchy, and relies on internal
            // Vec details that probably are not guaranteed. The most
            // _correct_ way to initialize a Vec is to call its
            // constructor.
            //
            // That is pretty tough with a type of any dynamically sized
            // layout, so we're going to munge the pointers ourselves.
            //
            // The critical thing to know about Vec's pointers is:
            //
            // - They must always be aligned correctly
            // - They are _never_ 0, even for empty Vec's, to allow null
            //   pointer optimizations.
            //
            // Vec uses `NonNull::dangling` to create invalid non-null
            // pointers, but that requires a concrete type of the
            // correct alignment. We dig even deeper and use
            // `ptr::invalid_mut`, which is an unstable function from
            // the pointer provenance project. As it is unstable we just
            // duplicate it in our `conv` module until it becomes
            // stable.
            //
            // This should be the only location in this crate where we
            // need to fabricate a pointer from an integer.
            let size = (*type_r.type_info).struct_.size;
            let size = usize::try_from(size).expect("overflow");
            let alignment = (*type_r.type_info).struct_.alignment;
            let alignment = usize::try_from(alignment).expect("overflow");

            assert!(size != 0); // can't handle ZSTs
            assert!(alignment != 0); // must have alignment
            assert!(alignment.is_power_of_two());

            let ptr = invalid_mut::<u8>(alignment);
            MoveUntypedVector {
                ptr,
                capacity: 0,
                length: 0,
            }
        },
        TypeDesc::Reference => rust_vec_to_move_vec::<MoveUntypedReference>(Vec::new()),
    }
}

pub unsafe fn destroy_empty(type_ve: &MoveType, v: MoveUntypedVector) {
    assert_eq!(v.length, 0);
    match type_ve.type_desc {
        TypeDesc::Bool => drop(move_vec_to_rust_vec::<bool>(v)),
        TypeDesc::U8 => drop(move_vec_to_rust_vec::<u8>(v)),
        TypeDesc::U16 => drop(move_vec_to_rust_vec::<u16>(v)),
        TypeDesc::U32 => drop(move_vec_to_rust_vec::<u32>(v)),
        TypeDesc::U64 => drop(move_vec_to_rust_vec::<u64>(v)),
        TypeDesc::U128 => drop(move_vec_to_rust_vec::<u128>(v)),
        TypeDesc::U256 => drop(move_vec_to_rust_vec::<U256>(v)),
        TypeDesc::Address => drop(move_vec_to_rust_vec::<MoveAddress>(v)),
        TypeDesc::Signer => drop(move_vec_to_rust_vec::<MoveSigner>(v)),
        TypeDesc::Vector => {
            // Safety: need the correct internal pointer alignment to
            // deallocate; need the outer vector to be empty to avoid
            // dropping the inner vectors. As in `empty`,
            // MoveUntypedVector should have the same size/alignment
            // regardless of the contained type, so no need to interpret
            // the vector type.
            drop(move_vec_to_rust_vec::<MoveUntypedVector>(v))
        }
        TypeDesc::Struct => {
            // Safety: like in `empty` we want to deallocate here without
            // creating a `Vec` of a concrete type, since handling the
            // alignment would requiring enumerating many types.
            //
            // So here we're just going to free the pointer ourselves,
            // constructing a correct `Layout` value to pass to the
            // allocator.
            //
            // Note that this function can only be called on empty vecs,
            // so we don't need to care about dropping elements.

            let size = (*type_ve.type_info).struct_.size;
            let size = usize::try_from(size).expect("overflow");
            let alignment = (*type_ve.type_info).struct_.alignment;
            let alignment = usize::try_from(alignment).expect("overflow");
            let capacity = usize::try_from(v.capacity).expect("overflow");

            assert!(size != 0); // can't handle ZSTs

            if capacity != 0 {
                let vec_byte_size = capacity.checked_mul(size).expect("overflow");
                let layout = alloc::alloc::Layout::from_size_align(vec_byte_size, alignment)
                    .expect("bad size or alignment");
                alloc::alloc::dealloc(v.ptr, layout);
            }
        }
        TypeDesc::Reference => drop(move_vec_to_rust_vec::<MoveUntypedReference>(v)),
    }
}

pub unsafe fn length(type_ve: &MoveType, v: &MoveUntypedVector) -> u64 {
    let rust_vec = TypedMoveBorrowedRustVec::new(type_ve, v);
    rust_vec.len()
}

pub unsafe fn borrow<'v>(type_ve: &'v MoveType, v: &'v MoveUntypedVector, i: u64) -> &'v AnyValue {
    let rust_vec = TypedMoveBorrowedRustVec::new(type_ve, v);
    rust_vec.borrow(i)
}

impl<'mv> TypedMoveBorrowedRustVec<'mv> {
    pub fn len(&self) -> u64 {
        let len = match self {
            TypedMoveBorrowedRustVec::Bool(v) => v.len(),
            TypedMoveBorrowedRustVec::U8(v) => v.len(),
            TypedMoveBorrowedRustVec::U16(v) => v.len(),
            TypedMoveBorrowedRustVec::U32(v) => v.len(),
            TypedMoveBorrowedRustVec::U64(v) => v.len(),
            TypedMoveBorrowedRustVec::U128(v) => v.len(),
            TypedMoveBorrowedRustVec::U256(v) => v.len(),
            TypedMoveBorrowedRustVec::Address(v) => v.len(),
            TypedMoveBorrowedRustVec::Signer(v) => v.len(),
            TypedMoveBorrowedRustVec::Vector(_t, v) => v.len(),
            TypedMoveBorrowedRustVec::Struct(s) => s.len(),
            TypedMoveBorrowedRustVec::Reference(_t, v) => v.len(),
        };

        u64::try_from(len).expect("u64")
    }

    pub fn borrow(&self, i: u64) -> &'mv AnyValue {
        unsafe {
            let i = usize::try_from(i).expect("usize");
            let value = match self {
                TypedMoveBorrowedRustVec::Bool(v) => mem::transmute(&v[i]),
                TypedMoveBorrowedRustVec::U8(v) => mem::transmute(&v[i]),
                TypedMoveBorrowedRustVec::U16(v) => mem::transmute(&v[i]),
                TypedMoveBorrowedRustVec::U32(v) => mem::transmute(&v[i]),
                TypedMoveBorrowedRustVec::U64(v) => mem::transmute(&v[i]),
                TypedMoveBorrowedRustVec::U128(v) => mem::transmute(&v[i]),
                TypedMoveBorrowedRustVec::U256(v) => mem::transmute(&v[i]),
                TypedMoveBorrowedRustVec::Address(v) => mem::transmute(&v[i]),
                TypedMoveBorrowedRustVec::Signer(v) => mem::transmute(&v[i]),
                TypedMoveBorrowedRustVec::Vector(_t, v) => mem::transmute(&v[i]),
                TypedMoveBorrowedRustVec::Struct(s) => s.get(i),
                TypedMoveBorrowedRustVec::Reference(_t, v) => mem::transmute(&v[i]),
            };

            value
        }
    }
}


pub unsafe fn push_back(type_ve: &MoveType, v: &mut MoveUntypedVector, e: *mut AnyValue) {
    let mut rust_vec = TypedMoveBorrowedRustVecMut::new(type_ve, v);
    rust_vec.push_back(e)
}

pub unsafe fn borrow_mut(
    type_ve: &MoveType,
    v: &mut MoveUntypedVector,
    i: u64,
) -> *mut AnyValue {
    let rust_vec = TypedMoveBorrowedRustVecMut::new(type_ve, v);
    rust_vec.borrow_mut(i)
}
/*pub unsafe fn borrow_mut(
    type_ve: &MoveType,
    v: &mut MoveUntypedVector,
    i: u64,
) -> *mut AnyValue {
    let rust_vec = TypedMoveBorrowedRustVecMut::new(type_ve, v);
    let i = usize::try_from(i).expect("usize");
    match rust_vec {
        TypedMoveBorrowedRustVecMut::Bool(mut v) => mem::transmute(&mut v[i]),
        TypedMoveBorrowedRustVecMut::U8(mut v) => mem::transmute(&mut v[i]),
        TypedMoveBorrowedRustVecMut::U16(mut v) => mem::transmute(&mut v[i]),
        TypedMoveBorrowedRustVecMut::U32(mut v) => mem::transmute(&mut v[i]),
        TypedMoveBorrowedRustVecMut::U64(mut v) => mem::transmute(&mut v[i]),
        TypedMoveBorrowedRustVecMut::U128(mut v) => mem::transmute(&mut v[i]),
        TypedMoveBorrowedRustVecMut::U256(mut v) => mem::transmute(&mut v[i]),
        TypedMoveBorrowedRustVecMut::Address(mut v) => mem::transmute(&mut v[i]),
        TypedMoveBorrowedRustVecMut::Signer(mut v) => mem::transmute(&mut v[i]),
        TypedMoveBorrowedRustVecMut::Vector(_t, mut v) => mem::transmute(&mut v[i]),
        TypedMoveBorrowedRustVecMut::Struct(mut s) => s.get_mut(i),
        TypedMoveBorrowedRustVecMut::Reference(_t, mut v) => mem::transmute(&mut v[i]),
    }
}*/

impl<'mv> TypedMoveBorrowedRustVecMut<'mv> {
    pub unsafe fn push_back(&mut self, e: *mut AnyValue) {
        match self {
            TypedMoveBorrowedRustVecMut::Bool(ref mut v) => v.push(ptr::read(e as *const bool)),
            TypedMoveBorrowedRustVecMut::U8(ref mut v) => v.push(ptr::read(e as *const u8)),
            TypedMoveBorrowedRustVecMut::U16(ref mut v) => v.push(ptr::read(e as *const u16)),
            TypedMoveBorrowedRustVecMut::U32(ref mut v) => v.push(ptr::read(e as *const u32)),
            TypedMoveBorrowedRustVecMut::U64(ref mut v) => v.push(ptr::read(e as *const u64)),
            TypedMoveBorrowedRustVecMut::U128(ref mut v) => v.push(ptr::read(e as *const u128)),
            TypedMoveBorrowedRustVecMut::U256(ref mut v) => v.push(ptr::read(e as *const U256)),
            TypedMoveBorrowedRustVecMut::Address(ref mut v) => v.push(ptr::read(e as *const MoveAddress)),
            TypedMoveBorrowedRustVecMut::Signer(ref mut v) => v.push(ptr::read(e as *const MoveSigner)),
            TypedMoveBorrowedRustVecMut::Vector(_t, ref mut v) => {
                v.push(ptr::read(e as *const MoveUntypedVector))
            }
            TypedMoveBorrowedRustVecMut::Struct(ref mut s) => s.push(e),
            TypedMoveBorrowedRustVecMut::Reference(_t, ref mut v) => {
                v.push(ptr::read(e as *const MoveUntypedReference))
            }
        }
    }

    pub fn borrow_mut(
        self,
        i: u64,
    ) -> *mut AnyValue {
        unsafe {
            let i = usize::try_from(i).expect("usize");
            match self {
                TypedMoveBorrowedRustVecMut::Bool(mut v) => mem::transmute(&mut v[i]),
                TypedMoveBorrowedRustVecMut::U8(mut v) => mem::transmute(&mut v[i]),
                TypedMoveBorrowedRustVecMut::U16(mut v) => mem::transmute(&mut v[i]),
                TypedMoveBorrowedRustVecMut::U32(mut v) => mem::transmute(&mut v[i]),
                TypedMoveBorrowedRustVecMut::U64(mut v) => mem::transmute(&mut v[i]),
                TypedMoveBorrowedRustVecMut::U128(mut v) => mem::transmute(&mut v[i]),
                TypedMoveBorrowedRustVecMut::U256(mut v) => mem::transmute(&mut v[i]),
                TypedMoveBorrowedRustVecMut::Address(mut v) => mem::transmute(&mut v[i]),
                TypedMoveBorrowedRustVecMut::Signer(mut v) => mem::transmute(&mut v[i]),
                TypedMoveBorrowedRustVecMut::Vector(_t, mut v) => mem::transmute(&mut v[i]),
                TypedMoveBorrowedRustVecMut::Struct(mut s) => s.get_mut(i),
                TypedMoveBorrowedRustVecMut::Reference(_t, mut v) => mem::transmute(&mut v[i]),
            }
        }
    }


    pub unsafe fn pop_back(&mut self, r: *mut AnyValue) {
        let msg = "popping from empty vec";
        match self {
            TypedMoveBorrowedRustVecMut::Bool(ref mut v) => {
                ptr::write(r as *mut bool, v.pop().expect(msg));
            }
            TypedMoveBorrowedRustVecMut::U8(ref mut v) => {
                ptr::write(r as *mut u8, v.pop().expect(msg));
            }
            TypedMoveBorrowedRustVecMut::U16(ref mut v) => {
                ptr::write(r as *mut u16, v.pop().expect(msg));
            }
            TypedMoveBorrowedRustVecMut::U32(ref mut v) => {
                ptr::write(r as *mut u32, v.pop().expect(msg));
            }
            TypedMoveBorrowedRustVecMut::U64(ref mut v) => {
                ptr::write(r as *mut u64, v.pop().expect(msg));
            }
            TypedMoveBorrowedRustVecMut::U128(ref mut v) => {
                ptr::write(r as *mut u128, v.pop().expect(msg));
            }
            TypedMoveBorrowedRustVecMut::U256(ref mut v) => {
                ptr::write(r as *mut U256, v.pop().expect(msg));
            }
            TypedMoveBorrowedRustVecMut::Address(ref mut v) => {
                ptr::write(r as *mut MoveAddress, v.pop().expect(msg));
            }
            TypedMoveBorrowedRustVecMut::Signer(ref mut v) => {
                ptr::write(r as *mut MoveSigner, v.pop().expect(msg));
            }
            TypedMoveBorrowedRustVecMut::Vector(_t, ref mut v) => {
                ptr::write(r as *mut MoveUntypedVector, v.pop().expect(msg));
            }
            TypedMoveBorrowedRustVecMut::Struct(ref mut s) => s.pop_into(r),
            TypedMoveBorrowedRustVecMut::Reference(_t, ref mut v) => {
                ptr::write(r as *mut MoveUntypedReference, v.pop().expect(msg));
            }
        }
    }
}

pub unsafe fn pop_back(type_ve: &MoveType, v: &mut MoveUntypedVector, r: *mut AnyValue) {
    let mut rust_vec = TypedMoveBorrowedRustVecMut::new(type_ve, v);
    rust_vec.pop_back(r)
}
/*pub unsafe fn pop_back(type_ve: &MoveType, v: &mut MoveUntypedVector, r: *mut AnyValue) {
    let rust_vec = TypedMoveBorrowedRustVecMut::new(type_ve, v);
    let msg = "popping from empty vec";
    match rust_vec {
        TypedMoveBorrowedRustVecMut::Bool(mut v) => {
            ptr::write(r as *mut bool, v.pop().expect(msg));
        }
        TypedMoveBorrowedRustVecMut::U8(mut v) => {
            ptr::write(r as *mut u8, v.pop().expect(msg));
        }
        TypedMoveBorrowedRustVecMut::U16(mut v) => {
            ptr::write(r as *mut u16, v.pop().expect(msg));
        }
        TypedMoveBorrowedRustVecMut::U32(mut v) => {
            ptr::write(r as *mut u32, v.pop().expect(msg));
        }
        TypedMoveBorrowedRustVecMut::U64(mut v) => {
            ptr::write(r as *mut u64, v.pop().expect(msg));
        }
        TypedMoveBorrowedRustVecMut::U128(mut v) => {
            ptr::write(r as *mut u128, v.pop().expect(msg));
        }
        TypedMoveBorrowedRustVecMut::U256(mut v) => {
            ptr::write(r as *mut U256, v.pop().expect(msg));
        }
        TypedMoveBorrowedRustVecMut::Address(mut v) => {
            ptr::write(r as *mut MoveAddress, v.pop().expect(msg));
        }
        TypedMoveBorrowedRustVecMut::Signer(mut v) => {
            ptr::write(r as *mut MoveSigner, v.pop().expect(msg));
        }
        TypedMoveBorrowedRustVecMut::Vector(_t, mut v) => {
            ptr::write(r as *mut MoveUntypedVector, v.pop().expect(msg));
        }
        TypedMoveBorrowedRustVecMut::Struct(mut s) => s.pop_into(r),
        TypedMoveBorrowedRustVecMut::Reference(_t, mut v) => {
            ptr::write(r as *mut MoveUntypedReference, v.pop().expect(msg));
        }
    }
}*/

pub unsafe fn swap(type_ve: &MoveType, v: &mut MoveUntypedVector, i: u64, j: u64) {
    let i = usize::try_from(i).expect("usize");
    let j = usize::try_from(j).expect("usize");

    let rust_vec = TypedMoveBorrowedRustVecMut::new(type_ve, v);

    match rust_vec {
        TypedMoveBorrowedRustVecMut::Bool(mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::U8(mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::U16(mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::U32(mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::U64(mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::U128(mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::U256(mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::Address(mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::Signer(mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::Vector(_t, mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::Struct(mut v) => v.swap(i, j),
        TypedMoveBorrowedRustVecMut::Reference(_t, mut v) => v.swap(i, j),
    }
}

pub unsafe fn copy(type_ve: &MoveType, dstv: &mut MoveUntypedVector, srcv: &MoveUntypedVector) {
    let src_len = length(type_ve, srcv);
    let dst_len = length(type_ve, dstv);

    // Drain the destination first.
    for _ in 0..dst_len {
        pop_back_discard(type_ve, dstv);
    }

    // Now copy.
    for i in 0..src_len {
        let se = borrow(type_ve, srcv, i);
        let septr = se as *const AnyValue as *mut AnyValue;
        push_back(type_ve, dstv, septr);
    }
}

unsafe fn pop_back_discard(type_ve: &MoveType, v: &mut MoveUntypedVector) {
    let rust_vec = TypedMoveBorrowedRustVecMut::new(type_ve, v);

    let msg = "popping from empty vec";
    match rust_vec {
        TypedMoveBorrowedRustVecMut::Bool(mut v) => {
            v.pop().expect(msg);
        }
        TypedMoveBorrowedRustVecMut::U8(mut v) => {
            v.pop().expect(msg);
        }
        TypedMoveBorrowedRustVecMut::U16(mut v) => {
            v.pop().expect(msg);
        }
        TypedMoveBorrowedRustVecMut::U32(mut v) => {
            v.pop().expect(msg);
        }
        TypedMoveBorrowedRustVecMut::U64(mut v) => {
            v.pop().expect(msg);
        }
        TypedMoveBorrowedRustVecMut::U128(mut v) => {
            v.pop().expect(msg);
        }
        TypedMoveBorrowedRustVecMut::U256(mut v) => {
            v.pop().expect(msg);
        }
        TypedMoveBorrowedRustVecMut::Address(mut v) => {
            v.pop().expect(msg);
        }
        TypedMoveBorrowedRustVecMut::Signer(mut v) => {
            v.pop().expect(msg);
        }
        TypedMoveBorrowedRustVecMut::Vector(_t, mut v) => {
            v.pop().expect(msg);
        }
        TypedMoveBorrowedRustVecMut::Struct(mut _v) => {
            todo!();
        }
        TypedMoveBorrowedRustVecMut::Reference(_t, mut v) => {
            v.pop().expect(msg);
        }
    };
}

pub unsafe fn cmp_eq(type_ve: &MoveType, v1: &MoveUntypedVector, v2: &MoveUntypedVector) -> bool {
    let v1_len = length(type_ve, v1);
    let v2_len = length(type_ve, v2);

    if v1_len != v2_len {
        return false;
    }

    let is_eq = match type_ve.type_desc {
        TypeDesc::Bool => {
            let rv1 = MoveBorrowedRustVec::<bool>::new(v1);
            let rv2 = MoveBorrowedRustVec::<bool>::new(v2);
            rv1.deref().eq(rv2.deref())
        }
        TypeDesc::U8 => {
            let rv1 = MoveBorrowedRustVec::<u8>::new(v1);
            let rv2 = MoveBorrowedRustVec::<u8>::new(v2);
            rv1.deref().eq(rv2.deref())
        }
        TypeDesc::U16 => {
            let rv1 = MoveBorrowedRustVec::<u16>::new(v1);
            let rv2 = MoveBorrowedRustVec::<u16>::new(v2);
            rv1.deref().eq(rv2.deref())
        }
        TypeDesc::U32 => {
            let rv1 = MoveBorrowedRustVec::<u32>::new(v1);
            let rv2 = MoveBorrowedRustVec::<u32>::new(v2);
            rv1.deref().eq(rv2.deref())
        }
        TypeDesc::U64 => {
            let rv1 = MoveBorrowedRustVec::<u64>::new(v1);
            let rv2 = MoveBorrowedRustVec::<u64>::new(v2);
            rv1.deref().eq(rv2.deref())
        }
        TypeDesc::U128 => {
            let rv1 = MoveBorrowedRustVec::<u128>::new(v1);
            let rv2 = MoveBorrowedRustVec::<u128>::new(v2);
            rv1.deref().eq(rv2.deref())
        }
        TypeDesc::U256 => {
            let rv1 = MoveBorrowedRustVec::<U256>::new(v1);
            let rv2 = MoveBorrowedRustVec::<U256>::new(v2);
            rv1.deref().eq(rv2.deref())
        }
        TypeDesc::Address => {
            let rv1 = MoveBorrowedRustVec::<MoveAddress>::new(v1);
            let rv2 = MoveBorrowedRustVec::<MoveAddress>::new(v2);
            rv1.deref().eq(rv2.deref())
        }
        TypeDesc::Signer => {
            let rv1 = MoveBorrowedRustVec::<MoveSigner>::new(v1);
            let rv2 = MoveBorrowedRustVec::<MoveSigner>::new(v2);
            rv1.deref().eq(rv2.deref())
        }
        TypeDesc::Vector => {
            assert!(v1_len == v2_len, "unexpected vec cmp lengths");
            let inner_element_type = *(*type_ve.type_info).vector.element_type;
            let mut tmp_result = true;
            for i in 0..v1_len {
                let anyval_ref1 = borrow(type_ve, v1, i);
                let anyval_ref2 = borrow(type_ve, v2, i);
                let mv_ut_vec1 = &*(anyval_ref1 as *const AnyValue as *const MoveUntypedVector);
                let mv_ut_vec2 = &*(anyval_ref2 as *const AnyValue as *const MoveUntypedVector);
                tmp_result = cmp_eq(&inner_element_type, mv_ut_vec1, mv_ut_vec2);
                if !tmp_result {
                    break;
                }
            }
            tmp_result
        }
        TypeDesc::Struct => {
            assert!(v1_len == v2_len, "unexpected vec cmp lengths");
            let mut tmp_result = true;
            for i in 0..v1_len {
                let anyval_ref1 = borrow(type_ve, v1, i);
                let anyval_ref2 = borrow(type_ve, v2, i);
                tmp_result = crate::structs::cmp_eq(type_ve, anyval_ref1, anyval_ref2);
                if !tmp_result {
                    break;
                }
            }
            tmp_result
        }
        _ => todo!(
            "vec_cmp_eq: unhandled element type: {:?}",
            type_ve.type_desc
        ),
    };
    is_eq
}

impl<'mv> MoveBorrowedRustVecOfStruct<'mv> {
    pub fn len(&self) -> usize {
        self.inner.length.try_into().expect("overflow")
    }

    pub fn type_(&self) -> &MoveType {
        self.full_type
    }

    pub unsafe fn iter(&self) -> impl Iterator<Item = &AnyValue> {
        let struct_size = usize::try_from(self.type_.size).expect("overflow");
        let vec_len = usize::try_from(self.inner.length).expect("overflow");
        (0..vec_len).map(move |i| {
            let base_ptr = self.inner.ptr;
            let offset = i.checked_mul(struct_size).expect("overflow");
            let offset = isize::try_from(offset).expect("overflow");
            let element_ptr = base_ptr.offset(offset);
            &*(element_ptr as *const AnyValue)
        })
    }

    pub unsafe fn get(&self, i: usize) -> &'mv AnyValue {
        let struct_size = usize::try_from(self.type_.size).expect("overflow");
        let vec_len = usize::try_from(self.inner.length).expect("overflow");

        if i >= vec_len {
            panic!("index out of bounds");
        }

        let base_ptr = self.inner.ptr;
        let offset = i.checked_mul(struct_size).expect("overflow");
        let offset = isize::try_from(offset).expect("overflow");
        let element_ptr = base_ptr.offset(offset);
        &*(element_ptr as *const AnyValue)
    }
}

impl<'mv> MoveBorrowedRustVecOfStructMut<'mv> {
    pub unsafe fn get_mut(&mut self, i: usize) -> &'mv mut AnyValue {
        let struct_size = usize::try_from(self.type_.size).expect("overflow");
        let vec_len = usize::try_from(self.inner.length).expect("overflow");

        if i >= vec_len {
            panic!("index out of bounds");
        }

        let base_ptr = self.inner.ptr;
        let offset = i.checked_mul(struct_size).expect("overflow");
        let offset = isize::try_from(offset).expect("overflow");
        let element_ptr = base_ptr.offset(offset);
        &mut *(element_ptr as *mut AnyValue)
    }

    /// Get a pointer to a possibly-uninitialized element.
    pub unsafe fn get_mut_unchecked_raw(&mut self, i: usize) -> *mut AnyValue {
        let struct_size = usize::try_from(self.type_.size).expect("overflow");
        let vec_capacity = usize::try_from(self.inner.capacity).expect("overflow");

        if i >= vec_capacity {
            panic!("index out of bounds");
        }

        let base_ptr = self.inner.ptr;
        let offset = i.checked_mul(struct_size).expect("overflow");
        let offset = isize::try_from(offset).expect("overflow");
        let element_ptr = base_ptr.offset(offset);
        element_ptr as *mut AnyValue
    }

    pub unsafe fn set_length(&mut self, len: usize) {
        let vec_capacity = usize::try_from(self.inner.capacity).expect("overflow");

        if len > vec_capacity {
            panic!("index greater than capacity");
        }

        let len = u64::try_from(len).expect("overflow");
        self.inner.length = len;
    }

    pub unsafe fn push(&mut self, ptr: *mut AnyValue) {
        self.maybe_grow();

        let struct_size = usize::try_from(self.type_.size).expect("overflow");
        let vec_len = usize::try_from(self.inner.length).expect("overflow");
        let vec_cap = usize::try_from(self.inner.capacity).expect("overflow");

        assert!(vec_len < vec_cap);

        let i = vec_len;

        let base_ptr = self.inner.ptr;
        let offset = i.checked_mul(struct_size).expect("overflow");
        let offset = isize::try_from(offset).expect("overflow");
        let element_ptr = base_ptr.offset(offset);

        let src_ptr = ptr as *mut u8;
        ptr::copy_nonoverlapping(src_ptr, element_ptr, struct_size);

        self.inner.length = self.inner.length.checked_add(1).expect("overflow");
    }

    pub unsafe fn maybe_grow(&mut self) {
        let vec_len = usize::try_from(self.inner.length).expect("overflow");
        let vec_cap = usize::try_from(self.inner.capacity).expect("overflow");

        if vec_len < vec_cap {
            return;
        }

        assert_eq!(vec_len, vec_cap);

        self.grow_amortized();
    }

    /// This is approximately like `RawVec::grow_amortized`.
    ///
    /// It always produces a power-of-two capacity.
    #[cold]
    pub unsafe fn grow_amortized(&mut self) {
        let struct_size = usize::try_from(self.type_.size).expect("overflow");
        let vec_len = usize::try_from(self.inner.length).expect("overflow");
        let vec_cap = usize::try_from(self.inner.capacity).expect("overflow");

        assert_eq!(vec_len, vec_cap);

        // Same as RawVec
        let min_non_zero_cap = if struct_size == 1 {
            8
        } else if struct_size <= 1024 {
            4
        } else {
            1
        };

        let new_cap = vec_cap.checked_mul(2).expect("overflow");
        let new_cap = core::cmp::max(new_cap, min_non_zero_cap);

        self.reserve_exact(new_cap);
    }

    pub unsafe fn reserve_exact(&mut self, new_cap: usize) {
        let struct_size = usize::try_from(self.type_.size).expect("overflow");
        let struct_align = usize::try_from(self.type_.alignment).expect("overflow");
        let vec_cap = usize::try_from(self.inner.capacity).expect("overflow");
        let new_cap_u64 = u64::try_from(new_cap).expect("overflow");

        assert!(struct_size != 0); // can't handle ZSTs
        assert!(new_cap >= vec_cap);

        let old_vec_byte_size = vec_cap.checked_mul(struct_size).expect("overflow");
        let new_vec_byte_size = new_cap.checked_mul(struct_size).expect("overflow");
        let new_layout = alloc::alloc::Layout::from_size_align(new_vec_byte_size, struct_align)
            .expect("bad size or alignment");

        if vec_cap == 0 {
            let new_ptr = alloc::alloc::alloc(new_layout);
            if new_ptr.is_null() {
                alloc::alloc::handle_alloc_error(new_layout);
            }
            self.inner.ptr = new_ptr;
            self.inner.capacity = new_cap_u64;
        } else {
            let old_layout = alloc::alloc::Layout::from_size_align(old_vec_byte_size, struct_align)
                .expect("bad size or alignment");

            let new_ptr = alloc::alloc::realloc(self.inner.ptr, old_layout, new_vec_byte_size);
            if new_ptr.is_null() {
                alloc::alloc::handle_alloc_error(new_layout);
            }
            self.inner.ptr = new_ptr;
            self.inner.capacity = new_cap_u64;
        }
    }

    pub unsafe fn pop_into(&mut self, ptr: *mut AnyValue) {
        let struct_size = usize::try_from(self.type_.size).expect("overflow");
        let vec_len = usize::try_from(self.inner.length).expect("overflow");

        let i = vec_len.checked_sub(1).expect("popping empty vector");

        let base_ptr = self.inner.ptr;
        let offset = i.checked_mul(struct_size).expect("overflow");
        let offset = isize::try_from(offset).expect("overflow");
        let element_ptr = base_ptr.offset(offset);

        let dest_ptr = ptr as *mut u8;
        ptr::copy_nonoverlapping(element_ptr, dest_ptr, struct_size);

        self.inner.length = self.inner.length.checked_sub(1).expect("overflow");
    }

    pub unsafe fn swap(&mut self, i: usize, j: usize) {
        let struct_size = usize::try_from(self.type_.size).expect("overflow");
        let vec_len = usize::try_from(self.inner.length).expect("overflow");

        if i >= vec_len || j >= vec_len {
            panic!("index out of bounds");
        }

        // Safety: must avoid overlapping pointers in swap_nonoverlapping
        // below.
        if i == j {
            return;
        }

        let base_ptr = self.inner.ptr;

        let i_offset = i.checked_mul(struct_size).expect("overflow");
        let i_offset = isize::try_from(i_offset).expect("overflow");
        let i_element_ptr = base_ptr.offset(i_offset);
        let j_offset = j.checked_mul(struct_size).expect("overflow");
        let j_offset = isize::try_from(j_offset).expect("overflow");
        let j_element_ptr = base_ptr.offset(j_offset);

        // Safety: because of the presense of uninitialized padding bytes,
        // we must (I think) do this swap with raw pointers, not slices.
        ptr::swap_nonoverlapping(i_element_ptr, j_element_ptr, struct_size);
    }
}
