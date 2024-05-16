//! _mm_unpacklo_epi8: size-promoting operation, 8-bit to 16-bit
//! _mm_unpackhi_epi8: size-promoting operation, 8-bit to 16-bit

use std::thread;
use simd::array::Array;

fn calc_mean_u8(array: &Array<u8>) -> (Option<u64>, Option<f64>) {
    let mut sum: u64 = 0;
    for &val in array.as_slice() {
        sum += val as u64;
    }

    (Some(sum), Some(sum as f64 / array.len() as f64))
}

fn calc_mean_u8_sse2(array: &Array<u8>) -> (Option<u64>, Option<f64>) {
    use std::arch::x86_64::{_mm_add_epi16, _mm_add_epi32, _mm_extract_epi32, _mm_load_si128, _mm_setzero_si128, _mm_unpackhi_epi16, _mm_unpackhi_epi8, _mm_unpacklo_epi16, _mm_unpacklo_epi8};

    if array.is_empty() || !array.is_aligned() {
        return (None, None);
    }
    const NUM_LANE: usize = 16;

    unsafe {
        let packed_zero = _mm_setzero_si128();
        let mut sums_u32 = _mm_setzero_si128();

        let mut i = 0;
        loop {
            if i + NUM_LANE * 4 > array.len() {
                break;
            }

            let mut sums_u16 = _mm_setzero_si128();
            let mut vals_lo_u16 = _mm_setzero_si128();
            let mut vals_hi_u16 = _mm_setzero_si128();
            let mut vals_u8;

            for j in 0..4 {
                vals_u8 = _mm_load_si128(array.as_ptr().add(i + NUM_LANE * j) as *const _);
                vals_lo_u16 = _mm_unpacklo_epi8(vals_u8, packed_zero);
                vals_hi_u16 = _mm_unpackhi_epi8(vals_u8, packed_zero);
                sums_u16 = _mm_add_epi16(sums_u16, vals_lo_u16);
                sums_u16 = _mm_add_epi16(sums_u16, vals_hi_u16);
            }

            // convert sums_u16 to u32, then update sums_u32
            let sums_u32_lo = _mm_unpacklo_epi16(sums_u16, packed_zero);
            let sums_u32_hi = _mm_unpackhi_epi16(sums_u16, packed_zero);
            sums_u32 = _mm_add_epi32(sums_u32, sums_u32_lo);
            sums_u32 = _mm_add_epi32(sums_u32, sums_u32_hi);

            i += NUM_LANE * 4;
        }

        // reduce sums_u32 to single u64
        let mut sum = _mm_extract_epi32::<0>(sums_u32) as u64;
        sum += _mm_extract_epi32::<1>(sums_u32) as u64;
        sum += _mm_extract_epi32::<2>(sums_u32) as u64;
        sum += _mm_extract_epi32::<3>(sums_u32) as u64;

        if i < array.len() {
            for j in i..array.len() {
                sum += array.as_slice()[j] as u64;
            }
        }

        (Some(sum), Some(sum as f64 / array.len() as f64))
    }
}

const NUM_ELEMENTS: usize = 1_000_000;

fn main() {
    let mut array = Array::<u8>::new(NUM_ELEMENTS);
    array.randomise(0, 255, false);

    thread::scope(|s| {
        s.spawn(|| {
            let (sum, mean) = calc_mean_u8(&array);

            let mut i = 0;
            let start = std::time::Instant::now();
            while i < 100 {
                let (_, _) = calc_mean_u8(&array);
                i += 1;
            }
            let duration = start.elapsed();
            println!("Scalar: sum = {:?}, mean = {:.6?} in {} μs", sum, mean, duration.as_micros() / 100);
        });

        s.spawn(|| {
            let (sum, mean) = calc_mean_u8_sse2(&array);

            let mut i = 0;
            let start = std::time::Instant::now();
            while i < 100 {
                let (_, _) = calc_mean_u8_sse2(&array);
                i += 1;
            }
            let duration = start.elapsed();
            println!("SSE2: sum = {:?}, mean = {:.6?} in {} μs", sum, mean, duration.as_micros() / 100);
        });
    });
}