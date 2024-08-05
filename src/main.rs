

#![no_main]
#![no_std]

use cortex_m::{self};
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use hal::{self, low_power, pac};

mod init;
mod setup;
mod system_status;



#[entry]
fn main() -> ! {


    init::run();

    loop {
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
