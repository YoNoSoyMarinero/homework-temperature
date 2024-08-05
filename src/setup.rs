use hal::gpio::{Pin, PinMode, Port};

pub fn setup_pins() {
    let sck = Pin::new(Port::A, 5, PinMode::Alt(5)); 
    let miso = Pin::new(Port::A, 6, PinMode::Alt(5));
    let mosi: Pin = Pin::new(Port::A, 7, PinMode::Alt(5));
}
