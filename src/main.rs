#![no_main]
#![no_std]

use cortex_m::{self};
use cortex_m_rt::entry;
use defmt_rtt as _;
use hal::gpio::{
    Pin,
    Port,
    PinMode
};
use hal::pac::{Peripherals, SPI1};
use hal::spi::{Spi, BaudRate};
mod init;
mod setup;
mod system_status;



#[entry]
fn main() -> ! {

    init::run();
    let dp = Peripherals::take().unwrap();
    //Setting up chip select and SPI communication
    let mut cs = Pin::new(Port::A, 4, PinMode::Alt(5));
    let mut spi: Spi<SPI1> = Spi::new(dp.SPI1,
        Default::default(),
        BaudRate::Div256);
    loop {
    }
}


#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
