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

    fn find_substr(&self, substr: &OsStr) -> Option<usize> {
        if substr.is_empty() {
            return Some(0);
        } else if self.is_empty() {
            return None;
        }

        let bytes = self.as_bytes();
        let substr_bytes = substr.as_bytes();

        let sub_len = substr_bytes.len();

        for i in 0..=(bytes.len().checked_sub(sub_len)?) {
            if &bytes[i..i + sub_len] == substr_bytes {
                return Some(i);
            }
        }

        None
    }

    fn rfind_substr(&self, substr: &OsStr) -> Option<usize> {
        if substr.is_empty() {
            return Some(self.as_bytes().len());
        } else if self.is_empty() {
            return None;
        }

        let bytes = self.as_bytes();
        let substr_bytes = substr.as_bytes();

        let sub_len = substr_bytes.len();

        for i in (0..=(bytes.len().checked_sub(sub_len)?)).rev() {
            if &bytes[i..i + sub_len] == substr_bytes {
                return Some(i);
            }
        }

        None
    }

    fn substr(&self, start: usize, end: usize) -> OsString {
        OsString::from_vec(self.as_bytes()[start..end].into())
    }
}
