#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m;
use cortex_m::asm::bkpt;
//#[macro_use]
use cortex_m_rt as rt;
use panic_semihosting;
use stm32l4_hal as hal;
use usb_device;
use stm32l432_usb;

use cortex_m_semihosting::hio;
use core::fmt::Write;

use self::hal::prelude::*;
use self::hal::stm32;
// use self::rt::ExceptionFrame;

use usb_device::prelude::*;
use stm32l432_usb::UsbBus;

mod cdc_acm;

#[entry]
fn main() -> ! {
    let cp = hal::CorePeripherals::take().unwrap();
    let dp = hal::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        // f103
        // .use_hse(8.mhz())
        // .sysclk(48.mhz())
        // .pclk1(24.mhz())

        // l432
        .hsi48(true)  // needed for RNG + USB
        // .sysclk(64.mhz())
        // .pclk1(32.mhz())

        .freeze(&mut flash.acr);

    bkpt();

    // assert!(clocks.usbclk_valid());

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    let usb_bus = UsbBus::usb(dp.USB, &mut rcc.apb1r1);
    usb_bus.init(|b| b.enable_reset(
        &clocks,
        &mut gpioa.moder,
        &mut gpioa.otyper,
        gpioa.pa12
    ));

    let serial = cdc_acm::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDevice::new(&usb_bus, UsbVidPid(0x5824, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(cdc_acm::USB_CLASS_CDC)
        .build(&[&serial]);

    bkpt();
    usb_dev.force_reset().expect("reset failed");

    loop {
        usb_dev.poll();
        let mut stdout = hio::hstdout().unwrap();
        writeln!(stdout, "usb_dev.state() = {:?}", usb_dev.state()).ok();

        if usb_dev.state() == UsbDeviceState::Configured {
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
}

// exception!(HardFault, hard_fault);
// fn hard_fault(ef: &ExceptionFrame) -> ! {
//     panic!("{:#?}", ef);
// }

// exception!(*, default_handler);
// fn default_handler(irqn: i16) {
//     panic!("Unhandled exception (IRQn = {})", irqn);
// }
