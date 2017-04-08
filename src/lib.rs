// #![allow(dead_code, non_camel_case_types, unused_variables, non_snake_case)]

mod ffi;
pub use ffi::VSGetPropErrors;

mod map;
pub use map::VSMap;

#[macro_export]
macro_rules! plugin_init {
    ($identifier:expr, $ns:expr, $name:expr, $reg:block) => (
        #[no_mangle]
        pub extern "C" fn VapourSynthPluginInit(configFunc: $crate::ffi::VSConfigPlugin,
                                                registerFunc: $crate::ffi::VSRegisterFunction,
                                                plugin: *mut $crate::ffi::VSPlugin) {

            let package = ::std::ffi::CString::new($identifier).unwrap().as_ptr();
            let ns = ::std::ffi::CString::new($ns).unwrap().as_ptr();
            let name = ::std::ffi::CString::new($name).unwrap().as_ptr();
            configFunc(package, ns, name, $crate::ffi::VAPOURSYNTH_API_VERSION as i32, 1, plugin);
            $reg
        }
    )
}

struct VSFilter {}

#[macro_export]
macro_rules! filter_reg {
    ($name:expr, $args:expr, $name_i:ident) => {

    }
}


// mod vsapi;
// use std::ffi::CString;
// use vsapi::{VSMap, VSAPI, VSCore};

// pub extern "C" fn filter_create(in_: *const VSMap,
//                                 out: *mut VSMap,
//                                 userData: *mut ::std::os::raw::c_void,
//                                 core: *mut VSCore,
//                                 vsapi: *const VSAPI) {

// }

// pub extern "C" fn VapourSynthPluginInit(configFunc: vsapi::VSConfigPlugin,
//                                         registerFunc: vsapi::VSRegisterFunction,
//                                         plugin: *mut vsapi::VSPlugin) {

//     let package = CString::new("com.example.filter").unwrap().as_ptr();
//     let ns = CString::new("filter").unwrap().as_ptr();
//     let name = CString::new("VapourSynth Filter Skeleton")
//         .unwrap()
//         .as_ptr();
//     configFunc(package,
//                ns,
//                name,
//                vsapi::VAPOURSYNTH_API_VERSION as i32,
//                1,
//                plugin);
// }
