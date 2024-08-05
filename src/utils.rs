use hal::gpio::Pin;
use hal::pac::SPI1;
use hal::spi::Spi;
use heapless::spsc::Queue;

pub fn read_temperature(spi: &mut Spi<SPI1>, cs: &mut Pin) -> f32{
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


pub struct MovingAverage {
    pub sum: f32,
    pub count: usize,
    pub buffer: Queue<f32, 60>
}

impl MovingAverage {
    //constructor
    pub fn new() -> Self {
        Self {
            sum: 0.0,
            count: 0,
            buffer: Queue::new(),
        }
    }

    //adding new value to the queue
    pub fn add_last(&mut self, value: f32){
        if self.buffer.is_full(){
            if let Some(first_value) = self.buffer.dequeue(){
                self.sum -= first_value;
                self.count -= 1;
            }
        }
        self.buffer.enqueue(value).unwrap();
        self.sum += value;
        self.count += 1;
    }

    //calculate an average
    pub fn get_average(&self) -> f32 {
        if self.count == 0 {
            0.0
        }else {
            self.sum / self.count as f32
        }
    }
}