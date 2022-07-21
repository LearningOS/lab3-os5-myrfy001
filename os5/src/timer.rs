//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::{set_timer, console_putchar};
use riscv::register::time;

const TICKS_PER_SEC: usize = 25;
const MICRO_PER_SEC: usize = 1_000_000;

/// read the `mtime` register
pub fn get_time() -> usize {
    time::read()
}

/// get current time in microseconds
pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}


static mut ccc :usize = 0;
/// set the next timer interrupt
pub fn set_next_trigger() {
    let now = get_time();
    let n = now + CLOCK_FREQ / TICKS_PER_SEC;
    set_timer(n);
    // println!("@st");
    unsafe {
        // println!("st@{}\n now={} \n", n, now);
        // print_int("now ", now);
        // print_int("nxt ", n);
        // print_int("det ", now-ccc);
        ccc = now;
    }
}

fn print_int(s: &str, x: usize) {
    let mut x = x;
    let mut b:[u8;12] = [32; 12];

    for i in (0..12).rev() {
        b[i] = (x % 10 + 48) as u8;
        x = x / 10;
    }

    
    for i in s.as_bytes().iter().chain(b.as_slice()) {
        console_putchar(*i as usize);
    }
    console_putchar('\n' as usize);





}