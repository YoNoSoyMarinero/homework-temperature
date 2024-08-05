#![no_main]
#![no_std]

use cortex_m::{self};
use cortex_m_rt::entry;
use defmt::println;
use defmt_rtt as _;
use hal::gpio::{
    Pin,
    Port,
    PinMode
};
use hal::pac::{Peripherals, SPI1};
use hal::spi::{Spi, BaudRate};
use utils::{read_temperature, MovingAverage};
use hal::delay_ms;
use panic_probe as _;

mod init;
mod setup;
mod system_status;
mod utils;


const AHB_FREQ: u32 = 80_000_000;
#[entry]
fn main() -> ! {

    init::run();
    let dp = Peripherals::take().unwrap();
    //Setting up chip select and SPI communication
    let mut cs = Pin::new(Port::A, 4, PinMode::Alt(5));
    let mut spi: Spi<SPI1> = Spi::new(dp.SPI1,
        Default::default(),
        BaudRate::Div256);

    let mut moving_average: MovingAverage = MovingAverage::new();

    loop {
        let current_temperature: f32 = read_temperature(&mut spi, &mut cs);
        moving_average.add_last(current_temperature);
        let average: f32 = moving_average.get_average();
        println!("Current average temperature for last minute is {}", average);
        delay_ms(1_000, AHB_FREQ);
    }
}


#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
