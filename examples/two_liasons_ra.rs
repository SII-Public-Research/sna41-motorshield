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
    // let mut ms = MotorShield::new(i2c).expect("I2C not detected.");
    // let address = Address::default();
    let mut pwm = Pca9685::new(i2c, 0x60).unwrap();

    pwm.set_prescale(100).unwrap();
    pwm.enable().unwrap();

    loop {
        pwm.set_channel_on_off(Channel::C0, 0, 3000).unwrap();
        delay.delay_ms(2000_u64);
        pwm.set_channel_on_off(Channel::C0, 0, 1000).unwrap();
        delay.delay_ms(2000_u64);
        pwm.set_channel_on_off(Channel::C0, 0, 3000).unwrap();
        delay.delay_ms(2000_u64);
        // pwm.set_channel_on_off(Channel::C0, 3000, 0).unwrap();
        // delay.delay_ms(2000_u64);
        pwm.set_channel_on_off(Channel::C0, 0, 0).unwrap();
        delay.delay_ms(2000_u64);
    }
    
}
