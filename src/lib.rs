use std::fmt;

pub mod xmm;
pub mod ymm;
pub mod array;

pub(crate) fn fmt_as_simd<T: fmt::Display>(f: &mut String, a: &[T], n: usize, w: usize) -> fmt::Result {
    for i in 0..a.len() {
        fmt::write(f, format_args!("{:w$}", a[i], w = w))?;
        if i + 1 == n / 2 {
            fmt::write(f, format_args!("    |"))?;
        }
    }
    fmt::write(f, format_args!("\n"))
}

pub(crate) fn fmt_as_simd_hex<T: fmt::Display + fmt::UpperHex>(f: &mut String, a: &[T], n: usize, w: usize) -> fmt::Result {
    for i in 0..a.len() {
        fmt::write(f, format_args!("{:#0w$X} ", a[i], w = w))?;
        if i + 1 == n / 2 {
            fmt::write(f, format_args!(" |  "))?;
        }
    }
    fmt::write(f, format_args!("\n"))
}