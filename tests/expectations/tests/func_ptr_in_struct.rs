/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum baz { }
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Foo {
    pub bar: ::std::option::Option<unsafe extern "C" fn(x:
                                                            ::std::os::raw::c_int,
                                                        y:
                                                            ::std::os::raw::c_int)
                                       -> baz>,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>() , 8usize);
    assert_eq! (::std::mem::align_of::<Foo>() , 8usize);
    assert_eq! (unsafe {
                & ( * ( 0 as * const Foo ) ) . bar as * const _ as usize } ,
                0usize);
}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}