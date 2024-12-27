// _mm_set1_epi[8/16/32/64]: set packed 8/16/32/64-bit integers to the same value (broadcast)
// _mm_setzero_si128: set packed 128-bit integers to zero

use std::arch::x86_64::{_mm_extract_epi8, _mm_load_si128, _mm_max_epu8, _mm_min_epu8, _mm_set1_epi8, _mm_setzero_si128, _mm_srli_si128};
use std::thread;
use simd::array::Array;

fn init_array_u8(array: &mut Array<u8>) {
    array.randomise(5, 250, false);

    // use known values for min & max to verify correctness
    let n = array.len();
    array.as_mut_slice()[n / 4 * 3 + 1] = 2;
    array.as_mut_slice()[n / 4 + 11] = 3;
    array.as_mut_slice()[n / 2] = 252;
    array.as_mut_slice()[n / 2 + 13] = 253;
    array.as_mut_slice()[n / 8 + 5] = 4;
    array.as_mut_slice()[n / 8 + 7] = 254;
}

fn calc_min_max_u8(array: &Array<u8>) -> (Option<u8>, Option<u8>) {
    if array.is_empty() || !array.is_aligned(16) {
        return (None, None);
    }

    let mut min = array.as_slice()[0];
    let mut max = array.as_slice()[0];

    for &val in array.as_slice() {
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }

    (Some(min), Some(max))
}

fn calc_min_max_u8_sse2(array: &Array<u8>) -> (Option<u8>, Option<u8>) {
    if array.is_empty() || !array.is_aligned(16) {
        return (None, None);
    }
    const NUM_LANE: usize = 16;
    unsafe {
        let mut min_vals = _mm_set1_epi8(-127);
        let mut max_vals = _mm_setzero_si128();

        let mut i = 0;
        loop {
            if i + NUM_LANE > array.len() {
                break;
            }

            let vals = _mm_load_si128(array.as_ptr().add(i) as *const _);
            min_vals = _mm_min_epu8(vals, min_vals);
            max_vals = _mm_max_epu8(vals, max_vals);

            i += NUM_LANE;
        }

        // reduce min_vals & max_vals
        let mut temp = _mm_srli_si128::<8>(min_vals);
        let mut vals_reduce = _mm_min_epu8(min_vals, temp);
        temp = _mm_srli_si128::<4>(vals_reduce);
        vals_reduce = _mm_min_epu8(vals_reduce, temp);
        temp = _mm_srli_si128::<2>(vals_reduce);
        vals_reduce = _mm_min_epu8(vals_reduce, temp);
        temp = _mm_srli_si128::<1>(vals_reduce);
        vals_reduce = _mm_min_epu8(vals_reduce, temp);

        let mut min = _mm_extract_epi8::<0>(vals_reduce) as u8;

        temp = _mm_srli_si128::<8>(max_vals);
        vals_reduce = _mm_max_epu8(max_vals, temp);
        temp = _mm_srli_si128::<4>(vals_reduce);
        vals_reduce = _mm_max_epu8(vals_reduce, temp);
        temp = _mm_srli_si128::<2>(vals_reduce);
        vals_reduce = _mm_max_epu8(vals_reduce, temp);
        temp = _mm_srli_si128::<1>(vals_reduce);
        vals_reduce = _mm_max_epu8(vals_reduce, temp);

        let mut max = _mm_extract_epi8::<0>(vals_reduce) as u8;

        // handle remaining elements
        if i < array.len() {
            loop {
                let val = array.as_slice()[i];
                if val < min {
                    min = val;
                }
                if val > max {
                    max = val;
                }
                i += 1;
                if i == array.len() {
                    break;
                }
            }
        }

        (Some(min), Some(max))
    }
}

const NUM_ELEMENTS: usize = 10_000_000;

fn main() {
    println!("Running benchmark min & max of u8 array with {} elements", NUM_ELEMENTS);

    let mut array = Array::<u8>::new(NUM_ELEMENTS, 16);
    init_array_u8(&mut array);

    thread::scope(|s| {
        s.spawn(|| {
            let (min, max) = calc_min_max_u8(&array);
            let mut i = 0;
            let start = std::time::Instant::now();
            while i < 100 {
                let (_, _) = calc_min_max_u8(&array);
                i += 1;
            }
            let duration = start.elapsed();
            println!("Scalar -- min: {:?}, max: {:?}, duration: {:?} μs", min, max, duration.as_micros() / 100);
        });
        s.spawn(|| {
            let (min, max) = calc_min_max_u8_sse2(&array);
            let mut i = 0;
            let start = std::time::Instant::now();
            while i < 100 {
                let (_, _) = calc_min_max_u8_sse2(&array);
                i += 1;
            }
            let duration = start.elapsed();
            println!("SIMD -- min: {:?}, max: {:?}, duration: {:?} μs", min, max, duration.as_micros() / 100);
        });
    });
}