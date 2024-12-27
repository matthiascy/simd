use std::mem::needs_drop;
use std::ptr::{NonNull};
use rand::distributions::Distribution;
use std::alloc;

/// Allocates an array of type T with a given length and alignment.
pub struct Array<T> {
    data: NonNull<T>,
    len: usize,
    layout: alloc::Layout,
    _marker: std::marker::PhantomData<[T]>,
}

unsafe impl<T> Send for Array<T> {}
unsafe impl<T> Sync for Array<T> {}

impl<T> Array<T> {
    pub fn new(len: usize, align: usize) -> Self {
        let layout = alloc::Layout::from_size_align(std::mem::size_of::<T>() * len, align).unwrap();
        let data = unsafe { alloc::alloc(layout) };

        Self {
            data: NonNull::new(data as *mut T).unwrap(),
            len,
            layout,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn is_aligned(&self, align: usize) -> bool {
        self.data.as_ptr() as usize % align == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn fill(&mut self, value: T)
    where T: Clone
    {
        for i in 0..self.len {
            unsafe { std::ptr::write(self.data.as_ptr().add(i), value.clone()) };
        }
    }

    pub fn randomise(&mut self, min: T, max: T, exclude_zero: bool)
    where T: rand::distributions::uniform::SampleUniform + PartialOrd + Copy + Default
    {
        let mut rng = rand::thread_rng();
        let uniform = rand::distributions::Uniform::new(min, max);
        for i in 0..self.len {
            let mut value = uniform.sample(&mut rng);
            if exclude_zero {
                while value == T::default() {
                    value = uniform.sample(&mut rng);
                }
            }
            unsafe { std::ptr::write(self.data.as_ptr().add(i), value) };
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data.as_ptr(), self.len) }
    }

    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_ptr()
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        if needs_drop::<T>() {
            for i in 0..self.len {
                unsafe { std::ptr::drop_in_place(self.data.as_ptr().add(i)) };
            }
        }
        unsafe { alloc::dealloc(self.data.as_ptr() as *mut u8, self.layout) };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array_copyable_element_dropping() {
        let mut array = Array::<u8>::new(16, 16);
        array.fill(0);
        assert_eq!(array.as_slice(), &[0; 16]);

        array.randomise(0, 255, false);
        let slice = array.as_slice();
    }

    #[test]
    fn test_array_cloneable_element_dropping() {
        let mut array = Array::<String>::new(16, 16);
        array.fill("a".to_string());

        let slice = array.as_slice();
        for val in slice {
            assert_eq!(val, "a");
        }
    }

    #[test]
    fn test_array_referenced_element_dropping() {
        let mut array = Array::<String>::new(16, 16);
        array.fill("a".to_string());

        let slice = array.as_slice();
        for val in slice {
            assert_eq!(val, "a");
        }
    }
}