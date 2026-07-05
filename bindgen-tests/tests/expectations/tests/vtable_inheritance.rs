#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct PrimaryBase__bindgen_vtable {
    pub PrimaryBase_RootMethod: unsafe extern "C" fn(this: *mut PrimaryBase),
    pub PrimaryBase_Overridden: unsafe extern "C" fn(
        this: *mut PrimaryBase,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub PrimaryBase_Alloc: unsafe extern "C" fn(
        this: *mut PrimaryBase,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub PrimaryBase_Alloc1: unsafe extern "C" fn(
        this: *mut PrimaryBase,
        size: f32,
    ) -> f32,
    pub __bindgen_destructor_complete: unsafe extern "C" fn(this: *mut PrimaryBase),
    pub __bindgen_destructor_deleting: unsafe extern "C" fn(this: *mut PrimaryBase),
}
#[repr(C)]
#[derive(Debug)]
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
impl PrimaryBase {
    #[inline]
    pub unsafe fn RootMethod(&mut self) {
        ((*(self.vtable_ as *const PrimaryBase__bindgen_vtable))
            .PrimaryBase_RootMethod)(self)
    }
    #[inline]
    pub unsafe fn Overridden(
        &mut self,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        ((*(self.vtable_ as *const PrimaryBase__bindgen_vtable))
            .PrimaryBase_Overridden)(self, value)
    }
    #[inline]
    pub unsafe fn Alloc(
        &mut self,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        ((*(self.vtable_ as *const PrimaryBase__bindgen_vtable))
            .PrimaryBase_Alloc)(self, size)
    }
    #[inline]
    pub unsafe fn Alloc1(&mut self, size: f32) -> f32 {
        ((*(self.vtable_ as *const PrimaryBase__bindgen_vtable))
            .PrimaryBase_Alloc1)(self, size)
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN11PrimaryBase10RootMethodEv"]
    pub fn PrimaryBase_RootMethod(this: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN11PrimaryBase10OverriddenEi"]
    pub fn PrimaryBase_Overridden(
        this: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN11PrimaryBase5AllocEi"]
    pub fn PrimaryBase_Alloc(
        this: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN11PrimaryBase5AllocEf"]
    pub fn PrimaryBase_Alloc1(this: *mut ::std::os::raw::c_void, size: f32) -> f32;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN11PrimaryBaseD1Ev"]
    pub fn PrimaryBase_PrimaryBase_destructor(this: *mut PrimaryBase);
}
#[repr(C)]
pub struct IntermediateClass__bindgen_vtable {
    pub PrimaryBase_RootMethod: unsafe extern "C" fn(this: *mut IntermediateClass),
    pub IntermediateClass_Overridden: unsafe extern "C" fn(
        this: *mut IntermediateClass,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub IntermediateClass_Alloc: unsafe extern "C" fn(
        this: *mut IntermediateClass,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub IntermediateClass_Alloc1: unsafe extern "C" fn(
        this: *mut IntermediateClass,
        size: f32,
    ) -> f32,
    pub __bindgen_destructor_complete: unsafe extern "C" fn(
        this: *mut IntermediateClass,
    ),
    pub __bindgen_destructor_deleting: unsafe extern "C" fn(
        this: *mut IntermediateClass,
    ),
    pub IntermediateClass_IntermediateMethod: unsafe extern "C" fn(
        this: *mut IntermediateClass,
    ),
}
#[repr(C)]
#[derive(Debug)]
pub struct IntermediateClass {
    pub _base: PrimaryBase,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IntermediateClass"][::std::mem::size_of::<IntermediateClass>() - 8usize];
    [
        "Alignment of IntermediateClass",
    ][::std::mem::align_of::<IntermediateClass>() - 8usize];
};
impl Default for IntermediateClass {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl IntermediateClass {
    #[inline]
    pub unsafe fn RootMethod(&mut self) {
        ((*(self._base.vtable_ as *const IntermediateClass__bindgen_vtable))
            .PrimaryBase_RootMethod)(self)
    }
    #[inline]
    pub unsafe fn Overridden(
        &mut self,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        ((*(self._base.vtable_ as *const IntermediateClass__bindgen_vtable))
            .IntermediateClass_Overridden)(self, value)
    }
    #[inline]
    pub unsafe fn Alloc(
        &mut self,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        ((*(self._base.vtable_ as *const IntermediateClass__bindgen_vtable))
            .IntermediateClass_Alloc)(self, size)
    }
    #[inline]
    pub unsafe fn Alloc1(&mut self, size: f32) -> f32 {
        ((*(self._base.vtable_ as *const IntermediateClass__bindgen_vtable))
            .IntermediateClass_Alloc1)(self, size)
    }
    #[inline]
    pub unsafe fn IntermediateMethod(&mut self) {
        ((*(self._base.vtable_ as *const IntermediateClass__bindgen_vtable))
            .IntermediateClass_IntermediateMethod)(self)
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN17IntermediateClass10OverriddenEi"]
    pub fn IntermediateClass_Overridden(
        this: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN17IntermediateClass5AllocEi"]
    pub fn IntermediateClass_Alloc(
        this: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN17IntermediateClass5AllocEf"]
    pub fn IntermediateClass_Alloc1(this: *mut ::std::os::raw::c_void, size: f32) -> f32;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN17IntermediateClass18IntermediateMethodEv"]
    pub fn IntermediateClass_IntermediateMethod(this: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN17IntermediateClassD1Ev"]
    pub fn IntermediateClass_IntermediateClass_destructor(this: *mut IntermediateClass);
}
#[repr(C)]
pub struct LeafClass__bindgen_vtable {
    pub PrimaryBase_RootMethod: unsafe extern "C" fn(this: *mut LeafClass),
    pub IntermediateClass_Overridden: unsafe extern "C" fn(
        this: *mut LeafClass,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub IntermediateClass_Alloc: unsafe extern "C" fn(
        this: *mut LeafClass,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub IntermediateClass_Alloc1: unsafe extern "C" fn(
        this: *mut LeafClass,
        size: f32,
    ) -> f32,
    pub __bindgen_destructor_complete: unsafe extern "C" fn(this: *mut LeafClass),
    pub __bindgen_destructor_deleting: unsafe extern "C" fn(this: *mut LeafClass),
    pub IntermediateClass_IntermediateMethod: unsafe extern "C" fn(this: *mut LeafClass),
    pub LeafClass_LeafMethod: unsafe extern "C" fn(this: *mut LeafClass),
}
#[repr(C)]
#[derive(Debug)]
pub struct LeafClass {
    pub _base: IntermediateClass,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LeafClass"][::std::mem::size_of::<LeafClass>() - 8usize];
    ["Alignment of LeafClass"][::std::mem::align_of::<LeafClass>() - 8usize];
};
impl Default for LeafClass {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl LeafClass {
    #[inline]
    pub unsafe fn RootMethod(&mut self) {
        ((*(self._base._base.vtable_ as *const LeafClass__bindgen_vtable))
            .PrimaryBase_RootMethod)(self)
    }
    #[inline]
    pub unsafe fn Overridden(
        &mut self,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        ((*(self._base._base.vtable_ as *const LeafClass__bindgen_vtable))
            .IntermediateClass_Overridden)(self, value)
    }
    #[inline]
    pub unsafe fn Alloc(
        &mut self,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        ((*(self._base._base.vtable_ as *const LeafClass__bindgen_vtable))
            .IntermediateClass_Alloc)(self, size)
    }
    #[inline]
    pub unsafe fn Alloc1(&mut self, size: f32) -> f32 {
        ((*(self._base._base.vtable_ as *const LeafClass__bindgen_vtable))
            .IntermediateClass_Alloc1)(self, size)
    }
    #[inline]
    pub unsafe fn IntermediateMethod(&mut self) {
        ((*(self._base._base.vtable_ as *const LeafClass__bindgen_vtable))
            .IntermediateClass_IntermediateMethod)(self)
    }
    #[inline]
    pub unsafe fn LeafMethod(&mut self) {
        ((*(self._base._base.vtable_ as *const LeafClass__bindgen_vtable))
            .LeafClass_LeafMethod)(self)
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN9LeafClass10LeafMethodEv"]
    pub fn LeafClass_LeafMethod(this: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN9LeafClassD1Ev"]
    pub fn LeafClass_LeafClass_destructor(this: *mut LeafClass);
}
#[repr(C)]
pub struct PassthroughClass__bindgen_vtable {
    pub PrimaryBase_RootMethod: unsafe extern "C" fn(this: *mut PassthroughClass),
    pub IntermediateClass_Overridden: unsafe extern "C" fn(
        this: *mut PassthroughClass,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub IntermediateClass_Alloc: unsafe extern "C" fn(
        this: *mut PassthroughClass,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub IntermediateClass_Alloc1: unsafe extern "C" fn(
        this: *mut PassthroughClass,
        size: f32,
    ) -> f32,
    pub __bindgen_destructor_complete: unsafe extern "C" fn(this: *mut PassthroughClass),
    pub __bindgen_destructor_deleting: unsafe extern "C" fn(this: *mut PassthroughClass),
    pub IntermediateClass_IntermediateMethod: unsafe extern "C" fn(
        this: *mut PassthroughClass,
    ),
    pub LeafClass_LeafMethod: unsafe extern "C" fn(this: *mut PassthroughClass),
}
#[repr(C)]
#[derive(Debug)]
pub struct PassthroughClass {
    pub _base: LeafClass,
    pub PassthroughValue: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PassthroughClass"][::std::mem::size_of::<PassthroughClass>() - 16usize];
    [
        "Alignment of PassthroughClass",
    ][::std::mem::align_of::<PassthroughClass>() - 8usize];
    [
        "Offset of field: PassthroughClass::PassthroughValue",
    ][::std::mem::offset_of!(PassthroughClass, PassthroughValue) - 8usize];
};
impl Default for PassthroughClass {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl PassthroughClass {
    #[inline]
    pub unsafe fn RootMethod(&mut self) {
        ((*(self._base._base._base.vtable_ as *const PassthroughClass__bindgen_vtable))
            .PrimaryBase_RootMethod)(self)
    }
    #[inline]
    pub unsafe fn Overridden(
        &mut self,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        ((*(self._base._base._base.vtable_ as *const PassthroughClass__bindgen_vtable))
            .IntermediateClass_Overridden)(self, value)
    }
    #[inline]
    pub unsafe fn Alloc(
        &mut self,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        ((*(self._base._base._base.vtable_ as *const PassthroughClass__bindgen_vtable))
            .IntermediateClass_Alloc)(self, size)
    }
    #[inline]
    pub unsafe fn Alloc1(&mut self, size: f32) -> f32 {
        ((*(self._base._base._base.vtable_ as *const PassthroughClass__bindgen_vtable))
            .IntermediateClass_Alloc1)(self, size)
    }
    #[inline]
    pub unsafe fn IntermediateMethod(&mut self) {
        ((*(self._base._base._base.vtable_ as *const PassthroughClass__bindgen_vtable))
            .IntermediateClass_IntermediateMethod)(self)
    }
    #[inline]
    pub unsafe fn LeafMethod(&mut self) {
        ((*(self._base._base._base.vtable_ as *const PassthroughClass__bindgen_vtable))
            .LeafClass_LeafMethod)(self)
    }
}
#[repr(C)]
pub struct SecondaryBase__bindgen_vtable {
    pub SecondaryBase_SecondaryMethod: unsafe extern "C" fn(
        this: *const SecondaryBase,
        value: f32,
    ) -> f32,
    pub __bindgen_destructor_complete: unsafe extern "C" fn(this: *mut SecondaryBase),
    pub __bindgen_destructor_deleting: unsafe extern "C" fn(this: *mut SecondaryBase),
}
#[repr(C)]
#[derive(Debug)]
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
impl SecondaryBase {
    #[inline]
    pub unsafe fn SecondaryMethod(&self, value: f32) -> f32 {
        ((*(self.vtable_ as *const SecondaryBase__bindgen_vtable))
            .SecondaryBase_SecondaryMethod)(self, value)
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZNK13SecondaryBase15SecondaryMethodEf"]
    pub fn SecondaryBase_SecondaryMethod(
        this: *mut ::std::os::raw::c_void,
        value: f32,
    ) -> f32;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN13SecondaryBaseD1Ev"]
    pub fn SecondaryBase_SecondaryBase_destructor(this: *mut SecondaryBase);
}
#[repr(C)]
pub struct MultipleDerivedClass__bindgen_vtable {
    pub MultipleDerivedClass_RootMethod: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
    ),
    pub IntermediateClass_Overridden: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub IntermediateClass_Alloc: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub IntermediateClass_Alloc1: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
        size: f32,
    ) -> f32,
    pub __bindgen_destructor_complete: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
    ),
    pub __bindgen_destructor_deleting: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
    ),
    pub IntermediateClass_IntermediateMethod: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
    ),
    pub LeafClass_LeafMethod: unsafe extern "C" fn(this: *mut MultipleDerivedClass),
    pub MultipleDerivedClass_MultipleOnly: unsafe extern "C" fn(
        this: *mut MultipleDerivedClass,
        value: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug)]
pub struct MultipleDerivedClass {
    pub _base: LeafClass,
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
impl MultipleDerivedClass {
    #[inline]
    pub unsafe fn RootMethod(&mut self) {
        ((*(self._base._base._base.vtable_
            as *const MultipleDerivedClass__bindgen_vtable))
            .MultipleDerivedClass_RootMethod)(self)
    }
    #[inline]
    pub unsafe fn Overridden(
        &mut self,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        ((*(self._base._base._base.vtable_
            as *const MultipleDerivedClass__bindgen_vtable))
            .IntermediateClass_Overridden)(self, value)
    }
    #[inline]
    pub unsafe fn Alloc(
        &mut self,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        ((*(self._base._base._base.vtable_
            as *const MultipleDerivedClass__bindgen_vtable))
            .IntermediateClass_Alloc)(self, size)
    }
    #[inline]
    pub unsafe fn Alloc1(&mut self, size: f32) -> f32 {
        ((*(self._base._base._base.vtable_
            as *const MultipleDerivedClass__bindgen_vtable))
            .IntermediateClass_Alloc1)(self, size)
    }
    #[inline]
    pub unsafe fn IntermediateMethod(&mut self) {
        ((*(self._base._base._base.vtable_
            as *const MultipleDerivedClass__bindgen_vtable))
            .IntermediateClass_IntermediateMethod)(self)
    }
    #[inline]
    pub unsafe fn LeafMethod(&mut self) {
        ((*(self._base._base._base.vtable_
            as *const MultipleDerivedClass__bindgen_vtable))
            .LeafClass_LeafMethod)(self)
    }
    #[inline]
    pub unsafe fn MultipleOnly(
        &mut self,
        value: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint {
        ((*(self._base._base._base.vtable_
            as *const MultipleDerivedClass__bindgen_vtable))
            .MultipleDerivedClass_MultipleOnly)(self, value)
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN20MultipleDerivedClass10RootMethodEv"]
    pub fn MultipleDerivedClass_RootMethod(this: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZThn8_NK20MultipleDerivedClass15SecondaryMethodEf"]
    pub fn MultipleDerivedClass_SecondaryMethod(
        this: *mut ::std::os::raw::c_void,
        value: f32,
    ) -> f32;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN20MultipleDerivedClass12MultipleOnlyEj"]
    pub fn MultipleDerivedClass_MultipleOnly(
        this: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN20MultipleDerivedClassD1Ev"]
    pub fn MultipleDerivedClass_MultipleDerivedClass_destructor(
        this: *mut MultipleDerivedClass,
    );
}
