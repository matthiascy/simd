use crate::fmt_as_simd;

/// 256-bit wide SIMD data type.
#[repr(C, align(32))]
pub union Ymm {
    pub int8: [i8; 32],
    pub int16: [i16; 16],
    pub int32: [i32; 8],
    pub int64: [i64; 4],
    pub uint8: [u8; 32],
    pub uint16: [u16; 16],
    pub uint32: [u32; 8],
    pub uint64: [u64; 4],
    pub float: [f32; 8],
    pub double: [f64; 4],
}

impl Ymm {
    #[inline(always)]
    pub fn as_ptr(&self) -> *const Ymm {
        self as *const Ymm
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut Ymm {
        self as *mut Ymm
    }

    pub fn fmt_i16(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.int16, std::mem::size_of::<Ymm>() / std::mem::size_of::<i16>(), 8).unwrap();
        }
        s
    }

    pub fn fmt_u16(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.uint16, std::mem::size_of::<Ymm>() / std::mem::size_of::<u16>(), 8).unwrap();
        }
        s
    }

    pub fn fmt_i8(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.int8, std::mem::size_of::<Ymm>() / std::mem::size_of::<i8>(), 4).unwrap();
        }
        s
    }

    pub fn fmt_u8(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.uint8, std::mem::size_of::<Ymm>() / std::mem::size_of::<u8>(), 4).unwrap();
        }
        s
    }

    pub fn fmt_i32(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.int32, std::mem::size_of::<Ymm>() / std::mem::size_of::<i32>(), 12).unwrap();
        }
        s
    }

    pub fn fmt_u32(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.uint32, std::mem::size_of::<Ymm>() / std::mem::size_of::<u32>(), 12).unwrap();
        }
        s
    }

    pub fn fmt_f32(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.float, std::mem::size_of::<Ymm>() / std::mem::size_of::<f32>(), 16).unwrap();
        }
        s
    }

    pub fn fmt_i64(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.int64, std::mem::size_of::<Ymm>() / std::mem::size_of::<i64>(), 24).unwrap();
        }
        s
    }

    pub fn fmt_u64(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.uint64, std::mem::size_of::<Ymm>() / std::mem::size_of::<u64>(), 24).unwrap();
        }
        s
    }

    pub fn fmt_f64(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.double, std::mem::size_of::<Ymm>() / std::mem::size_of::<f64>(), 32).unwrap();
        }
        s
    }
}