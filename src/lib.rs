//! SNA41 Motorhield and Servo-motors driver.
//!
//! It aims to provide ready-to-use interface for building robots with 4 wheels and servos.
//!
//! You can also find a PS2 controller on the shield that is for now not provided on thi crate.
//!
//!
//!
//!
//!
#![no_std]

pub mod motors;
pub mod servo;

use core::fmt::{Debug, Formatter, Result as fmtResult};
use embedded_hal::blocking::i2c::{Write, WriteRead};
use pwm_pca9685::{Channel, Pca9685};

pub enum Error<E> {
    PcaError(pwm_pca9685::Error<E>),
    MotorError(i8),
    ServoError(i8),
}

impl<E> Debug for Error<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        match self {
            Error::PcaError(error) => write!(f, "PcaError( {:?})", error),
            Error::MotorError(error) => write!(f, "Error from motor : ( {:?})", error),
            Error::ServoError(error) => match error {
                0 => write!(f, "Error from servo : Specified angle value is not valid (0 - 180)"),
                _ => write!(f, "Error from servo : unknown"),
            } 
        }  
    }
}

#[derive(Debug)]
pub struct MotorShield<I2C> {
    pca: Pca9685<I2C>,
}

impl<I2C, E> MotorShield<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(i2c: I2C) -> Result<MotorShield<I2C>, Error<E>> {
        let mut motorshield = MotorShield {
            pca: Pca9685::new(i2c, 0x60).map_err(Error::PcaError)?,
        };

        motorshield.pca.set_prescale(100).map_err(Error::PcaError)?;
        motorshield.pca.enable().map_err(Error::PcaError)?;

        Ok(motorshield)
    }

    // Set the 'channel' pca to be up from 0 to 'power'
    fn set_power(&mut self, channel: Channel, power: u16) -> Result<(), Error<E>> {
        self.pca
            .set_channel_on(channel, 0)
            .map_err(Error::PcaError)?;
        self.pca
            .set_channel_off(channel, power)
            .map_err(Error::PcaError)?;
        Ok(())
    }
}

// convert number from 0-100 to 0-4096
fn convert(power: f32) -> u16 {
    if power > 100.0 {
        4095
    } else {
        (power * 4095.0 / 100.0) as u16
    }
}
