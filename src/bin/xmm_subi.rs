//! _mm_sub_epi[8/16/32/64]: subtracts packed 32-bit integers (**signed**) in a and b (wraparound)
//! _mm_subs_epi[8/16]: subtracts packed 16-bit **signed** integers in a and b (saturated)
//! _mm_subs_epu[8/16]: subtracts packed unsigned 16-bit **unsigned** integers in a and b (saturated)

use simd::xmm::*;

pub fn sub_i32_sse2(a: &Xmm, b: &Xmm, wrapped: &mut Xmm) {
    use std::arch::x86_64::{
        __m128i, _mm_load_si128, _mm_store_si128, _mm_sub_epi32,
    };
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const __m128i);
        let b_val = _mm_load_si128(b as *const Xmm as *const __m128i);
        _mm_store_si128(wrapped as *mut Xmm as *mut __m128i, _mm_sub_epi32(a_val, b_val));
    }
}

pub fn sub_i64_sse2(a: &Xmm, b: &Xmm, wrapped: &mut Xmm) {
    use std::arch::x86_64::{
        __m128i, _mm_load_si128, _mm_store_si128, _mm_sub_epi64,
    };
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const __m128i);
        let b_val = _mm_load_si128(b as *const Xmm as *const __m128i);
        _mm_store_si128(wrapped as *mut Xmm as *mut __m128i, _mm_sub_epi64(a_val, b_val));
    }
}

fn main() {
    let ai32 = Xmm {
        int32: [1_000_000, 200, -30, 40_000_000],
    };
    let bi32 = Xmm {
        int32: [100, -200, 30_000, 5_000],
    };
    let mut wrapped = Xmm {
        int32: [0; 4],
    };

    sub_i32_sse2(&ai32, &bi32, &mut wrapped);

    println!("Subtraction i32[{},{}] sse2:", i32::MIN, i32::MAX);
    print!("a:                  {}", ai32.fmt_i32());
    print!("b:                  {}", bi32.fmt_i32());
    print!("a - b (wraparound): {}", wrapped.fmt_i32());

    let ai64 = Xmm {
        int64: [100_000_000_000, 200],
    };
    let bi64 = Xmm {
        int64: [99, 300_000_000_000],
    };
    let mut wrapped = Xmm {
        int64: [0; 2],
    };

    sub_i64_sse2(&ai64, &bi64, &mut wrapped);

    println!("Subtraction i64[{},{}] sse2:", i64::MIN, i64::MAX);
    print!("a:                  {}", ai64.fmt_i64());
    print!("b:                  {}", bi64.fmt_i64());
    print!("a - b (wraparound): {}", wrapped.fmt_i64());
}