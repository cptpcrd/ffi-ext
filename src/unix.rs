use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::*;

use crate::OsStrExt2;

impl OsStrExt2 for OsStr {
    fn starts_with(&self, prefix: &OsStr) -> bool {
        self.as_bytes().starts_with(prefix.as_bytes())
    }

    fn ends_with(&self, suffix: &OsStr) -> bool {
        self.as_bytes().ends_with(suffix.as_bytes())
    }

    #[cfg(feature = "twoway")]
    #[inline]
    fn find_substr(&self, substr: &OsStr) -> Option<usize> {
        twoway::find_bytes(self.as_bytes(), substr.as_bytes())
    }

    #[cfg(not(feature = "twoway"))]
    fn find_substr(&self, substr: &OsStr) -> Option<usize> {
        if substr.is_empty() {
            return Some(0);
        } else if self.is_empty() {
            return None;
        }

        let bytes = self.as_bytes();
        let substr_bytes = substr.as_bytes();

        #[cfg(feature = "memchr")]
        let indices = match substr_bytes.len() {
            1 => return memchr::memchr(substr_bytes[0], bytes),
            len => {
                memchr::memchr_iter(substr_bytes[0], &bytes[..bytes.len().checked_sub(len)? + 1])
            }
        };
        #[cfg(not(feature = "memchr"))]
        let indices = 0..=(bytes.len().checked_sub(substr_bytes.len())?);

        for i in indices {
            if &bytes[i..i + substr_bytes.len()] == substr_bytes {
                return Some(i);
            }
        }

        None
    }

    #[cfg(feature = "twoway")]
    #[inline]
    fn rfind_substr(&self, substr: &OsStr) -> Option<usize> {
        twoway::rfind_bytes(self.as_bytes(), substr.as_bytes())
    }

    #[cfg(not(feature = "twoway"))]
    fn rfind_substr(&self, substr: &OsStr) -> Option<usize> {
        if substr.is_empty() {
            return Some(self.as_bytes().len());
        } else if self.is_empty() {
            return None;
        }

        let bytes = self.as_bytes();
        let substr_bytes = substr.as_bytes();

        #[cfg(feature = "memchr")]
        let indices = match substr_bytes.len() {
            1 => return memchr::memrchr(substr_bytes[0], bytes),
            len => {
                memchr::memrchr_iter(substr_bytes[0], &bytes[..bytes.len().checked_sub(len)? + 1])
            }
        };
        #[cfg(not(feature = "memchr"))]
        let indices = (0..=(bytes.len().checked_sub(substr_bytes.len())?)).rev();

        for i in indices {
            if &bytes[i..i + substr_bytes.len()] == substr_bytes {
                return Some(i);
            }
        }

        None
    }

    fn substr(&self, start: usize, end: usize) -> OsString {
        OsString::from_vec(self.as_bytes()[start..end].into())
    }
}
