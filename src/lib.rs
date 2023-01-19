#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};
use pwm_pca9685::{Channel, Pca9685};

#[derive(Debug)]
pub enum Error<E> {
    //Overflow,
    Pca(pwm_pca9685::Error<E>),
}

impl<E> From<pwm_pca9685::Error<E>> for Error<E> {
    fn from(error: pwm_pca9685::Error<E>) -> Self {
        Error::Pca(error)
    }
}

#[derive(Debug)]
pub struct MotorShield<I2C> {
    pwm: Pca9685<I2C>,
    motors: [(Channel, Channel); 4],
}

impl<I2C, E> MotorShield<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(i2c: I2C) -> Result<MotorShield<I2C>, Error<E>> {
        let mut motorshield = MotorShield {
            pwm: Pca9685::new(i2c, 0x60)?,
            motors: [
                (Channel::C8, Channel::C9),
                (Channel::C13, Channel::C12),
                (Channel::C10, Channel::C11),
                (Channel::C15, Channel::C14),
            ],
        };

        motorshield.pwm.set_prescale(100)?;
        motorshield.pwm.enable()?;

        Ok(motorshield)
    }

    pub fn ultimate_move(&mut self, powers: [f32; 4]) -> Result<(), Error<E>> {
        for (i, _) in powers.iter().enumerate() {
            self.set_power(
                self.motors[i],
                (
                    if powers[i] > 0.0 {
                        crate::convert(powers[i])
                    } else {
                        0
                    },
                    if powers[i] < 0.0 {
                        crate::convert(-powers[i])
                    } else {
                        0
                    },
                ),
            )?;
        }
        Ok(())
    }

    fn set_power(
        &mut self,
        channels: (Channel, Channel),
        powers: (u16, u16),
    ) -> Result<(), Error<E>> {
        self.pwm.set_channel_on(channels.0, 0)?;
        self.pwm.set_channel_off(channels.0, powers.0)?;
        self.pwm.set_channel_on(channels.1, 0)?;
        self.pwm.set_channel_off(channels.1, powers.1)?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error<E>> {
        self.ultimate_move([0.0; 4])?;
        Ok(())
    }

    pub fn brake(&mut self) -> Result<(), Error<E>> {
        for motor in self.motors {
            self.set_power(motor, (4095, 4095))?;
        }
        Ok(())
    }
}

// convert power in pourcentage TO MODIFY
fn convert(power: f32) -> u16 {
    if power > 100.0 {
        4095
    } else {
        (power * 4095.0 / 100.0) as u16
    }
}
