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
    fn find(&self, needle: &OsStr) -> Option<usize> {
        twoway::find_bytes(self.as_bytes(), needle.as_bytes())
    }

    #[cfg(not(feature = "twoway"))]
    fn find(&self, needle: &OsStr) -> Option<usize> {
        let haystack = self.as_bytes();
        let needle = needle.as_bytes();

        #[cfg(feature = "memchr")]
        let indices = match needle.len() {
            0 => return Some(0),
            1 => return memchr::memchr(needle[0], haystack),
            len => {
                memchr::memchr_iter(needle[0], &haystack[..haystack.len().checked_sub(len)? + 1])
            }
        };

        #[cfg(not(feature = "memchr"))]
        let indices = {
            if needle.is_empty() {
                return Some(0);
            }
            0..=(haystack.len().checked_sub(needle.len())?)
        };

        for i in indices {
            if &haystack[i..i + needle.len()] == needle {
                return Some(i);
            }
        }

        None
    }

    #[cfg(feature = "twoway")]
    #[inline]
    fn rfind(&self, needle: &OsStr) -> Option<usize> {
        twoway::rfind_bytes(self.as_bytes(), needle.as_bytes())
    }

    #[cfg(not(feature = "twoway"))]
    fn rfind(&self, needle: &OsStr) -> Option<usize> {
        let haystack = self.as_bytes();
        let needle = needle.as_bytes();

        #[cfg(feature = "memchr")]
        let indices = match needle.len() {
            0 => return Some(haystack.len()),
            1 => return memchr::memrchr(needle[0], haystack),
            len => {
                memchr::memrchr_iter(needle[0], &haystack[..haystack.len().checked_sub(len)? + 1])
            }
        };

        #[cfg(not(feature = "memchr"))]
        let indices = {
            if needle.is_empty() {
                return Some(haystack.len());
            }
            (0..=(haystack.len().checked_sub(needle.len())?)).rev()
        };

        for i in indices {
            if &haystack[i..i + needle.len()] == needle {
                return Some(i);
            }
        }

        None
    }

    fn substr(&self, start: usize, end: usize) -> OsString {
        OsString::from_vec(self.as_bytes()[start..end].into())
    }
}
