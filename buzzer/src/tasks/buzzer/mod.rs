//!
//! # Buzzer Task
//!

use crate::hal::{gpio, time::hz, timer};
use crate::system::*;

use gpio::OutputType::PushPull as Mode;
use low_level::CountingMode::EdgeAlignedUp;
use timer::simple_pwm::{PwmPin, SimplePwm};
use timer::{Channel, low_level};

mod canon;
mod typedef;

use canon::TUNE;
use typedef::Buzzer;

#[embassy_executor::task]
pub async fn task(p: BuzzerSrc) -> ! {
    // let mut t = utils::init_ticker!(1);

    let buzz_pin = PwmPin::new(p.buzz_pin, Mode);
    let beep_g = SimplePwm::new(
        p.tim_p,
        None,
        Some(buzz_pin),
        None,
        None,
        hz(1),
        EdgeAlignedUp,
    );

    let mut buzzer: _ = Buzzer::new(beep_g, Channel::Ch2);

    loop {
        // for &(f, d) in TUNE {
        //     Device::MasterController.wait(&mut t).await;
        //     buzzer.note(f, d as _).await;
        // }

        buzzer.play(TUNE).await;
    }
}
