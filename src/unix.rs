use std::ffi::{OsStr, OsString};
use std::iter::FusedIterator;
use std::os::unix::ffi::*;

use crate::OsStrExt2;

pub struct OsStrFindIter<'a> {
    haystack: &'a [u8],
    needle: &'a [u8],
    left: usize,
    right: usize,
}

impl<'a> OsStrFindIter<'a> {
    fn new(haystack: &'a [u8], needle: &'a [u8]) -> Self {
        let (left, right) = if let Some(diff) = haystack.len().checked_sub(needle.len()) {
            // Add 1 to the right bound; this allow matching on the very last element
            (0, diff + 1)
        } else {
            // Needle is longer than haystack -> force immediate failure
            (1, 0)
        };

        Self {
            haystack,
            needle,
            left,
            right,
        }
    }
}

impl Iterator for OsStrFindIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.left >= self.right {
            return None;
        } else if self.needle.is_empty() {
            // An empty needle matches the whole way through
            let index = self.left;
            self.left += 1;
            return Some(index);
        }

        // Delegate to twoway
        #[cfg(feature = "twoway")]
        if let Some(mut index) = twoway::find_bytes(
            &self.haystack[self.left..self.right + self.needle.len() - 1],
            self.needle,
        ) {
            // Adjust the index
            index += self.left;
            // Update for next time
            self.left = index + 1;
            return Some(index);
        }

        // Use memchr to find the first character, then make sure that the rest match
        #[cfg(all(not(feature = "twoway"), feature = "memchr"))]
        while let Some(mut index) =
            memchr::memchr(self.needle[0], &self.haystack[self.left..self.right])
        {
            // Adjust the index
            index += self.left;
            // Update for next time
            self.left = index + 1;

            if self.needle.len() == 1
                || self.haystack[index + 1..index + self.needle.len()] == self.needle[1..]
            {
                // We found a match!
                return Some(index);
            }
        }

        // Naive algorithm
        #[cfg(not(any(feature = "twoway", feature = "memchr")))]
        for index in self.left..self.right {
            if &self.haystack[index..index + self.needle.len()] == self.needle {
                self.left = index + 1;
                return Some(index);
            }
        }

        // Force immediate return next time
        self.left = self.right + 1;
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.right.saturating_sub(self.left)))
    }
}

impl DoubleEndedIterator for OsStrFindIter<'_> {
    fn next_back(&mut self) -> Option<usize> {
        if self.left >= self.right {
            return None;
        } else if self.needle.is_empty() {
            // An empty needle matches the whole way through
            self.right -= 1;
            return Some(self.right);
        }

        // Delegate to twoway
        #[cfg(feature = "twoway")]
        if let Some(mut index) = twoway::rfind_bytes(
            &self.haystack[self.left..self.right + self.needle.len() - 1],
            self.needle,
        ) {
            // Adjust the index
            index += self.left;

            // Update for next time
            self.right = index;

            return Some(index);
        }

        // Use memchr to find the first character, then make sure that the rest match
        #[cfg(all(not(feature = "twoway"), feature = "memchr"))]
        while let Some(mut index) =
            memchr::memrchr(self.needle[0], &self.haystack[self.left..self.right])
        {
            // Adjust the index
            index += self.left;

            // Update for next time
            self.right = index;

            if self.needle.len() == 1
                || self.haystack[index + 1..index + self.needle.len()] == self.needle[1..]
            {
                // We found a match!
                return Some(index);
            }
        }

        // Naive algorithm
        #[cfg(not(any(feature = "twoway", feature = "memchr")))]
        for index in (self.left..self.right).rev() {
            if &self.haystack[index..index + self.needle.len()] == self.needle {
                self.right = index;
                return Some(index);
            }
        }

        // Force immediate return next time
        self.left = self.right + 1;
        None
    }
}

impl FusedIterator for OsStrFindIter<'_> {}

impl OsStrExt2 for OsStr {
    fn starts_with(&self, prefix: &OsStr) -> bool {
        self.as_bytes().starts_with(prefix.as_bytes())
    }

    fn ends_with(&self, suffix: &OsStr) -> bool {
        self.as_bytes().ends_with(suffix.as_bytes())
    }

    #[inline]
    fn find_all<'a>(&'a self, needle: &'a OsStr) -> OsStrFindIter {
        OsStrFindIter::new(self.as_bytes(), needle.as_bytes())
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
        let indices = match needle.len() {
            0 => return Some(0),
            1 => {
                let search_ch = needle[0];
                return haystack.iter().position(|&ch| ch == search_ch);
            }
            len => (0..=(haystack.len().checked_sub(len)?)),
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
        let indices = match needle.len() {
            0 => return Some(haystack.len()),
            1 => {
                let search_ch = needle[0];
                return haystack.iter().rposition(|&ch| ch == search_ch);
            }
            len => (0..=(haystack.len().checked_sub(len)?)).rev(),
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
