use rppal::i2c::I2c;
use sna41_motorshield::{motors::MotorNumber, servo::ServoNumber, MotorShield};

fn main() {
    // Create motorshield
    let i2c = I2c::new().expect("1");
    let mut ms = MotorShield::new(i2c).expect("I2C not detected.");

    // set a motor from its number
    ms.set_motor(MotorNumber::M1, 75.).unwrap();
    // or set everything at once
    ms.set_all_motors([100., 100., 0., 0.]).unwrap();

    // you can brake
    ms.brake().unwrap();
    // and stop
    ms.stop().unwrap();

    // you can use the servo motors too
    ms.set_servo(ServoNumber::S1, 33.3).unwrap();
}
