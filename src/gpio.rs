//! Named GPIO pin types
//!
//! This module maps the GPIO pin names as described in the
//! [Pins and Signals section of the micro:bit site](https://tech.microbit.org/hardware/edgeconnector/#pins-and-signals)
//! Where appropriate the pins are restricted with the appropriate `MODE`
//! from `nrf-hal`.
#![allow(clippy::upper_case_acronyms, missing_docs)]
use crate::hal::gpio::{p0, Floating, Input, Output, PushPull};
#[cfg(feature = "v2")]
use crate::hal::gpio::p1;

/* GPIO pads */
#[cfg(feature = "v1")]
pub type PAD1<MODE> = p0::P0_03<MODE>;
#[cfg(feature = "v2")]
pub type PAD1<MODE> = p0::P0_02<MODE>;
#[cfg(feature = "v1")]
pub type PAD2<MODE> = p0::P0_02<MODE>;
#[cfg(feature = "v2")]
pub type PAD2<MODE> = p0::P0_03<MODE>;
#[cfg(feature = "v1")]
pub type PAD3<MODE> = p0::P0_01<MODE>;
#[cfg(feature = "v2")]
pub type PAD3<MODE> = p0::P0_04<MODE>;


/* LED display */
#[cfg(feature = "v1")]
pub type COL1 = p0::P0_04<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type COL2 = p0::P0_05<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type COL3 = p0::P0_06<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type COL4 = p0::P0_07<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type COL5 = p0::P0_08<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type COL6 = p0::P0_09<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type COL7 = p0::P0_10<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type COL8 = p0::P0_11<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type COL9 = p0::P0_12<Output<PushPull>>;

#[cfg(feature = "v1")]
pub type ROW1 = p0::P0_13<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type ROW2 = p0::P0_14<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type ROW3 = p0::P0_15<Output<PushPull>>;

#[cfg(feature = "v2")]
pub type COL1 = p0::P0_28<Output<PushPull>>;
#[cfg(feature = "v2")]
pub type COL2 = p0::P0_11<Output<PushPull>>;
#[cfg(feature = "v2")]
pub type COL3 = p0::P0_31<Output<PushPull>>;
#[cfg(feature = "v2")]
pub type COL4 = p1::P1_05<Output<PushPull>>;
#[cfg(feature = "v2")]
pub type COL5 = p0::P0_30<Output<PushPull>>;

#[cfg(feature = "v2")]
pub type ROW1 = p0::P0_21<Output<PushPull>>;
#[cfg(feature = "v2")]
pub type ROW2 = p0::P0_22<Output<PushPull>>;
#[cfg(feature = "v2")]
pub type ROW3 = p0::P0_15<Output<PushPull>>;
#[cfg(feature = "v2")]
pub type ROW4 = p0::P0_24<Output<PushPull>>;
#[cfg(feature = "v2")]
pub type ROW5 = p0::P0_19<Output<PushPull>>;

/// GPIO pins connected to the LED matrix
#[cfg(feature = "v1")]
pub struct DisplayPins {
    pub col1: COL1,
    pub col2: COL2,
    pub col3: COL3,
    pub col4: COL4,
    pub col5: COL5,
    pub col6: COL6,
    pub col7: COL7,
    pub col8: COL8,
    pub col9: COL9,
    pub row1: ROW1,
    pub row2: ROW2,
    pub row3: ROW3,
}

#[cfg(feature = "v2")]
pub struct DisplayPins {
    pub col1: COL1,
    pub col2: COL2,
    pub col3: COL3,
    pub col4: COL4,
    pub col5: COL5,
    pub row1: ROW1,
    pub row2: ROW2,
    pub row3: ROW3,
    pub row4: ROW4,
    pub row5: ROW5,
}

/// Create [DisplayPins] from a [GPIO Parts](crate::hal::gpio::p0::Parts)
#[macro_export]
#[cfg(feature = "v1")]
macro_rules! display_pins {
    ( $p0parts:expr ) => {{
        use microbit::{gpio::DisplayPins, hal::gpio::Level};

        DisplayPins {
            row1: $p0parts.p0_13.into_push_pull_output(Level::Low),
            row2: $p0parts.p0_14.into_push_pull_output(Level::Low),
            row3: $p0parts.p0_15.into_push_pull_output(Level::Low),
            col1: $p0parts.p0_04.into_push_pull_output(Level::Low),
            col2: $p0parts.p0_05.into_push_pull_output(Level::Low),
            col3: $p0parts.p0_06.into_push_pull_output(Level::Low),
            col4: $p0parts.p0_07.into_push_pull_output(Level::Low),
            col5: $p0parts.p0_08.into_push_pull_output(Level::Low),
            col6: $p0parts.p0_09.into_push_pull_output(Level::Low),
            col7: $p0parts.p0_10.into_push_pull_output(Level::Low),
            col8: $p0parts.p0_11.into_push_pull_output(Level::Low),
            col9: $p0parts.p0_12.into_push_pull_output(Level::Low),
        }
    }};
}

/// Create [DisplayPins] from a [GPIO Parts](crate::hal::gpio::p0::Parts)
#[macro_export]
#[cfg(feature = "v2")]
macro_rules! display_pins {
    ( $p0parts:expr, $p1parts:expr ) => {{
        use microbit::{gpio::DisplayPins, hal::gpio::Level};

        DisplayPins {
            row1: $p0parts.p0_21.into_push_pull_output(Level::Low),
            row2: $p0parts.p0_22.into_push_pull_output(Level::Low),
            row3: $p0parts.p0_15.into_push_pull_output(Level::Low),
            row4: $p0parts.p0_24.into_push_pull_output(Level::Low),
            row5: $p0parts.p0_19.into_push_pull_output(Level::Low),
            col1: $p0parts.p0_28.into_push_pull_output(Level::Low),
            col2: $p0parts.p0_11.into_push_pull_output(Level::Low),
            col3: $p0parts.p0_31.into_push_pull_output(Level::Low),
            col4: $p1parts.p1_05.into_push_pull_output(Level::Low),
            col5: $p0parts.p0_30.into_push_pull_output(Level::Low),
        }
    }};
}

/* buttons */
#[cfg(feature = "v1")]
pub type BTN_A = p0::P0_17<Input<Floating>>;
#[cfg(feature = "v2")]
pub type BTN_A = p0::P0_14<Input<Floating>>;
#[cfg(feature = "v1")]
pub type BTN_B = p0::P0_26<Input<Floating>>;
#[cfg(feature = "v2")]
pub type BTN_B = p0::P0_23<Input<Floating>>;

/* spi */
#[cfg(feature = "v1")]
pub type MOSI<MODE> = p0::P0_21<MODE>;
#[cfg(feature = "v2")]
pub type MOSI<MODE> = p0::P0_13<MODE>;
#[cfg(feature = "v1")]
pub type MISO<MODE> = p0::P0_22<MODE>;
#[cfg(feature = "v2")]
pub type MISO<MODE> = p0::P0_01<MODE>;
#[cfg(feature = "v1")]
pub type SCK<MODE> = p0::P0_23<MODE>;
#[cfg(feature = "v2")]
pub type SCK<MODE> = p0::P0_17<MODE>;

/* i2c */
#[cfg(feature = "v1")]
pub type SCL = p0::P0_00<Input<Floating>>;
#[cfg(feature = "v2")]
pub type SCL = p0::P0_26<Input<Floating>>;
#[cfg(feature = "v1")]
pub type SDA = p0::P0_30<Input<Floating>>;
#[cfg(feature = "v2")]
pub type SDA = p1::P1_00<Input<Floating>>;

/* uart */
#[cfg(feature = "v1")]
pub type UART_TX = p0::P0_24<Output<PushPull>>;
#[cfg(feature = "v2")]
pub type UART_TX = p0::P0_06<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type UART_RX = p0::P0_25<Input<Floating>>;
#[cfg(feature = "v2")]
pub type UART_RX = p1::P1_08<Input<Floating>>;

/* edge connector */
#[cfg(feature = "v1")]
pub type EDGE01 = COL1;
#[cfg(feature = "v2")]
pub type EDGE01 = COL3;
pub type EDGE02<MODE> = PAD1<MODE>; // <- big pad 1
#[cfg(feature = "v1")]
pub type EDGE03 = COL2;
#[cfg(feature = "v2")]
pub type EDGE03 = COL1;
pub type EDGE04 = BTN_A;
#[cfg(feature = "v1")]
pub type EDGE05 = COL9;
#[cfg(feature = "v2")]
pub type EDGE05 = COL4;
#[cfg(feature = "v1")]
pub type EDGE06 = COL8;
#[cfg(feature = "v2")]
pub type EDGE06 = COL2;
pub type EDGE07<MODE> = PAD2<MODE>; // <- big pad 2

#[cfg(feature = "v1")]
pub type EDGE08<MODE> = p0::P0_18<MODE>;
#[cfg(feature = "v2")]
pub type EDGE08<MODE> = p0::P0_10<MODE>;
#[cfg(feature = "v1")]
pub type EDGE09 = COL7;
#[cfg(feature = "v2")]
// TODO(Liamolucko): Should this be generic? I just put it like this for now to match the existing `EDGE09`.
// Also, should they then all be generic?
pub type EDGE09 = p0::P0_09<Output<PushPull>>;
#[cfg(feature = "v1")]
pub type EDGE10 = COL3;
#[cfg(feature = "v2")]
pub type EDGE10 = COL5;
pub type EDGE11 = BTN_B;
#[cfg(feature = "v1")]
pub type EDGE12<MODE> = p0::P0_20<MODE>;
#[cfg(feature = "v2")]
pub type EDGE12<MODE> = p0::P0_12<MODE>;
pub type EDGE13<MODE> = PAD3<MODE>; // <- big pad 3
pub type EDGE14<MODE> = SCK<MODE>;
pub type EDGE15<MODE> = MISO<MODE>;
pub type EDGE16<MODE> = MOSI<MODE>;
#[cfg(feature = "v1")]
pub type EDGE17<MODE> = p0::P0_16<MODE>;
#[cfg(feature = "v2")]
pub type EDGE17<MODE> = p1::P1_02<MODE>;
// EDGE18 -> +V
// EDGE19 -> +V
// EDGE20 -> +V
pub type EDGE21 = SCL;
pub type EDGE22 = SDA;
// EDGE23 -> GND
// EDGE24 -> GND
// EDGE25 -> GND
