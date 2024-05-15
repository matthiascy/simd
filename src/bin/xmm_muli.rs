use std::arch::x86_64::{_mm_mul_epi32, _mm_srli_si128};
use simd::xmm::Xmm;

pub fn mul_i16_sse2(a: &Xmm, b: &Xmm, lo: &mut Xmm, hi: &mut Xmm) {
    use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_store_si128, _mm_mullo_epi16, _mm_mulhi_epi16, _mm_unpacklo_epi16, _mm_unpackhi_epi16};
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const __m128i);
        let b_val = _mm_load_si128(b as *const Xmm as *const __m128i);

        let temp_lo = _mm_mullo_epi16(a_val, b_val);
        let temp_hi = _mm_mulhi_epi16(a_val, b_val);

        _mm_store_si128(lo as *mut Xmm as *mut __m128i, _mm_unpacklo_epi16(temp_lo, temp_hi));
        _mm_store_si128(hi as *mut Xmm as *mut __m128i, _mm_unpackhi_epi16(temp_lo, temp_hi));
    }
}

pub fn mul_i32_sse2(a: &Xmm, b: &Xmm, lo: &mut Xmm) {
    use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_store_si128, _mm_mullo_epi32};
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const __m128i);
        let b_val = _mm_load_si128(b as *const Xmm as *const __m128i);

        // The product should no overflow, so we can use _mm_mullo_epi32
        let result = _mm_mullo_epi32(a_val, b_val);

        _mm_store_si128(lo as *mut Xmm as *mut __m128i, result);
    }
}

pub fn mul_i32_sse2_v2(a: &Xmm, b: &Xmm, lo: &mut Xmm, hi: &mut Xmm) {
    use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_extract_epi64};
    unsafe {
        // a3 | a2 | a1 | a0
        // b3 | b2 | b1 | b0
        let a_val = _mm_load_si128(a as *const Xmm as *const __m128i);
        let b_val = _mm_load_si128(b as *const Xmm as *const __m128i);

        // _mm_mul_epi32(a, b)
        // Multiply the low signed 32-bit integers from each packed 64-bit element

        // a2 * b2 | a0 * b0
        //    q2   |   q0
        let temp1 = _mm_mul_epi32(a_val, b_val); // q2 | q0
        // right shift 4 bytes a_val and b_val
        // 0 | a3 | a2 | a1
        let temp2 = _mm_srli_si128::<4>(a_val);
        // 0 | b3 | b2 | b1
        let temp3 = _mm_srli_si128::<4>(b_val);
        // a3 * b3 | a1 * b1
        //    q3   |   q1
        let temp4 = _mm_mul_epi32(temp2, temp3); // q3 | q1

        lo.int64[0] = _mm_extract_epi64::<0>(temp1); // q0
        lo.int64[1] = _mm_extract_epi64::<0>(temp4); // q1
        hi.int64[0] = _mm_extract_epi64::<1>(temp1); // q2
        hi.int64[1] = _mm_extract_epi64::<1>(temp4); // q3
    }
}

fn main() {
    // i16
    let ai16 = Xmm { int16: [10, 3000, -2000, 42, -5000, 8, 10000, -60] };
    let bi16 = Xmm { int16: [-5, 100, -9000, 1000, 25000, 16384, 3500, 6000] };
    let mut loi16 = Xmm { int16: [0; 8] };
    let mut hii16 = Xmm { int16: [0; 8] };

    mul_i16_sse2(&ai16, &bi16, &mut loi16, &mut hii16);

    println!("Multiplication i16[{},{}] sse2:", i16::MIN, i16::MAX);
    print!("a:                  {}", ai16.fmt_i16());
    print!("b:                  {}", bi16.fmt_i16());
    print!("a * b (lo):         {}", loi16.fmt_i32());
    print!("a * b (hi):         {}", hii16.fmt_i32());

    // i32
    let ai32 = Xmm { int32: [10, 3000, -2000, 4200] };
    let bi32 = Xmm { int32: [-500, 100, -12000, 1000] };
    let mut loi32 = Xmm { int32: [0; 4] };

    mul_i32_sse2(&ai32, &bi32, &mut loi32);

    println!("Multiplication i32[{},{}] sse2:", i32::MIN, i32::MAX);
    print!("a:                  {}", ai32.fmt_i32());
    print!("b:                  {}", bi32.fmt_i32());
    print!("a * b (lo):         {}", loi32.fmt_i32());

    // i32 v2
    let mut out = [Xmm { int64: [0; 2] }, Xmm { int64: [0; 2] }];
    let ai32_v2 = Xmm { int32: [10, 3000, -40000, 4200] };
    let bi32_v2 = Xmm { int32: [-500, 100, -120000, 1000] };
    let mut loi32_v2 = Xmm { int32: [0; 4] };
    let mut hii32_v2 = Xmm { int32: [0; 4] };

    mul_i32_sse2_v2(&ai32_v2, &bi32_v2, &mut loi32_v2, &mut hii32_v2);

    println!("Multiplication i32[{},{}] sse2 v2:", i32::MIN, i32::MAX);
    print!("a:                  {}", ai32_v2.fmt_i32());
    print!("b:                  {}", bi32_v2.fmt_i32());
    print!("a * b (lo):         {}", loi32_v2.fmt_i64());
    print!("a * b (hi):         {}", hii32_v2.fmt_i64());
}