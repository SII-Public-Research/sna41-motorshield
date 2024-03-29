// servo is controled from 101 to 658

use std::ops::Add;

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
// test connexion
use rppal::i2c::I2c;
use rppal::hal::Delay;
use sna41_motorshield::{servo::ServoNumber, MotorShield};

use pwm_pca9685::{Address, Channel, Pca9685};


fn main() {

    let mut delay = Delay::new();

    let i2c = I2c::new().expect("1");
    let mut ms = MotorShield::new(i2c).expect("I2C not detected.");
    let address = Address::default();
    // let mut pwm = Pca9685::new(i2c, 0x60).unwrap();

    // pwm.set_prescale(100).unwrap();
    // pwm.enable().unwrap();

    loop {

        // valeure min
        // pwm.set_channel_on_off(Channel::C0, 0, 110).unwrap();
        // delay.delay_ms(5000_u64);
        // // valeure max
        // pwm.set_channel_on_off(Channel::C0, 0, 600).unwrap();
        // delay.delay_ms(5000_u64);

        ms.set_servo_angle(ServoNumber::S0, 0.0).unwrap();
        // ms.set_servo_angle(ServoNumber::S1, 50.0).unwrap();
        // ms.set_servo_angle(ServoNumber::S2, 90.0).unwrap();
        delay.delay_ms(2000_u64);
        ms.set_servo_angle(ServoNumber::S0, 90.0).unwrap();
        delay.delay_ms(2000_u64);
        // ms.set_servo_angle(ServoNumber::S1, 180.0).unwrap();
        // delay.delay_ms(2000_u64);
        // ms.set_servo_angle(ServoNumber::S1, 60.0).unwrap();
        // ms.set_servo_angle(ServoNumber::S2, 0.0).unwrap();
        // delay.delay_ms(2000_u64);
        // ms.set_servo_angle(ServoNumber::S2, 180.0).unwrap();
        // delay.delay_ms(2000_u64);
    }

}
