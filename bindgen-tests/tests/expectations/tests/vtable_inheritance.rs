#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct PrimaryBase__bindgen_vtable {
    pub PrimaryBase_BaseMethod: unsafe extern "C" fn(this: *mut PrimaryBase),
    pub PrimaryBase_Overridden: unsafe extern "C" fn(
        this: *mut PrimaryBase,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PrimaryBase {
    pub vtable_: *const PrimaryBase__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PrimaryBase"][::std::mem::size_of::<PrimaryBase>() - 8usize];
    ["Alignment of PrimaryBase"][::std::mem::align_of::<PrimaryBase>() - 8usize];
};
impl Default for PrimaryBase {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN11PrimaryBase10BaseMethodEv"]
    pub fn PrimaryBase_BaseMethod(this: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN11PrimaryBase10OverriddenEi"]
    pub fn PrimaryBase_Overridden(
        this: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
pub struct SecondaryBase__bindgen_vtable {
    pub SecondaryBase_OtherMethod: unsafe extern "C" fn(
        this: *const SecondaryBase,
        value: f32,
    ) -> f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SecondaryBase {
    pub vtable_: *const SecondaryBase__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of SecondaryBase"][::std::mem::size_of::<SecondaryBase>() - 8usize];
    ["Alignment of SecondaryBase"][::std::mem::align_of::<SecondaryBase>() - 8usize];
};
impl Default for SecondaryBase {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZNK13SecondaryBase11OtherMethodEf"]
    pub fn SecondaryBase_OtherMethod(
        this: *mut ::std::os::raw::c_void,
        value: f32,
    ) -> f32;
}
#[repr(C)]
pub struct DerivedClass__bindgen_vtable {
    pub PrimaryBase_BaseMethod: unsafe extern "C" fn(this: *mut DerivedClass),
    pub DerivedClass_Overridden: unsafe extern "C" fn(
        this: *mut DerivedClass,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub DerivedClass_DerivedMethod: unsafe extern "C" fn(this: *mut DerivedClass),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DerivedClass {
    pub _base: PrimaryBase,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of DerivedClass"][::std::mem::size_of::<DerivedClass>() - 8usize];
    ["Alignment of DerivedClass"][::std::mem::align_of::<DerivedClass>() - 8usize];
};
impl Default for DerivedClass {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN12DerivedClass10OverriddenEi"]
    pub fn DerivedClass_Overridden(
        this: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN12DerivedClass13DerivedMethodEv"]
    pub fn DerivedClass_DerivedMethod(this: *mut ::std::os::raw::c_void);
}
#[repr(C)]
pub struct MultipleDerivedClass__bindgen_vtable {
    pub MultipleDerivedClass_BaseMethod: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
    ),
    pub PrimaryBase_Overridden: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub MultipleDerivedClass_OtherMethod: unsafe extern "C" fn(
        this: *const MultipleDerivedClass,
        value: f32,
    ) -> f32,
    pub MultipleDerivedClass_MultiMethod: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
        value: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MultipleDerivedClass {
    pub _base: PrimaryBase,
    pub _base_1: SecondaryBase,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MultipleDerivedClass",
    ][::std::mem::size_of::<MultipleDerivedClass>() - 16usize];
    [
        "Alignment of MultipleDerivedClass",
    ][::std::mem::align_of::<MultipleDerivedClass>() - 8usize];
};
impl Default for MultipleDerivedClass {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN20MultipleDerivedClass10BaseMethodEv"]
    pub fn MultipleDerivedClass_BaseMethod(this: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZThn8_NK20MultipleDerivedClass11OtherMethodEf"]
    pub fn MultipleDerivedClass_OtherMethod(
        this: *mut ::std::os::raw::c_void,
        value: f32,
    ) -> f32;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN20MultipleDerivedClass11MultiMethodEj"]
    pub fn MultipleDerivedClass_MultiMethod(
        this: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
