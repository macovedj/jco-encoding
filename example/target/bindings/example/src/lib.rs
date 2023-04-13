#[allow(clippy::all)]
pub mod foo {
    #[derive(Clone)]
    pub struct Input {
        pub a: wit_bindgen::rt::string::String,
        pub b: wit_bindgen::rt::string::String,
    }
    impl core::fmt::Debug for Input {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Input")
                .field("a", &self.a)
                .field("b", &self.b)
                .finish()
        }
    }
    pub trait Foo {
        fn hello_world(input: Input) -> ();
    }

    #[doc(hidden)]
    pub unsafe fn call_hello_world<T: Foo>(arg0: i32, arg1: i32, arg2: i32, arg3: i32) {
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, string::String, vec::Vec};
        let len0 = arg1 as usize;
        let len1 = arg3 as usize;
        T::hello_world(Input {
            a: String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap(),
            b: String::from_utf8(Vec::from_raw_parts(arg2 as *mut _, len1, len1)).unwrap(),
        });
    }
}

/// Declares the export of the component's world for the
/// given type.
#[macro_export]
macro_rules! export(($t:ident) => {
  const _: () = {

    #[doc(hidden)]
    #[export_name = "foo#hello-world"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_foo_hello_world(arg0: i32,arg1: i32,arg2: i32,arg3: i32,) {
      bindings::foo::call_hello_world::<$t>(arg0,arg1,arg2,arg3,)
    }

  };

  #[used]
  #[doc(hidden)]
  #[cfg(target_arch = "wasm32")]
  static __FORCE_SECTION_REF: fn() = __force_section_ref;
  #[doc(hidden)]
  #[cfg(target_arch = "wasm32")]
  fn __force_section_ref() {
    bindings::__link_section()
  }
});

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:component"]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 291] = [
    2, 0, 5, 119, 111, 114, 108, 100, 5, 119, 111, 114, 108, 100, 9, 99, 111, 109, 112, 111, 110,
    101, 110, 116, 0, 97, 115, 109, 12, 0, 1, 0, 7, 186, 1, 1, 65, 4, 1, 66, 4, 1, 114, 2, 1, 97,
    115, 1, 98, 115, 4, 5, 105, 110, 112, 117, 116, 0, 3, 0, 0, 1, 64, 1, 5, 105, 110, 112, 117,
    116, 1, 1, 0, 4, 11, 104, 101, 108, 108, 111, 45, 119, 111, 114, 108, 100, 0, 1, 2, 4, 3, 102,
    111, 111, 14, 112, 107, 103, 58, 47, 119, 111, 114, 108, 100, 47, 102, 111, 111, 5, 0, 1, 65,
    2, 1, 66, 4, 1, 114, 2, 1, 97, 115, 1, 98, 115, 4, 5, 105, 110, 112, 117, 116, 0, 3, 0, 0, 1,
    64, 1, 5, 105, 110, 112, 117, 116, 1, 1, 0, 4, 11, 104, 101, 108, 108, 111, 45, 119, 111, 114,
    108, 100, 0, 1, 2, 4, 3, 102, 111, 111, 14, 112, 107, 103, 58, 47, 119, 111, 114, 108, 100, 47,
    102, 111, 111, 5, 0, 4, 9, 99, 111, 109, 112, 111, 110, 101, 110, 116, 20, 112, 107, 103, 58,
    47, 119, 111, 114, 108, 100, 47, 99, 111, 109, 112, 111, 110, 101, 110, 116, 4, 1, 0, 45, 9,
    112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100,
    45, 98, 121, 1, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 5, 48, 46,
    54, 46, 48, 11, 21, 1, 5, 119, 111, 114, 108, 100, 10, 112, 107, 103, 58, 47, 119, 111, 114,
    108, 100, 3, 0, 0,
];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
