/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]

extern crate core;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct foo {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
    pub bar: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(::core::mem::size_of::<foo>() , 16usize);
    assert_eq! (::core::mem::align_of::<foo>() , 8usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo ) ) . a as * const _ as usize } ,
                0usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo ) ) . b as * const _ as usize } ,
                4usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const foo ) ) . bar as * const _ as usize } ,
                8usize);
}
impl Clone for foo {
    fn clone(&self) -> Self { *self }
}
pub type fooFunction =
    ::core::option::Option<unsafe extern "C" fn(bar: ::std::os::raw::c_int)>;