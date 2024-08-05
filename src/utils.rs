use hal::gpio::Pin;
use hal::pac::SPI1;
use hal::spi::Spi;

fn read_temperature(spi: &mut Spi<SP1>, cs: &mut Pin) -> f32{
    const TEMP_LSB_CMD: u8 = 0x01;
    const TEMP_MSB_CMD: u8 = 0x02;
    cs.set_low(); 

    //send message to sensor and read response
    spi.write(&[TEMP_LSB_CMD]).unwrap();
    let lsb: u8 = spi.read().unwrap();
    spi.write(&[TEMP_MSB_CMD]).unwrap();
    let msb: u8 = spi.read().unwrap();

    //process the response
    let raw_temp: u16 = ((msb as u16) << 8) | (lsb as u16);
    let temperature: f32 = raw_temp as f32 * 0.0625;
    temperature
}