use std::alloc::{Layout, alloc_zeroed, dealloc, handle_alloc_error};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};
use std::slice;

pub const KB: usize = 1024;
pub const MB: usize = 1024 * KB;
pub const GB: usize = 1024 * MB;

/// # Safety
/// All-zero bit pattern must be a valid value of `Self`.
pub unsafe trait Zeroable {}

unsafe impl Zeroable for u8 {}
unsafe impl Zeroable for u16 {}
unsafe impl Zeroable for u32 {}
unsafe impl Zeroable for u64 {}
unsafe impl Zeroable for u128 {}
unsafe impl Zeroable for usize {}
unsafe impl Zeroable for i8 {}
unsafe impl Zeroable for i16 {}
unsafe impl Zeroable for i32 {}
unsafe impl Zeroable for i64 {}
unsafe impl Zeroable for i128 {}
unsafe impl Zeroable for isize {}
unsafe impl Zeroable for f32 {}
unsafe impl Zeroable for f64 {}

pub struct AlignedBuffer<T> {
    ptr: NonNull<T>,
    len: usize,
    layout: Layout,
    _marker: PhantomData<T>,
}

impl<T: Zeroable> AlignedBuffer<T> {
    #[inline]
    pub fn new(len: usize, align: usize) -> Self {
        assert!(len > 0, "len must be > 0");
        assert!(align.is_power_of_two(), "align must be power of two");
        assert!(align >= align_of::<T>(), "align too small for T");

        let layout = Layout::array::<T>(len)
            .expect("size overflow")
            .align_to(align)
            .expect("invalid alignment")
            .pad_to_align();

        // SAFETY: layout has nonzero size; T: Zeroable makes all-zeros a valid T.
        let raw = unsafe { alloc_zeroed(layout) };
        let ptr = NonNull::new(raw.cast()).unwrap_or_else(|| handle_alloc_error(layout));

        Self { ptr, len, layout, _marker: PhantomData }
    }
}

impl<T> AlignedBuffer<T> {
    #[inline]
    pub fn byte_len(&self) -> usize {
        self.len * size_of::<T>()
    }
}

impl<T> Deref for AlignedBuffer<T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &[T] {
        // SAFETY: own a valid contiguous allocation.
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for AlignedBuffer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        // SAFETY: &mut self ensures unique access.
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> Drop for AlignedBuffer<T> {
    fn drop(&mut self) {
        if std::mem::needs_drop::<T>() {
            // SAFETY: drop initialized elements before freeing.
            unsafe {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            }
        }
        // SAFETY: layout matches alloc_zeroed.
        unsafe { dealloc(self.ptr.as_ptr().cast(), self.layout) };
    }
}

unsafe impl<T: Send> Send for AlignedBuffer<T> {}
unsafe impl<T: Sync> Sync for AlignedBuffer<T> {}
