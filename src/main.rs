use std::io::{self, BufRead};
use std::f64::consts::PI;

const FREQUENCY: f64 = 0.25;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Error while reading from stdin");
        rainbow_println(line);
    }
}
// \x1b[0m
// \x1b[38;5;{}m
fn rainbow_println(s: String) {
    let mut offset = 0;
    let mut escaped = false;
    for (i, c) in s.chars().enumerate() {
        if escaped {
            offset += 1;
            if c == 'm' {
                escaped = false;
            }
            continue;
        }
        if c == '\x1b' {
            escaped = true;
            continue;
        }
        print!(
            "\x1b[38;5;{}m{}", 
            rgb_to_ansi(rainbow(FREQUENCY, i - offset)),
            c
        );
    }
    print!("\x1b[0m\n");
}
// returned f64s should be cast to u8
fn rainbow(freq: f64, i: usize) -> (f64, f64, f64) {
    let m = freq * i as f64;
    let r = (m + 0.0)           .sin() * 127.0 + 128.0;
    let g = (m + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
    let b = (m + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;
    (r, g, b)
}
fn rgb_to_ansi(color: (f64, f64, f64)) -> i32 {
    let (r, g, b) = color;
    if r == g && g == b {
        if r < 8.0 {
            return 16;
        }
        if r > 248.0 {
            return 231;
        }
        return ((((r - 8.0) / 247.0) * 24.0) + 232.0) as i32;
    }
    return 16
        + (36 * (r / 255.0 * 5.0) as i32)
        + (6  * (g / 255.0 * 5.0) as i32)
        + (b / 255.0 * 5.0) as i32;
}
