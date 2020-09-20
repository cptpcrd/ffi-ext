use std::ffi::{OsStr, OsString};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

pub trait OsStrExt2 {
    fn starts_with(&self, prefix: &OsStr) -> bool;
    fn ends_with(&self, suffix: &OsStr) -> bool;

    fn find_all<'a>(&'a self, needle: &'a OsStr) -> OsStrFindIter;

    fn rfind(&self, needle: &OsStr) -> Option<usize>;
    fn find(&self, needle: &OsStr) -> Option<usize>;

    fn substr(&self, start: usize, end: usize) -> OsString;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_with() {
        assert!(OsStr::new("abc").starts_with(OsStr::new("abc")));
        assert!(OsStr::new("abc").starts_with(OsStr::new("ab")));
        assert!(OsStr::new("abc").starts_with(OsStr::new("")));

        assert!(!OsStr::new("abc").starts_with(OsStr::new("abcd")));
        assert!(!OsStr::new("abc").starts_with(OsStr::new("bc")));

        assert!(!OsStr::new("").starts_with(OsStr::new("abc")));
        assert!(!OsStr::new("").starts_with(OsStr::new("ab")));
        assert!(OsStr::new("").starts_with(OsStr::new("")));
    }

    #[test]
    fn test_ends_with() {
        assert!(OsStr::new("abc").ends_with(OsStr::new("abc")));
        assert!(OsStr::new("abc").ends_with(OsStr::new("bc")));
        assert!(OsStr::new("abc").ends_with(OsStr::new("")));

        assert!(!OsStr::new("abc").ends_with(OsStr::new("abcd")));
        assert!(!OsStr::new("abc").ends_with(OsStr::new("ab")));

        assert!(!OsStr::new("").ends_with(OsStr::new("abc")));
        assert!(!OsStr::new("").ends_with(OsStr::new("ab")));
        assert!(OsStr::new("").ends_with(OsStr::new("")));
    }

    #[test]
    fn test_find() {
        assert_eq!(OsStr::new("abcabc").find(OsStr::new("abc")), Some(0));
        assert_eq!(OsStr::new("abcabc").find(OsStr::new("ab")), Some(0));
        assert_eq!(OsStr::new("abc").find(OsStr::new("bc")), Some(1));
        assert_eq!(OsStr::new("abcabc").find(OsStr::new("c")), Some(2));

        assert_eq!(OsStr::new("abc").find(OsStr::new("abcd")), None);
        assert_eq!(OsStr::new("abc").find(OsStr::new("abcde")), None);
        assert_eq!(OsStr::new("abc").find(OsStr::new("abcdefghi")), None);
        assert_eq!(OsStr::new("abc").find(OsStr::new("d")), None);

        assert_eq!(OsStr::new("abc").find(OsStr::new("")), Some(0));
        assert_eq!(OsStr::new("").find(OsStr::new("")), Some(0));
        assert_eq!(OsStr::new("").find(OsStr::new("a")), None);
        assert_eq!(OsStr::new("").find(OsStr::new("ab")), None);
        assert_eq!(OsStr::new("").find(OsStr::new("abc")), None);

        assert_eq!(
            OsStr::new("abcdefghijklabce").find(OsStr::new("abcd")),
            Some(0)
        );
        assert_eq!(
            OsStr::new("abcdefghijklabce").find(OsStr::new("abc")),
            Some(0)
        );
        assert_eq!(
            OsStr::new("abcdefghijklabce").find(OsStr::new("abce")),
            Some(12)
        );
        assert_eq!(
            OsStr::new("abcdefghijklabce").find(OsStr::new("abcf")),
            None
        );
    }

    #[test]
    fn test_rfind() {
        assert_eq!(OsStr::new("abcabc").rfind(OsStr::new("abc")), Some(3));
        assert_eq!(OsStr::new("abcabc").rfind(OsStr::new("ab")), Some(3));
        assert_eq!(OsStr::new("abc").rfind(OsStr::new("bc")), Some(1));
        assert_eq!(OsStr::new("abcabc").rfind(OsStr::new("c")), Some(5));

        assert_eq!(OsStr::new("abc").rfind(OsStr::new("abcd")), None);
        assert_eq!(OsStr::new("abc").rfind(OsStr::new("abcde")), None);
        assert_eq!(OsStr::new("abc").rfind(OsStr::new("abcdefghi")), None);
        assert_eq!(OsStr::new("abc").rfind(OsStr::new("d")), None);

        assert_eq!(OsStr::new("abc").rfind(OsStr::new("")), Some(3));
        assert_eq!(OsStr::new("").rfind(OsStr::new("")), Some(0));
        assert_eq!(OsStr::new("").rfind(OsStr::new("a")), None);
        assert_eq!(OsStr::new("").rfind(OsStr::new("ab")), None);
        assert_eq!(OsStr::new("").rfind(OsStr::new("abc")), None);

        assert_eq!(
            OsStr::new("abcdefghijklabce").rfind(OsStr::new("abcd")),
            Some(0)
        );
        assert_eq!(
            OsStr::new("abcdefghijklabce").rfind(OsStr::new("abc")),
            Some(12)
        );
        assert_eq!(
            OsStr::new("abcdefghijklabce").rfind(OsStr::new("abce")),
            Some(12)
        );
        assert_eq!(
            OsStr::new("abcdefghijklabce").find(OsStr::new("abcf")),
            None
        );
    }

    #[test]
    fn test_substr() {
        assert_eq!(OsStr::new("abc").substr(0, 3), OsStr::new("abc"));
        assert_eq!(OsStr::new("abc").substr(0, 2), OsStr::new("ab"));
        assert_eq!(OsStr::new("abc").substr(1, 2), OsStr::new("b"));
        assert_eq!(OsStr::new("abc").substr(1, 3), OsStr::new("bc"));

        assert_eq!(OsStr::new("abc").substr(0, 0), OsStr::new(""));
        assert_eq!(OsStr::new("abc").substr(3, 3), OsStr::new(""));
    }

    fn collect_fused<T, I: Iterator<Item = T>>(mut it: I) -> Vec<T> {
        let res = it.by_ref().collect();
        // Check that it ends after None
        assert!(it.next().is_none());
        res
    }

    #[test]
    fn test_find_all() {
        assert_eq!(
            collect_fused(OsStr::new("abccbaab").find_all(OsStr::new("abc"))),
            [0],
        );
        assert_eq!(
            collect_fused(OsStr::new("abccbaabc").find_all(OsStr::new("abc"))),
            [0, 6],
        );

        assert_eq!(
            collect_fused(OsStr::new("").find_all(OsStr::new("abc"))),
            []
        );
        assert_eq!(collect_fused(OsStr::new("").find_all(OsStr::new("ab"))), []);
        assert_eq!(collect_fused(OsStr::new("").find_all(OsStr::new("a"))), []);
        assert_eq!(collect_fused(OsStr::new("").find_all(OsStr::new(""))), [0]);

        assert_eq!(
            collect_fused(OsStr::new("a").find_all(OsStr::new(""))),
            [0, 1]
        );
        assert_eq!(
            collect_fused(OsStr::new("a").find_all(OsStr::new("a"))),
            [0]
        );
        assert_eq!(
            collect_fused(OsStr::new("a").find_all(OsStr::new("ab"))),
            []
        );

        assert_eq!(
            collect_fused(OsStr::new("abccbaabc").find_all(OsStr::new(""))),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        );

        assert_eq!(
            collect_fused(OsStr::new("abccbaabc").find_all(OsStr::new("a"))),
            [0, 5, 6],
        );
        assert_eq!(
            collect_fused(OsStr::new("abccbaabc").find_all(OsStr::new("b"))),
            [1, 4, 7],
        );
        assert_eq!(
            collect_fused(OsStr::new("abccbaabc").find_all(OsStr::new("c"))),
            [2, 3, 8],
        );
    }

    #[test]
    fn test_find_all_rev() {
        assert_eq!(
            collect_fused(OsStr::new("abccbaab").find_all(OsStr::new("abc")).rev()),
            [0],
        );
        assert_eq!(
            collect_fused(OsStr::new("abccbaabc").find_all(OsStr::new("abc")).rev()),
            [6, 0],
        );

        assert_eq!(
            collect_fused(OsStr::new("").find_all(OsStr::new("abc")).rev()),
            []
        );
        assert_eq!(
            collect_fused(OsStr::new("").find_all(OsStr::new("ab")).rev()),
            []
        );
        assert_eq!(
            collect_fused(OsStr::new("").find_all(OsStr::new("a")).rev()),
            []
        );
        assert_eq!(
            collect_fused(OsStr::new("").find_all(OsStr::new("")).rev()),
            [0]
        );

        assert_eq!(
            collect_fused(OsStr::new("a").find_all(OsStr::new("")).rev()),
            [1, 0]
        );
        assert_eq!(
            collect_fused(OsStr::new("a").find_all(OsStr::new("a")).rev()),
            [0]
        );
        assert_eq!(
            collect_fused(OsStr::new("a").find_all(OsStr::new("ab")).rev()),
            []
        );

        assert_eq!(
            collect_fused(OsStr::new("abccbaabc").find_all(OsStr::new("")).rev()),
            [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        );

        assert_eq!(
            collect_fused(OsStr::new("abccbaabc").find_all(OsStr::new("a")).rev()),
            [6, 5, 0],
        );
        assert_eq!(
            collect_fused(OsStr::new("abccbaabc").find_all(OsStr::new("b")).rev()),
            [7, 4, 1],
        );
        assert_eq!(
            collect_fused(OsStr::new("abccbaabc").find_all(OsStr::new("c")).rev()),
            [8, 3, 2],
        );
    }

    #[test]
    fn test_find_all_both() {
        let mut it = OsStr::new("abcbabcdabc").find_all(OsStr::new("abc"));

        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next_back(), Some(8));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);
    }
}
