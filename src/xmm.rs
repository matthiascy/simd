use crate::{fmt_as_simd, fmt_as_simd_hex};

/// 128-bit wide SIMD data type.
#[repr(C, align(16))]
pub union Xmm {
    pub int8: [i8; 16],
    pub int16: [i16; 8],
    pub int32: [i32; 4],
    pub int64: [i64; 2],
    pub uint8: [u8; 16],
    pub uint16: [u16; 8],
    pub uint32: [u32; 4],
    pub uint64: [u64; 2],
    pub float32: [f32; 4],
    pub float64: [f64; 2],
}

impl Xmm {
    pub fn fmt_i16(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.int16, std::mem::size_of::<Xmm>() / std::mem::size_of::<i16>(), 8).unwrap();
        }
        s
    }

    pub fn fmt_u16(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.uint16, std::mem::size_of::<Xmm>() / std::mem::size_of::<u16>(), 8).unwrap();
        }
        s
    }

    pub fn fmt_u16hex(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd_hex(&mut s, &self.uint16, std::mem::size_of::<Xmm>() / std::mem::size_of::<u16>(), 6).unwrap();
        }
        s
    }

    pub fn fmt_i8(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.int8, std::mem::size_of::<Xmm>() / std::mem::size_of::<i8>(), 4).unwrap();
        }
        s
    }

    pub fn fmt_u8(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.uint8, std::mem::size_of::<Xmm>() / std::mem::size_of::<u8>(), 4).unwrap();
        }
        s
    }

    pub fn fmt_i32(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.int32, std::mem::size_of::<Xmm>() / std::mem::size_of::<i32>(), 12).unwrap();
        }
        s
    }

    pub fn fmt_u32(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.uint32, std::mem::size_of::<Xmm>() / std::mem::size_of::<u32>(), 12).unwrap();
        }
        s
    }

    pub fn fmt_i64(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.int64, std::mem::size_of::<Xmm>() / std::mem::size_of::<i64>(), 20).unwrap();
        }
        s
    }

    pub fn fmt_u64(&self) -> String {
        let mut s = String::new();
        unsafe {
            fmt_as_simd(&mut s, &self.uint64, std::mem::size_of::<Xmm>() / std::mem::size_of::<u64>(), 20).unwrap();
        }
        s
    }
}
