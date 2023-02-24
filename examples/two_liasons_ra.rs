use std::ops::Add;

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
// test connexion

use sna41_motorshield::{servo::ServoNumber, MotorShield};

use pwm_pca9685::{Address, Channel, Pca9685};
use linux_embedded_hal::I2cdev;


fn main() {

    let i2c = I2cdev::new("/dev/i2c-1").expect("1");
    // let mut ms = MotorShield::new(i2c).expect("I2C not detected.");
    let address = Address::default();
    let mut pwm = Pca9685::new(i2c, address).unwrap();

    pwm.set_prescale(100).unwrap();

    pwm.enable().unwrap();

    pwm.set_channel_on(Channel::C0, 0).unwrap();

    pwm.set_channel_off(Channel::C0, 2047).unwrap();

    loop {
        // ms.set_servo(ServoNumber::S2, 30.0).unwrap();
        // delay.delay_ms(1000_u64);
        // ms.set_servo(ServoNumber::S2, 40.0).unwrap();
        // delay.delay_ms(1000_u64);


    }
    
}
