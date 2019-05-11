#![no_std]
#![no_main]

/// CDC-ACM serial port example using polling in a busy loop.

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32l4xx_hal::{prelude::*, stm32};

use usb_device::prelude::*;
use stm32l43x_usbd::UsbBus;

use stm32l4xx_hal::stm32::{USB};

mod cdc_acm;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        // .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .pclk2(24.mhz())
        .hsi48(true)
        .freeze(&mut flash.acr);

    // assert!(clocks.usbclk_valid());

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    let _usb_dm = gpioa.pa11.into_af10(&mut gpioa.moder, &mut gpioa.afrh);
    let usb_dp = gpioa.pa12.into_af10(&mut gpioa.moder, &mut gpioa.afrh);

    // disable Vddusb power isolation
    let pwr = dp.PWR.constrain(&mut rcc.apb1r1); // turns it on
    pwr.enable_usb();

    let usb_bus = UsbBus::usb_with_reset(dp.USB,
        &mut rcc.apb1r1, &clocks, &mut gpioa.moder, &mut gpioa.otyper, usb_dp);

    let mut serial = cdc_acm::SerialPort::new(&usb_bus);

    // let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x5824, 0x27dd))
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(cdc_acm::USB_CLASS_CDC)
        .build();


    // uuhhhh... this probably belongs in bus::Reset?
    unsafe {
        let bcdr = &(*USB::ptr()).bcdr;

        // bcdr.write(|w| w.bcden().set_bit().dcden().set_bit());
        // if bcdr.read().dcdet().bit_is_set() {
        //     bcdr.write(|w| w.bcden().set_bit().pden().set_bit());
        //     if bcdr.read().ps2det().bit_is_set() {
        //         // res = unk
        //     } else if bcdr.read().pdet().bit_is_set() {
        //         bcdr.write(|w| w.bcden().set_bit().sden().set_bit());
        //         if bcdr.read().sdet().bit_is_set() {
        //             // res = dcp
        //         } else {
        //             // res = cdp
        //         }
        //     } else {
        //         // res = sdp
        //     }
        // } else {
        //     // res = dsc
        // }

        bcdr.write(|w| w.dppu().set_bit());
    }

    usb_dev.force_reset().expect("reset failed");

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                serial.write(&buf[0..count]).ok();
            },
            _ => { },
        }
    }
}
