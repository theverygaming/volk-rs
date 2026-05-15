use std::mem;
use std::ptr::NonNull;

pub struct AlignedVec<T: Sized> {
    ptr: NonNull<T>,
    length: usize,
}

impl<T: Sized> AlignedVec<T> {
    fn new(n: usize) -> Self {
        assert!(mem::size_of::<T>() != 0, "cannot handle zero-sized type");
        assert!(n != 0, "cannot allocate vector with capacity zero");
        AlignedVec {
            ptr: match NonNull::new(unsafe {
                let ptr =
                    volk_sys::volk_malloc(mem::size_of::<T>() * n, volk_sys::volk_get_alignment());
                ptr
            } as *mut T)
            {
                Some(p) => p,
                None => panic!("volk_malloc failed"),
            },
            length: n,
        }
    }

    // SAFETY: this thing is completely unsafe lmfao
    // BUG: // FIXME:
    pub fn new_zeroed(n: usize) -> Self {
        let v = Self::new(n);
        unsafe {
            std::ptr::write_bytes(v.ptr.as_ptr(), 0, n);
        }
        v
    }

    pub fn as_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { self.ptr.as_mut() }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T: Sized + Clone> AlignedVec<T> {
    pub fn from_elem(elem: T, n: usize) -> Self {
        let v = Self::new(n);
        for i in 0..n {
            unsafe {
                std::ptr::write(v.ptr.as_ptr().offset(i.try_into().unwrap()), elem.clone());
            }
        }
        v
    }
}

impl<T: Sized + Clone + Default> AlignedVec<T> {
    pub fn new_default(n: usize) -> Self {
        Self::from_elem(Default::default(), n)
    }
}

impl<T: Sized> Drop for AlignedVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(
                self.ptr.as_ptr(),
                self.length,
            ));
            volk_sys::volk_free(self.ptr.as_ptr() as *mut std::os::raw::c_void);
        }
    }
}

// SAFETY: should be fine to implement these long as T also has them,
// volk_malloc and volk_free should be thread safe
unsafe impl<T: Send> Send for AlignedVec<T> where T: Send {}
unsafe impl<T: Send> Sync for AlignedVec<T> where T: Sync {}

impl<T: Sized> std::ops::Deref for AlignedVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.length) }
    }
}

impl<T: Sized> std::ops::DerefMut for AlignedVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.length) }
    }
}

impl<T: Sized + Copy> Clone for AlignedVec<T> {
    fn clone(&self) -> Self {
        let mut new = Self::new(self.length);
        new.copy_from_slice(self);
        new
    }
}
