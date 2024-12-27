use std::arch::x86_64::_mm256_broadcast_ss;
use simd::ymm::Ymm;

fn packed_f32_avx(a: &Ymm, b: &Ymm, out: &mut [Ymm; 8]) {
    use std::arch::x86_64::{_mm256_load_ps, _mm256_store_ps, _mm256_add_ps};
    unsafe {
        let a_val = _mm256_load_ps(a.as_ptr() as *const f32);
        let b_val = _mm256_load_ps(b.as_ptr() as *const f32);
        const ABS_MASK: u32 = 0x7FFFFFFF;
        //_mm256_set1_ps(f32::from_bits(ABS_MASK));
        let abs_mask = _mm256_broadcast_ss(&f32::from_bits(ABS_MASK));

        _mm256_store_ps(out[0].as_mut_ptr() as *mut f32, _mm256_add_ps(a_val, b_val));
    }
}

fn main() {
    use std::f32::consts::{PI, SQRT_2};
    let af32 = Ymm { float: [36.0, 1.0 / 32.0, 2.0, 42.0, PI, 18.6, 3.0, 142.0] };
    let bf32 = Ymm { float: [-1.0 / 9.0, 64.0, -0.0625, 8.666667, -4.0, -64.0, 5.95, SQRT_2] };
}