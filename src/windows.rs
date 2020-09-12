use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::*;

use crate::OsStrExt2;

impl OsStrExt2 for OsStr {
    fn starts_with(&self, prefix: &OsStr) -> bool {
        if prefix.is_empty() {
            return true;
        } else if self.is_empty() {
            return false;
        }

        let mut self_it = self.encode_wide();
        let mut prefix_it = prefix.encode_wide();

        loop {
            let prefix_ch = match prefix_it.next() {
                Some(ch) => ch,
                // End of the prefix -> success
                None => return true,
            };

            match self_it.next() {
                // We're not at the end of `self`
                Some(self_ch) => {
                    if self_ch != prefix_ch {
                        // Mismatch -> failure
                        return false;
                    }
                }

                // We've hit the end of `self`, but not the end of the prefix.
                None => return false,
            }
        }
    }

    fn ends_with(&self, suffix: &OsStr) -> bool {
        if suffix.is_empty() {
            return true;
        } else if self.is_empty() {
            return false;
        }

        // Collect the unicode points in the suffix
        let suffix_seq: Vec<u16> = suffix.encode_wide().collect();

        // Now collect the unicode points in self, but only keep the last
        // <however many the suffix has>.
        let mut self_seq = VecDeque::with_capacity(suffix_seq.len());
        for self_ch in self.encode_wide() {
            if self_seq.len() >= suffix_seq.len() {
                self_seq.pop_front();
            }
            self_seq.push_back(self_ch);
        }

        // Compare the two
        self_seq == suffix_seq
    }

    fn find_substr(&self, substr: &OsStr) -> Option<usize> {
        if substr.is_empty() {
            return Some(0);
        } else if self.is_empty() {
            return None;
        }

        // Collect the unicode points in the search string
        let substr_seq: Vec<u16> = substr.encode_wide().collect();

        let mut self_seq = VecDeque::with_capacity(substr_seq.len());
        for (i, self_ch) in self.encode_wide().enumerate() {
            // Collect the unicode points in self, but only keep the last
            // <however many the search string has>.
            if self_seq.len() >= substr_seq.len() {
                self_seq.pop_front();
            }
            self_seq.push_back(self_ch);

            // Found a match; return
            if self_seq == substr_seq {
                return Some(i + 1 - self_seq.len());
            }
        }

        None
    }

    fn rfind_substr(&self, substr: &OsStr) -> Option<usize> {
        if substr.is_empty() {
            return Some(self.encode_wide().count());
        } else if self.is_empty() {
            return None;
        }

        let mut res = None;

        // Collect the unicode points in the search string
        let substr_seq: Vec<u16> = substr.encode_wide().collect();

        let mut self_seq = VecDeque::with_capacity(substr_seq.len());
        for (i, self_ch) in self.encode_wide().enumerate() {
            // Collect the unicode points in self, but only keep the last
            // <however many the search string has>.
            if self_seq.len() >= substr_seq.len() {
                self_seq.pop_front();
            }
            self_seq.push_back(self_ch);

            // Found a match; store it until the end
            if self_seq == substr_seq {
                res = Some(i + 1 - self_seq.len());
            }
        }

        res
    }

    fn substr(&self, start: usize, end: usize) -> OsString {
        OsString::from_wide(
            &self
                .encode_wide()
                .skip(start)
                .take(end - start)
                .collect::<Vec<u16>>(),
        )
    }
}
