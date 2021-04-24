#[cfg(test)]
#[path = "./glob_test.rs"]
mod test;

use libc::{glob,globfree,glob_t,c_int};
use std::{error::Error, ffi, fmt, mem, slice, str};
use ffi::{CString,CStr};

const GLOB_BRACE: c_int = 1 << 10;

pub struct Glob {
    result: glob_t,
    pub paths: Vec<&'static str>
}


impl Glob {
    
    pub fn glob(pattern: &str) -> Result<Glob, Box<dyn Error>> {
        CString::new(pattern).map_err(Into::into)
            .and_then(|pat| {
                let mut result = unsafe { mem::zeroed::<glob_t>() };

                unsafe {
                    glob(pat.as_ptr(), GLOB_BRACE, None, &mut result)
                };

                Self::get_paths(result).map_err(Into::into)
                    .map(|paths| {
                        Glob { result, paths }
                    })
            })
    }

    fn get_paths(result: glob_t) -> Result<Vec<&'static str>, str::Utf8Error> {
        let ptrs = unsafe { slice::from_raw_parts(result.gl_pathv, result.gl_pathc) };

        ptrs.iter()
            .map(|ptr| unsafe { CStr::from_ptr(*ptr) })
            .map(|cstr| cstr.to_str())
            .collect::<Result<Vec<_>, _>>()
    }
}

impl Drop for Glob {
    fn drop(&mut self) {
        println!("DROP!!!");
        unsafe {
            globfree(&mut self.result)
        }
    }
}

impl fmt::Debug for Glob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Glob")
         .field("count", &self.result.gl_pathc)
         .finish()
    }
}

