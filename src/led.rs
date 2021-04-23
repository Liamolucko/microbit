//! Blocking support for the 5x5 LED display.
//!
//! This module provides a simple blocking interface
//! to the on board 5x5 LED display. If you need a more sophisticated
//! or non-blocking interface use the [`display`](crate::display) module.
//!
//! # Example
//!
//! ```no_run
//! use microbit::{
//!     display_pins,
//!     hal::{gpio::p0::Parts, prelude::*, Timer},
//! };
//! // take the peripherals
//! let p = microbit::pac::Peripherals::take().unwrap();
//! // make a timer
//! let mut timer = Timer::new(p.TIMER0);
//! // split off the p0::Parts
//! let p0parts = Parts::new(p.GPIO);
//! // create the DisplayPins struct
//! let pins = display_pins!(p0parts);
//! // create the Display
//! let mut leds = led::Display::new(pins);
//! // and light up some LEDs
//! let heart = [
//!     [0, 1, 0, 1, 0],
//!     [1, 0, 1, 0, 1],
//!     [1, 0, 0, 0, 1],
//!     [0, 1, 0, 1, 0],
//!     [0, 0, 1, 0, 0],
//! ];
//! loop {
//!     leds.display(&mut timer, heart, 1000);
//!     leds.clear();
//!     timer.delay_ms(250);
//! }
//! ```
//!
//! See a working example at `examples/led_blocking.rs`
use crate::display;
use crate::display::MATRIX_COLS;
use crate::display::MATRIX_ROWS;
use crate::hal::{
    gpio::{Output, Pin, PushPull},
    prelude::*,
};

use crate::gpio::DisplayPins;

use embedded_hal::blocking::delay::DelayUs;

#[allow(clippy::upper_case_acronyms)]
pub(crate) type LED = Pin<Output<PushPull>>;

const DEFAULT_DELAY_MS: u32 = 2;
#[cfg(feature = "v1")]
const LED_LAYOUT: [[(usize, usize); 5]; 5] = [
    [(0, 0), (1, 3), (0, 1), (1, 4), (0, 2)],
    [(2, 3), (2, 4), (2, 5), (2, 6), (2, 7)],
    [(1, 1), (0, 8), (1, 2), (2, 8), (1, 0)],
    [(0, 7), (0, 6), (0, 5), (0, 4), (0, 3)],
    [(2, 2), (1, 6), (2, 0), (1, 5), (2, 1)],
];

/// Blocking interface to the on board LED display
pub struct Display {
    delay_ms: u32,
    rows: [LED; display::MATRIX_ROWS],
    cols: [LED; display::MATRIX_COLS],
}

type Matrix = [[u8; MATRIX_COLS]; MATRIX_ROWS];

impl Display {
    /// Initialise display
    ///
    /// The [`display_pins!`](crate::display_pins) macro can be used
    /// to create [`DisplayPins`].
    pub fn new(pins: DisplayPins) -> Self {
        let mut retval = Display {
            delay_ms: DEFAULT_DELAY_MS,
            rows: [
                pins.row1.degrade(),
                pins.row2.degrade(),
                pins.row3.degrade(),
                #[cfg(feature = "v2")]
                pins.row4.degrade(),
                #[cfg(feature = "v2")]
                pins.row5.degrade(),
            ],
            cols: [
                pins.col1.degrade(),
                pins.col2.degrade(),
                pins.col3.degrade(),
                pins.col4.degrade(),
                pins.col5.degrade(),
                #[cfg(feature = "v1")]
                pins.col6.degrade(),
                #[cfg(feature = "v1")]
                pins.col7.degrade(),
                #[cfg(feature = "v1")]
                pins.col8.degrade(),
                #[cfg(feature = "v1")]
                pins.col9.degrade(),
            ],
        };
        // This is needed to reduce flickering on reset
        retval.clear();
        retval
    }

    /// Clear display
    pub fn clear(&mut self) {
        for row in &mut self.rows {
            row.set_low().ok();
        }
        for col in &mut self.cols {
            col.set_high().ok();
        }
    }

    /// Set delay, time spent on each matrix row, in ms
    pub fn set_delay_ms(&mut self, delay_ms: u32) {
        self.delay_ms = delay_ms;
    }

    /// Set refresh rate, time for matrix scan
    pub fn set_refresh_rate(&mut self, freq_hz: u32) {
        self.delay_ms = 1000 / freq_hz / 3;
    }

    /// Convert 5x5 display image to 3x9 matrix image
    pub fn display2matrix(led_display: [[u8; 5]; 5]) -> Matrix {
        #[cfg(feature = "v2")]
        return led_display;
        #[cfg(feature = "v1")]
        {
            let mut led_matrix: Matrix = [[0; MATRIX_COLS]; MATRIX_ROWS];
            for (led_display_row, layout_row) in led_display.iter().zip(LED_LAYOUT.iter()) {
                for (led_display_val, layout_loc) in led_display_row.iter().zip(layout_row) {
                    led_matrix[layout_loc.0][layout_loc.1] = *led_display_val;
                }
            }
            led_matrix
        }
    }

    /// Display 5x5 display image for a given duration
    pub fn display<D: DelayUs<u32>>(
        &mut self,
        delay: &mut D,
        led_display: [[u8; 5]; 5],
        duration_ms: u32,
    ) {
        let led_matrix = Display::display2matrix(led_display);
        self.display_pre(delay, led_matrix, duration_ms);
    }

    /// Display 3x9 matrix image for a given duration
    pub fn display_pre<D: DelayUs<u32>>(
        &mut self,
        delay: &mut D,
        led_matrix: Matrix,
        duration_ms: u32,
    ) {
        // TODO: something more intelligent with timers
        let loops = duration_ms / (self.rows.len() as u32 * self.delay_ms);
        for _ in 0..loops {
            for (row_line, led_matrix_row) in self.rows.iter_mut().zip(led_matrix.iter()) {
                row_line.set_high().ok();
                for (col_line, led_matrix_val) in self.cols.iter_mut().zip(led_matrix_row.iter()) {
                    // TODO : use value to set brightness
                    if *led_matrix_val > 0 {
                        col_line.set_low().ok();
                    }
                }
                delay.delay_us(self.delay_ms * 1000);
                for col_line in &mut self.cols {
                    col_line.set_high().ok();
                }
                row_line.set_low().ok();
            }
        }
    }
}
