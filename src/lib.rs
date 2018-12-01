//! USB peripheral driver for STM32F103 microcontrollers.
//!
//! This also serves as the reference implementation and example repository for the `usb-device`
//! crate for now.

#![no_std]
#![feature(asm)]

use bare_metal;
use cortex_m;
// use stm32f103xx;
use stm32l4_hal; // as hal;
use vcell;
use usb_device;

mod endpoint;

mod atomic_mutex;

mod freezable_ref_cell;

/// USB peripheral driver.
pub mod bus;

pub use crate::bus::UsbBus;
