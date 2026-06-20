#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct BaseVtable__bindgen_vtable {
    pub BaseVtable_BaseMethod: unsafe extern "C" fn(this: *mut BaseVtable),
    pub BaseVtable_Overridden: unsafe extern "C" fn(
        this: *mut BaseVtable,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BaseVtable {
    pub vtable_: *const BaseVtable__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BaseVtable"][::std::mem::size_of::<BaseVtable>() - 8usize];
    ["Alignment of BaseVtable"][::std::mem::align_of::<BaseVtable>() - 8usize];
};
impl Default for BaseVtable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN10BaseVtable10BaseMethodEv"]
    pub fn BaseVtable_BaseMethod(this: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN10BaseVtable10OverriddenEi"]
    pub fn BaseVtable_Overridden(
        this: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
pub struct OtherBaseVtable__bindgen_vtable {
    pub OtherBaseVtable_OtherMethod: unsafe extern "C" fn(
        this: *const OtherBaseVtable,
        value: f32,
    ) -> f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OtherBaseVtable {
    pub vtable_: *const OtherBaseVtable__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OtherBaseVtable"][::std::mem::size_of::<OtherBaseVtable>() - 8usize];
    ["Alignment of OtherBaseVtable"][::std::mem::align_of::<OtherBaseVtable>() - 8usize];
};
impl Default for OtherBaseVtable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZNK15OtherBaseVtable11OtherMethodEf"]
    pub fn OtherBaseVtable_OtherMethod(
        this: *mut ::std::os::raw::c_void,
        value: f32,
    ) -> f32;
}
#[repr(C)]
pub struct DerivedVtable__bindgen_vtable {
    pub BaseVtable_BaseMethod: unsafe extern "C" fn(this: *mut DerivedVtable),
    pub DerivedVtable_Overridden: unsafe extern "C" fn(
        this: *mut DerivedVtable,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub DerivedVtable_DerivedMethod: unsafe extern "C" fn(this: *mut DerivedVtable),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DerivedVtable {
    pub _base: BaseVtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of DerivedVtable"][::std::mem::size_of::<DerivedVtable>() - 8usize];
    ["Alignment of DerivedVtable"][::std::mem::align_of::<DerivedVtable>() - 8usize];
};
impl Default for DerivedVtable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN13DerivedVtable10OverriddenEi"]
    pub fn DerivedVtable_Overridden(
        this: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN13DerivedVtable13DerivedMethodEv"]
    pub fn DerivedVtable_DerivedMethod(this: *mut ::std::os::raw::c_void);
}
#[repr(C)]
pub struct MultiDerivedVtable__bindgen_vtable {
    pub MultiDerivedVtable_BaseMethod: unsafe extern "C" fn(
        this: *mut MultiDerivedVtable,
    ),
    pub BaseVtable_Overridden: unsafe extern "C" fn(
        this: *mut MultiDerivedVtable,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub MultiDerivedVtable_OtherMethod: unsafe extern "C" fn(
        this: *const MultiDerivedVtable,
        value: f32,
    ) -> f32,
    pub MultiDerivedVtable_MultiMethod: unsafe extern "C" fn(
        this: *mut MultiDerivedVtable,
        value: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MultiDerivedVtable {
    pub _base: BaseVtable,
    pub _base_1: OtherBaseVtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MultiDerivedVtable",
    ][::std::mem::size_of::<MultiDerivedVtable>() - 16usize];
    [
        "Alignment of MultiDerivedVtable",
    ][::std::mem::align_of::<MultiDerivedVtable>() - 8usize];
};
impl Default for MultiDerivedVtable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN18MultiDerivedVtable10BaseMethodEv"]
    pub fn MultiDerivedVtable_BaseMethod(this: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZThn8_NK18MultiDerivedVtable11OtherMethodEf"]
    pub fn MultiDerivedVtable_OtherMethod(
        this: *mut ::std::os::raw::c_void,
        value: f32,
    ) -> f32;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN18MultiDerivedVtable11MultiMethodEj"]
    pub fn MultiDerivedVtable_MultiMethod(
        this: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
