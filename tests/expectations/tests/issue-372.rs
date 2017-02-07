/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Debug, Copy)]
    pub struct d {
        pub m: root::i,
    }
    #[test]
    fn bindgen_test_layout_d() {
        assert_eq!(::std::mem::size_of::<d>() , 24usize);
        assert_eq! (::std::mem::align_of::<d>() , 8usize);
        assert_eq! (unsafe {
                    & ( * ( 0 as * const d ) ) . m as * const _ as usize } ,
                    0usize);
    }
    impl Clone for d {
        fn clone(&self) -> Self { *self }
    }
    #[repr(C)]
    #[derive(Debug, Copy)]
    pub struct i {
        pub j: *mut root::i,
        pub k: *mut root::i,
        pub l: bool,
    }
    #[test]
    fn bindgen_test_layout_i() {
        assert_eq!(::std::mem::size_of::<i>() , 24usize);
        assert_eq! (::std::mem::align_of::<i>() , 8usize);
        assert_eq! (unsafe {
                    & ( * ( 0 as * const i ) ) . j as * const _ as usize } ,
                    0usize);
        assert_eq! (unsafe {
                    & ( * ( 0 as * const i ) ) . k as * const _ as usize } ,
                    8usize);
        assert_eq! (unsafe {
                    & ( * ( 0 as * const i ) ) . l as * const _ as usize } ,
                    16usize);
    }
    impl Clone for i {
        fn clone(&self) -> Self { *self }
    }
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum n {
        o = 0,
        p = 1,
        q = 2,
        r = 3,
        s = 4,
        t = 5,
        b = 6,
        ae = 7,
        e = 8,
        ag = 9,
        ah = 10,
        ai = 11,
    }
    #[repr(C)]
    pub struct F {
        pub w: [u64; 33usize],
    }
    #[test]
    fn bindgen_test_layout_F() {
        assert_eq!(::std::mem::size_of::<F>() , 264usize);
        assert_eq! (::std::mem::align_of::<F>() , 8usize);
        assert_eq! (unsafe {
                    & ( * ( 0 as * const F ) ) . w as * const _ as usize } ,
                    0usize);
    }
}