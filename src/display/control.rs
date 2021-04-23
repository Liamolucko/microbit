//! Implementation of [`DisplayControl`] for the micro:bit's GPIO peripheral.
//!
//! This controls the micro:bit's 5×5 LED display.
//!
//! [`DisplayControl`]: tiny_led_matrix::DisplayControl

use crate::gpio::DisplayPins;
use nrf52833_hal::prelude::*;
use tiny_led_matrix::DisplayControl;

#[cfg(feature = "v1")]
pub(crate) const MATRIX_COLS: usize = 9;
#[cfg(feature = "v2")]
pub(crate) const MATRIX_COLS: usize = 5;

#[cfg(feature = "v1")]
pub(crate) const MATRIX_ROWS: usize = 3;
#[cfg(feature = "v2")]
pub(crate) const MATRIX_ROWS: usize = 5;

/// Implementation of [`DisplayControl`] for the micro:bit's GPIO peripheral.
///
/// This controls the micro:bit's 5×5 LED display.
///
/// The `initialise_for display` implementation assumes the port is in the
/// state it would have after system reset.
///
/// [`DisplayControl`]: tiny_led_matrix::DisplayControl
impl DisplayControl for DisplayPins {
    fn initialise_for_display(&mut self) {
        // Set all cols high.
        // nrf-hal GPIO pins are infallible
        self.col1.set_high().unwrap();
        self.col2.set_high().unwrap();
        self.col3.set_high().unwrap();
        self.col4.set_high().unwrap();
        self.col5.set_high().unwrap();
        #[cfg(feature = "v1")]
        {
            self.col6.set_high().unwrap();
            self.col7.set_high().unwrap();
            self.col8.set_high().unwrap();
            self.col9.set_high().unwrap();
        }
    }

    fn display_row_leds(&mut self, row: usize, cols: u32) {
        // To light an LED, we set the row bit and clear the col bit.

        // Clear all rows
        self.row1.set_low().unwrap();
        self.row2.set_low().unwrap();
        self.row3.set_low().unwrap();
        #[cfg(feature = "v2")]
        {
            self.row4.set_low().unwrap();
            self.row5.set_low().unwrap();
        }

        // Set/clear columns
        if cols & 0b000000001 != 0 {
            self.col1.set_low().unwrap();
        } else {
            self.col1.set_high().unwrap();
        }
        if cols & 0b000000010 != 0 {
            self.col2.set_low().unwrap();
        } else {
            self.col2.set_high().unwrap();
        }
        if cols & 0b000000100 != 0 {
            self.col3.set_low().unwrap();
        } else {
            self.col3.set_high().unwrap();
        }
        if cols & 0b000001000 != 0 {
            self.col4.set_low().unwrap();
        } else {
            self.col4.set_high().unwrap();
        }
        if cols & 0b000010000 != 0 {
            self.col5.set_low().unwrap();
        } else {
            self.col5.set_high().unwrap();
        }
        #[cfg(feature = "v1")]
        {
            if cols & 0b000100000 != 0 {
                self.col6.set_low().unwrap();
            } else {
                self.col6.set_high().unwrap();
            }
            if cols & 0b001000000 != 0 {
                self.col7.set_low().unwrap();
            } else {
                self.col7.set_high().unwrap();
            }
            if cols & 0b010000000 != 0 {
                self.col8.set_low().unwrap();
            } else {
                self.col8.set_high().unwrap();
            }
            if cols & 0b100000000 != 0 {
                self.col9.set_low().unwrap();
            } else {
                self.col9.set_high().unwrap();
            }
        }

        // Set current row
        if row == 0 {
            self.row1.set_high().unwrap();
        }
        if row == 1 {
            self.row2.set_high().unwrap();
        }
        if row == 2 {
            self.row3.set_high().unwrap();
        }
        #[cfg(feature = "v2")]
        {
            if row == 3 {
                self.row4.set_high().unwrap();
            }
            if row == 4 {
                self.row5.set_high().unwrap();
            }
        }
    }

    fn light_current_row_leds(&mut self, cols: u32) {
        if cols & 0b000000001 != 0 {
            self.col1.set_low().unwrap();
        }
        if cols & 0b000000010 != 0 {
            self.col2.set_low().unwrap();
        }
        if cols & 0b000000100 != 0 {
            self.col3.set_low().unwrap();
        }
        if cols & 0b000001000 != 0 {
            self.col4.set_low().unwrap();
        }
        if cols & 0b000010000 != 0 {
            self.col5.set_low().unwrap();
        }
        #[cfg(feature = "v1")]
        {
            if cols & 0b000100000 != 0 {
                self.col6.set_low().unwrap();
            }
            if cols & 0b001000000 != 0 {
                self.col7.set_low().unwrap();
            }
            if cols & 0b010000000 != 0 {
                self.col8.set_low().unwrap();
            }
            if cols & 0b100000000 != 0 {
                self.col9.set_low().unwrap();
            }
        }
    }
}
