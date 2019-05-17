#![no_std]
#![no_main]

// extern crate panic_halt;
extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32l4xx_hal::{prelude::*, stm32};

use usb_device::prelude::*;
use stm32l43x_usbd::UsbBus;

mod ccid;
mod cdc_acm;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .hsi48(true)
        .freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    let usb_dp = gpioa.pa12.into_af10(&mut gpioa.moder, &mut gpioa.afrh);

    // disable Vddusb power isolation
    let pwr = dp.PWR.constrain(&mut rcc.apb1r1); // turns it on
    pwr.enable_usb();

    let usb_bus = UsbBus::usb_with_reset(dp.USB,
        &mut rcc.apb1r1, &clocks, &mut gpioa.moder, &mut gpioa.otyper, usb_dp);

    let mut smartcard = ccid::SmartCard::new(&usb_bus);
    // let mut smartcard2 = ccid::SmartCard::new(&usb_bus);
    let mut serial = cdc_acm::SerialPort::new(&usb_bus);
    // let mut serial2 = cdc_acm::SerialPort::new(&usb_bus);

    // vid/pid: http://pid.codes/1209/CC1D/
    let mut usb_dev = UsbDeviceBuilder::new(
            &usb_bus,
            UsbVidPid(0x1209, 0xCC1D),
        )
        .manufacturer("HardcoreBits")
        .product("Zissou")
        .serial_number("N/a")
        // .device_class(ccid::USB_CLASS_NONE)
        .build();

    // usb_dev.force_reset().expect("reset failed");

    loop {
        if !usb_dev.poll(&mut [&mut smartcard, &mut serial]) {
        // if !usb_dev.poll(&mut [&mut serial, &mut smartcard]) {
        // if !usb_dev.poll(&mut [&mut serial, &mut serial2]) {
        // if !usb_dev.poll(&mut [&mut smartcard, &mut smartcard2]) {
        // if !usb_dev.poll(&mut [&mut smartcard]) {
        // if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        // match serial.read(&mut buf) {
        //     Ok(count) if count > 0 => {
        //         // Echo back in upper case
        //         for c in buf[0..count].iter_mut() {
        //             if 0x61 <= *c && *c <= 0x7a {
        //                 *c &= !0x20;
        //             }
        //         }

        //         serial.write(&buf[0..count]).ok();
        //     },
        //     _ => { },
        // }
        // match serial2.read(&mut buf) {
        //     Ok(count) if count > 0 => {
        //         // Echo back in upper case
        //         // for c in buf[0..count].iter_mut() {
        //         //     if 0x61 <= *c && *c <= 0x7a {
        //         //         *c &= !0x20;
        //         //     }
        //         // }

        //         serial2.write(&buf[0..count]).ok();
        //     },
        //     _ => { },
        // }
    }
}

