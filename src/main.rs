#![no_std]
#![no_main]

use core::str::from_utf8;

use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_time::Duration;
use microbit_bsp::{display, Microbit};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Microbit::default();
    let mut btn_a = board.btn_a;
    let mut btn_b = board.btn_b;
    let mut display = board.display;

    display.set_brightness(display::Brightness::MAX);
    let mut hours = 13;
    let mut minutes = 1;
    let mut buf = [0u8; 5];
    loop {
        format_time(&mut buf, hours, minutes);
        let time_str = from_utf8(&buf).expect("bytes should be valid UTF-8");
        display.scroll_with_speed(time_str, Duration::from_millis(2000)).await;
        // match select(btn_a.wait_for_low(), btn_b.wait_for_low()).await {
        //     Either::First(()) => {
        //         defmt::info!("A pressed");
        //         hours = (hours + 1) % 24;
        //     },
        //     Either::Second(()) => {
        //         defmt::info!("B pressed");
        //         minutes = (minutes + 1) % 60;
        //     },
        // }
    }
}


fn format_time(buf: &mut [u8], hours: u8, minutes: u8) {
    buf[0] = b'0' + hours / 10;
    buf[1] = b'0' + hours % 10;
    buf[2] = b':';
    buf[3] = b'0' + minutes / 10;
    buf[4] = b'0' + minutes % 10;
}