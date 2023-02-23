// test connexion
use rppal::i2c::I2c;
use sna41_motorshield::{servo::ServoNumber, MotorShield};


fn main() {

    let i2c = I2c::new().expect("1");
    let mut ms = MotorShield::new(i2c).expect("I2C not detected.");
    loop {
        ms.set_servo(ServoNumber::S2, 33.3).unwrap();
    }
    
}
