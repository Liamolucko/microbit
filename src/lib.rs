//! microbit contains everything required to get started with the use of Rust
//! to create firmwares for the fabulous [BBC micro:bit](https://microbit.org)
//! microcontroller board.
#![no_std]
#![deny(missing_docs)]
#![allow(non_camel_case_types)]

#[cfg(feature = "v1")]
pub use nrf51_hal as hal;
#[cfg(feature = "v2")]
pub use nrf52833_hal as hal;
#[cfg(not(any(feature = "v1", feature = "v2")))]
compile_error!("Must enable either the `v1` or the `v2` feature.");

pub use hal::pac;
pub use hal::pac::Peripherals;

pub mod display;
pub mod gpio;
pub mod led;

/// Create a [Uart](hal::uart::Uart) client with the default pins
#[macro_export]
#[cfg(feature = "v1")]
macro_rules! serial_port {
    ( $gpio:expr, $uart:expr, $speed:expr ) => {{
        use microbit::hal::{gpio::Level, uart};

        /* Configure RX and TX pins accordingly */
        let pins = uart::Pins {
            rxd: $gpio.p0_25.into_floating_input().degrade(),
            txd: $gpio.p0_24.into_push_pull_output(Level::Low).degrade(),
            cts: None,
            rts: None,
        };

        /* Set up serial port using the prepared pins */
        uart::Uart::new($uart, pins, uart::Parity::EXCLUDED, $speed)
    }};
}

/// Create a [Uarte](hal::uart::Uarte) client with the default pins
#[macro_export]
#[cfg(feature = "v2")]
macro_rules! serial_port {
    ( $p0:expr, $p1:expr, $uart:expr, $speed:expr ) => {{
        use microbit::hal::{gpio::Level, uarte};

        /* Configure RX and TX pins accordingly */
        let pins = uarte::Pins {
            rxd: $p1.p1_08.into_floating_input().degrade(),
            txd: $p0.p0_06.into_push_pull_output(Level::Low).degrade(),
            cts: None,
            rts: None,
        };

        /* Set up serial port using the prepared pins */
        uarte::Uarte::new($uart, pins, uarte::Parity::EXCLUDED, $speed)
    }};
}
