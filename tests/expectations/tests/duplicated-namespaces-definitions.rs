/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy)]
        pub struct Bar {
            pub foo: ::std::os::raw::c_int,
            pub baz: bool,
        }
        #[test]
        fn bindgen_test_layout_Bar() {
            assert_eq!(::std::mem::size_of::<Bar>() , 8usize);
            assert_eq! (::std::mem::align_of::<Bar>() , 4usize);
            assert_eq! (unsafe {
                        & ( * ( 0 as * const Bar ) ) . foo as * const _ as
                        usize } , 0usize);
            assert_eq! (unsafe {
                        & ( * ( 0 as * const Bar ) ) . baz as * const _ as
                        usize } , 4usize);
        }
        impl Clone for Bar {
            fn clone(&self) -> Self { *self }
        }
    }
    pub mod bar {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy)]
        pub struct Foo {
            pub ptr: *mut root::foo::Bar,
        }
        #[test]
        fn bindgen_test_layout_Foo() {
            assert_eq!(::std::mem::size_of::<Foo>() , 8usize);
            assert_eq! (::std::mem::align_of::<Foo>() , 8usize);
            assert_eq! (unsafe {
                        & ( * ( 0 as * const Foo ) ) . ptr as * const _ as
                        usize } , 0usize);
        }
        impl Clone for Foo {
            fn clone(&self) -> Self { *self }
        }
    }
}