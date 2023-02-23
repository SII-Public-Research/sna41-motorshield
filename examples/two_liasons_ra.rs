use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
// test connexion
use rppal::i2c::I2c;
use rppal::hal::Delay;
use sna41_motorshield::{servo::ServoNumber, MotorShield};


fn main() {

    let mut delay = Delay::new();

    let i2c = I2c::new().expect("1");
    let mut ms = MotorShield::new(i2c).expect("I2C not detected.");
    loop {
        ms.set_servo(ServoNumber::S2, 30.0).unwrap();
        delay.delay_ms(1000_u64);
        ms.set_servo(ServoNumber::S2, 40.0).unwrap();
        delay.delay_ms(1000_u64);
    }
    
}
