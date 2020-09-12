use std::ffi::CStr;
use std::ops::Deref;

unsafe fn strlen(ptr: *const u8) -> usize {
    for i in 0.. {
        if *ptr.add(i) == 0 {
            return i;
        }
    }

    unreachable!();
}

/// A type representing a C string allocated using `malloc()`.
///
/// This may be useful if you want to interact with C strings returned by foreign libraries.
pub struct MallocCString {
    ptr: *mut u8,
    len: usize,
}

impl MallocCString {
    /// Create a new `MallocCString` and copy in the given data.
    ///
    /// If the data contains an interior 0 byte, this function will fail and return its position.
    #[inline]
    pub fn new<T: AsRef<[u8]>>(bytes: T) -> Result<Self, usize> {
        Self::create(bytes.as_ref())
    }

    fn create(bytes: &[u8]) -> Result<Self, usize> {
        let mut res = Self {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
        unsafe {
            res.reallocate(bytes.len() + 1);
        }

        unsafe {
            for (i, &b) in bytes.iter().enumerate() {
                if b == 0 {
                    return Err(i);
                }
                *res.ptr.add(i) = b;
            }

            // Nul-terminate
            *res.ptr.add(bytes.len()) = 0;
        }

        res.len = bytes.len();

        Ok(res)
    }

    /// # Safety:
    ///
    /// 1. `self.ptr` must be either NULL or a correctly allocated pointer
    /// 2. `new_size` must be > 0 and < `isize::MAX`
    unsafe fn reallocate(&mut self, new_size: usize) {
        self.ptr = libc::realloc(self.ptr as *mut libc::c_void, new_size) as *mut u8;

        if self.ptr.is_null() {
            let layout =
                std::alloc::Layout::from_size_align(new_size, std::mem::size_of::<*const u8>() * 8)
                    .unwrap();
            std::alloc::handle_alloc_error(layout);
        }
    }

    /// Return the contents of this `MallocCString` as a slice of bytes **without** the trailing
    /// NUL.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }

    /// Return the contents of this `MallocCString` as a slice of bytes **with** the trailing NUL.
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len + 1) }
    }

    /// Extract a `CStr` slice containing the entire string.
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.ptr as *const libc::c_char) }
    }

    /// Take ownership of a string allocated with `malloc()`.
    ///
    /// # Safety
    ///
    /// This should only be called with a pointer that was either a) obtained by [`into_raw`] or b)
    /// allocated using the C `malloc()` function.
    ///
    /// In addition, the string MUST be nul-terminated.
    ///
    /// [`into_raw`]: ./fn.into_raw.html
    pub unsafe fn from_raw(ptr: *mut libc::c_char) -> Self {
        let ptr = ptr as *mut u8;

        let len = strlen(ptr);

        Self { ptr, len }
    }

    /// Consume the `MallocCString` and transfer ownership to the caller.
    ///
    /// To deallocate the pointer (and avoid memory leaks), either a) pass this pointer to
    /// [`from_raw`] or b) use the C `free()` function.
    ///
    /// [`from_raw`]: ./fn.from_raw.html
    pub fn into_raw(self) -> *mut libc::c_char {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr as *mut libc::c_char
    }
}

impl AsRef<CStr> for MallocCString {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl Deref for MallocCString {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl Drop for MallocCString {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            libc::free(self.ptr as *mut libc::c_void);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_as_bytes() {
        let mut s = MallocCString::new(b"").unwrap();
        assert_eq!(s.as_bytes(), b"");
        assert_eq!(s.as_bytes_with_nul(), b"\0");

        s = MallocCString::new(b"abc").unwrap();
        assert_eq!(s.as_bytes(), b"abc");
        assert_eq!(s.as_bytes_with_nul(), b"abc\0");
    }

    #[test]
    fn test_create_nul() {
        assert!(MallocCString::new(b"\0").is_err());
        assert!(MallocCString::new(b"abc\0").is_err());
        assert!(MallocCString::new(b"abc\0def").is_err());
    }

    #[test]
    fn test_cstr() {
        let s = MallocCString::new(b"abc").unwrap();

        assert_eq!(s.as_c_str(), s.as_ref());
        assert_eq!(s.as_c_str(), &*s);

        assert_eq!(s.as_c_str().as_ptr(), s.as_ptr());
    }

    #[test]
    fn test_raw() {
        let s = MallocCString::new(b"abc").unwrap();
        assert_eq!(s.as_bytes(), b"abc");
        assert_eq!(s.as_bytes_with_nul(), b"abc\0");

        let ptr = s.into_raw();
        assert_eq!(
            unsafe { std::slice::from_raw_parts(ptr as *const u8, 4) },
            b"abc\0"
        );

        let s = unsafe { MallocCString::from_raw(ptr) };
        assert_eq!(s.as_bytes(), b"abc");
        assert_eq!(s.as_bytes_with_nul(), b"abc\0");
    }
}
