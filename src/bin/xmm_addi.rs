//! _mm_add_epi[8/16/32/64]: adds packed 16-bit integers (**signed** & **unsigned**) in a and b (wraparound)
//! _mm_adds_epi[8/16]: adds packed 16-bit **signed** integers in a and b (saturated)
//! _mm_adds_epu[8/16]: adds packed unsigned 16-bit **unsigned** integers in a and b (saturated)

use simd::xmm::*;

pub fn add_i16_sse2(a: &Xmm, b: &Xmm, wrapped: &mut Xmm, saturated: &mut Xmm) {
    use std::arch::x86_64::{
        __m128i, _mm_add_epi16, _mm_adds_epi16, _mm_load_si128, _mm_store_si128,
    };
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const _);
        let b_val = _mm_load_si128(b as *const Xmm as *const _);

        let c1_val = _mm_add_epi16(a_val, b_val);
        let c2_val = _mm_adds_epi16(a_val, b_val);
        _mm_store_si128(wrapped as *mut Xmm as *mut __m128i, c1_val);
        _mm_store_si128(saturated as *mut Xmm as *mut __m128i, c2_val);
    }
}

pub fn add_u16_sse2(a: &Xmm, b: &Xmm, wrapped: &mut Xmm, saturated: &mut Xmm) {
    use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_store_si128, _mm_add_epi16, _mm_adds_epu16};
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const _);
        let b_val = _mm_load_si128(b as *const Xmm as *const _);

        let c1_val = _mm_add_epi16(a_val, b_val);
        let c2_val = _mm_adds_epu16(a_val, b_val);
        _mm_store_si128(wrapped as *mut Xmm as *mut __m128i, c1_val);
        _mm_store_si128(saturated as *mut Xmm as *mut __m128i, c2_val);
    }
}

fn main() {
    let ia = Xmm { int16: [10, 200, 30, -32766, 50, 60, 32000, -32000] };
    let ib = Xmm { int16: [100, -200, 32760, -400, 500, -600, 1200, -950] };

    let mut wraparound = Xmm { int16: [0; 8] };
    let mut saturated = Xmm { int16: [0; 8] };

    add_i16_sse2(&ia, &ib, &mut wraparound, &mut saturated);

    println!("Addition i16[{},{}] sse2:", i16::MIN, i16::MAX);
    print!("a:                  {}", ia.fmt_i16());
    print!("b:                  {}", ib.fmt_i16());
    print!("a + b (wraparound): {}", wraparound.fmt_i16());
    print!("a + b (saturated):  {}", saturated.fmt_i16());

    let ua = Xmm { uint16: [10, 200, 300, 32766, 50, 20000, 32000, 32000] };
    let ub = Xmm { uint16: [100, 200, 65530, 40000, 500, 25000, 1200, 50000] };

    add_u16_sse2(&ua, &ub, &mut wraparound, &mut saturated);

    println!("Addition u16[{},{}] sse2:", u16::MIN, u16::MAX);
    print!("a:                  {}", ua.fmt_u16());
    print!("b:                  {}", ub.fmt_u16());
    print!("a + b (wraparound): {}", wraparound.fmt_u16());
    print!("a + b (saturated):  {}", saturated.fmt_u16());
}