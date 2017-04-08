#![allow(dead_code)]
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw;

use ffi;
use ffi::VSGetPropErrors;

#[derive(Debug)]
pub struct GetPropError<'a> {
    map_error: Option<&'a str>,
    get_error: Option<VSGetPropErrors>,
}

use std::error;
use std::fmt;

impl<'a> error::Error for GetPropError<'a> {
    fn description(&self) -> &str {
        if let Some(map_err) = self.map_error {
            return map_err;
        } else if let Some(get_err) = self.get_error {
            return match get_err {
                       VSGetPropErrors::peIndex => "Out of index",
                       VSGetPropErrors::peType => "Requested invalid type",
                       VSGetPropErrors::peUnset => "Requested key was not found",
                   };
        }
        unreachable!()
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl<'a> fmt::Display for GetPropError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        if let Some(_) = self.map_error {
            return write!(f, "Map error: {}", self.description());
        } else if let Some(_) = self.get_error {
            return write!(f, "Get error: {}", self.description());
        }
        unreachable!()
    }
}

pub type GetPropResult<'a, T> = Result<T, GetPropError<'a>>;

#[derive(Debug)]
pub struct VSMap {
    vsapi: *const ffi::VSAPI,
    map: *const ffi::VSMap,
}

macro_rules! check_map {
    ($self_:ident) => (
        if let Some(err) = $self_.check_map() {
            return Err(err)
        }
    )
}

type PropGetFunc<T> = extern "C" fn(map: *const ffi::VSMap,
                                     key: *const raw::c_char,
                                     index: raw::c_int,
                                     error: *mut raw::c_int) -> T;

impl VSMap {
    fn check_map(&self) -> Option<GetPropError> {
        unsafe {
            ((*self.vsapi).getError)(self.map)
                .as_ref()
                .map(|p| {
                         GetPropError {
                             map_error: Some(CStr::from_ptr(p).to_str().unwrap()),
                             get_error: None,
                         }
                     })
        }
    }

    fn get_using<T>(&self,
                 key: &str,
                 index: raw::c_int,
                 func: PropGetFunc<T>) -> GetPropResult<T> {
        let mut err: raw::c_int = 0;
        let result = func(self.map, CString::new(key).unwrap().as_ptr(), index, &mut err);

        if err != 0 {
            Err(GetPropError {
                map_error: None, get_error: Some(unsafe {::std::mem::transmute(err)})
            })
        } else {
            Ok(result)
        }
    }

    fn get_int(&self, key: &str, index: raw::c_int) -> GetPropResult<raw::c_longlong> {
        check_map!(self);
        self.get_using(key, index, unsafe {(*self.vsapi).propGetInt})
    }

    fn get_float(&self, key: &str, index: raw::c_int) -> GetPropResult<raw::c_double> {
        check_map!(self);
        self.get_using(key, index, unsafe {(*self.vsapi).propGetFloat})
    }
}