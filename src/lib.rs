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
}
