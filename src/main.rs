#![no_std]
#![no_main]

use core::str::from_utf8;

use embassy_executor::Spawner;
use embassy_futures::select::{select3, Either3};
use embassy_time::Duration;
use microbit_bsp::{display, Microbit};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(#[allow(clippy::used_underscore_binding)]_spawner: Spawner) {
    let board = Microbit::default();
    let mut btn_a = board.btn_a;
    let mut btn_b = board.btn_b;
    let mut display = board.display;

    display.set_brightness(display::Brightness::MAX);
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;
    let mut buf = [0u8; 6];
    loop {
        if seconds == 60 {
            seconds = 0;
            minutes += 1;
            if minutes == 60 {
                minutes = 0;
                hours = (hours + 1) % 24;
            }
        }
        format_time(&mut buf, hours, minutes);
        let time_str = from_utf8(&buf).expect("bytes should be valid UTF-8");
        defmt::info!("Time {:02}:{:02}:{:02}", hours, minutes, seconds);
        match select3(
            display.scroll_with_speed(time_str, Duration::from_millis(2000)),
            btn_a.wait_for_low(),
            btn_b.wait_for_low(),
        )
        .await
        {
            Either3::First(()) => {
                seconds += 2;
            }
            Either3::Second(()) => {
                hours = (hours + 1) % 24;
                btn_a.wait_for_high().await;
            }
            Either3::Third(()) => {
                minutes = (minutes + 1) % 60;
                seconds = 0;
                btn_b.wait_for_high().await;
            }
        }
    }
}

fn format_time(buf: &mut [u8], hours: u8, minutes: u8) {
    buf[0] = b' ';
    buf[1] = b'0' + hours / 10;
    buf[2] = b'0' + hours % 10;
    buf[3] = b':';
    buf[4] = b'0' + minutes / 10;
    buf[5] = b'0' + minutes % 10;
}
