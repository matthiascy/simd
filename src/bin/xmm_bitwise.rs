//! _mm_s[l/r]li_epi[8/16/32/64]: shift (in zeros) packed 8/16/32/64-bit integers in a left/right
//! _mm_s[l/r]ai_epi[8/16/32/64]: shift (in sign) packed 8/16/32/64-bit integers in a left/right

use simd::xmm::Xmm;

fn and_u16_sse2(a: &Xmm, b: &Xmm, out: &mut Xmm) {
    use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_and_si128, _mm_store_si128};
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const __m128i);
        let b_val = _mm_load_si128(b as *const Xmm as *const __m128i);
        let c_val = _mm_and_si128(a_val, b_val);

        _mm_store_si128(out as *mut Xmm as *mut __m128i, c_val);
    }
}

fn or_u16_sse2(a: &Xmm, b: &Xmm, out: &mut Xmm) {
    use std::arch::x86_64::{_mm_load_si128, _mm_or_si128, _mm_store_si128};
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const _);
        let b_val = _mm_load_si128(b as *const Xmm as *const _);
        let c_val = _mm_or_si128(a_val, b_val);

        _mm_store_si128(out as *mut Xmm as *mut _, c_val);
    }
}

fn xor_u16_sse2(a: &Xmm, b: &Xmm, out: &mut Xmm) {
    use std::arch::x86_64::{_mm_load_si128, _mm_xor_si128, _mm_store_si128};
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const _);
        let b_val = _mm_load_si128(b as *const Xmm as *const _);
        let c_val = _mm_xor_si128(a_val, b_val);

        _mm_store_si128(out as *mut Xmm as *mut _, c_val);
    }
}

fn sll_u16_sse2<const C: i32>(a: &Xmm, out: &mut Xmm) {
    use std::arch::x86_64::{_mm_load_si128, _mm_slli_epi16, _mm_store_si128};
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const _);
        let out_val = _mm_slli_epi16::<C>(a_val);

        _mm_store_si128(out as *mut Xmm as *mut _, out_val);
    }
}

fn srl_u16_sse2<const C: i32>(a: &Xmm, out: &mut Xmm) {
    use std::arch::x86_64::{_mm_load_si128, _mm_srli_epi16, _mm_store_si128};
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const _);
        let out_val = _mm_srli_epi16::<C>(a_val);

        _mm_store_si128(out as *mut Xmm as *mut _, out_val);
    }
}

fn sra_u16_sse2<const C: i32>(a: &Xmm, out: &mut Xmm) {
    use std::arch::x86_64::{_mm_load_si128, _mm_srai_epi16, _mm_store_si128};
    unsafe {
        let a_val = _mm_load_si128(a as *const Xmm as *const _);
        let out_val = _mm_srai_epi16::<C>(a_val);

        _mm_store_si128(out as *mut Xmm as *mut _, out_val);
    }
}

fn main() {
    let a = Xmm { uint16: [0x1234, 0xABDC, 0xAA55, 0x1111, 0xFFFF, 0x7F7F, 0x9876, 0x7F00] };
    let b = Xmm { uint16: [0xFF00, 0x00FF, 0xAAAA, 0x5555, 0x8000, 0x7FFF, 0xF0F0, 0x0880] };
    let mut c = Xmm { uint16: [0; 8] };

    and_u16_sse2(&a, &b, &mut c);

    println!("AND u16 sse2:");
    print!("a:     {}", a.fmt_u16hex());
    print!("b:     {}", b.fmt_u16hex());
    print!("a & b: {}", c.fmt_u16hex());

    or_u16_sse2(&a, &b, &mut c);

    println!("OR u16 sse2:");
    print!("a:     {}", a.fmt_u16hex());
    print!("b:     {}", b.fmt_u16hex());
    print!("a | b: {}", c.fmt_u16hex());

    xor_u16_sse2(&a, &b, &mut c);

    println!("XOR u16 sse2:");
    print!("a:     {}", a.fmt_u16hex());
    print!("b:     {}", b.fmt_u16hex());
    print!("a ^ b: {}", c.fmt_u16hex());

    let a = Xmm { uint16: [0x1234, 0xFFB0, 0x00CC, 0x8080, 0x00FF, 0xAAAA, 0x0F0F, 0x0101] };
    let mut out = Xmm { uint16: [0; 8] };

    sll_u16_sse2::<4>(&a, &mut out);

    println!("Slli u16 sse2:");
    print!("a:      {}", a.fmt_u16hex());
    print!("a << 4: {}", out.fmt_u16hex());

    srl_u16_sse2::<4>(&a, &mut out);

    println!("Srli u16 sse2:");
    print!("a:      {}", a.fmt_u16hex());
    print!("a >> 4: {}", out.fmt_u16hex());

    sra_u16_sse2::<4>(&a, &mut out);

    println!("Srai u16 sse2:");
    print!("a:      {}", a.fmt_u16hex());
    print!("a >> 4: {}", out.fmt_u16hex());
}