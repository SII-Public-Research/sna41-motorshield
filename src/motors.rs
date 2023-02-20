use embedded_hal::blocking::i2c::{Write, WriteRead};
use pwm_pca9685::Channel;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use crate::{convert, Error, MotorShield};

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum MotorNumber {
    M1,
    M2,
    M3,
    M4,
}

impl MotorNumber {
    fn get_channels(self) -> (Channel, Channel) {
        match self {
            Self::M1 => (Channel::C8, Channel::C9),
            Self::M2 => (Channel::C13, Channel::C12),
            Self::M3 => (Channel::C10, Channel::C11),
            Self::M4 => (Channel::C15, Channel::C14),
        }
    }
}

impl<I2C, E> MotorShield<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Set all 4 motors power in a scale from 0 (off) to 100 (fullspeed)
    /// front_left, front_right, back_left, back_right
    pub fn set_all_motors(&mut self, powers: [f32; 4]) -> Result<(), Error<E>> {
        for (i, motor) in MotorNumber::iter().enumerate() {
            self.set_powers(motor.get_channels(), convert_power(powers[i]))?;
        }
        Ok(())
    }

    /// Set motor with given power from 0 (off) to 100 (fullspeed)
    /// 1: front_left, 2: front_right, 3: back_left, 4: back_right
    pub fn set_motor(&mut self, motor: MotorNumber, power: f32) -> Result<(), Error<E>> {
        self.set_powers(motor.get_channels(), convert_power(power))?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error<E>> {
        self.set_all_motors([0.0; 4])?;
        Ok(())
    }

    pub fn brake(&mut self) -> Result<(), Error<E>> {
        for motor in MotorNumber::iter() {
            self.set_powers(motor.get_channels(), (4095, 4095))?;
        }
        Ok(())
    }

    fn set_powers(
        &mut self,
        channels: (Channel, Channel),
        powers: (u16, u16),
    ) -> Result<(), Error<E>> {
        self.set_power(channels.0, powers.0)?;
        self.set_power(channels.1, powers.1)?;
        Ok(())
    }
}

// convert power from 0-100 to a pair of 0-4096
fn convert_power(power: f32) -> (u16, u16) {
    (
        if power > 0.0 { convert(power) } else { 0 },
        if power < 0.0 { convert(-power) } else { 0 },
    )
}

/* PLACEMENT TO BE ADDDED SI ON A ENVIE */

/*
pub enum MotorPlacement {
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight,
}

impl MotorNumber {
    pub fn get_placement(self) -> MotorPlacement {
        match self {
            Self::M1 => MotorPlacement::FrontLeft,
            Self::M2 => MotorPlacement::FrontRight,
            Self::M3 => MotorPlacement::BackLeft,
            Self::M4 => MotorPlacement::BackRight,
        }
    }
}

impl MotorPlacement {
    pub fn get_number(self) -> MotorNumber {
        match self {
            Self::FrontLeft => MotorNumber::M1,
            Self::FrontRight => MotorNumber::M2,
            Self::BackLeft => MotorNumber::M3,
            Self::BackRight => MotorNumber::M4,
        }
    }
}


impl<I2C, E> MotorShield<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Set motor with given power from 0 (off) to 100 (fullspeed)
    /// 1: front_left, 2: front_right, 3: back_left, 4: back_right
    pub fn set_motor_by_placement(
        &mut self,
        motor: MotorPlacement,
        power: f32,
    ) -> Result<(), Error<E>> {
        self.set_powers(motor.get_number().get_channels(), convert_power(power))?;
        Ok(())
    }
}

*/
