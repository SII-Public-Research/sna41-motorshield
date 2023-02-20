use embedded_hal::blocking::i2c::{Write, WriteRead};
use pwm_pca9685::Channel;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use crate::{convert, Error, MotorShield};

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum ServoNumber {
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
}

impl ServoNumber {
    fn get_channel(self) -> Channel {
        match self {
            Self::S1 => Channel::C0,
            Self::S2 => Channel::C1,
            Self::S3 => Channel::C2,
            Self::S4 => Channel::C3,
            Self::S5 => Channel::C4,
            Self::S6 => Channel::C5,
            Self::S7 => Channel::C6,
            Self::S8 => Channel::C7,
        }
    }
}

impl<I2C, E> MotorShield<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Set all 8 servo motors with given power in a scale from 0 (off) to 100 (fullspeed)
    pub fn set_all_servos(&mut self, powers: [f32; 8]) -> Result<(), Error<E>> {
        for (i, servo) in ServoNumber::iter().enumerate() {
            self.set_power(servo.get_channel(), convert(powers[i]))?;
        }
        Ok(())
    }

    /// Set servo motor with given power from 0 (off) to 100 (fullspeed)
    pub fn set_servo(&mut self, servo: ServoNumber, power: f32) -> Result<(), Error<E>> {
        self.set_power(servo.get_channel(), convert(power))?;
        Ok(())
    }
}
