//!
//! # Buzzer Type Definitions
//!

use crate::hal::{time::hz, timer};

use simple_pwm::SimplePwm as PWM;
use timer::GeneralInstance4Channel as TIM;
use timer::{Channel, simple_pwm};

pub struct Buzzer<'t, P: TIM> {
    pwm: PWM<'t, P>,
    channel: Channel,
}

impl<'t, P: TIM> Buzzer<'t, P> {
    pub const fn new(pwm: PWM<'t, P>, ch: Channel) -> Buzzer<'t, P> {
        Self { pwm, channel: ch }
    }
}

impl<P: TIM> Buzzer<'_, P> {
    pub fn enable(&mut self) {
        let ch = self.channel;
        let beep = &mut self.pwm;
        let mut buzzer: _ = beep.channel(ch);
        buzzer.set_duty_cycle_fully_off();
        buzzer.enable();
    }

    pub fn disable(&mut self) {
        let ch = self.channel;
        self.pwm.channel(ch).disable();
    }

    pub fn set(&mut self, freq_hz: u32) {
        let ch = self.channel;
        let beep = &mut self.pwm;
        if freq_hz == 0 {
            beep.channel(ch).set_duty_cycle_fully_off();
            return;
        }
        beep.set_frequency(hz(freq_hz));
        beep.channel(ch).set_duty_cycle_percent(50);
    }
}

impl<P: TIM> Buzzer<'_, P> {
    pub async fn note(&mut self, freq_hz: u32, dura_ms: u32) {
        self.enable();
        self.set(freq_hz);
        utils::T::after_millis(dura_ms as _).await;
        self.disable();
    }

    pub async fn play(&mut self, tone: &[(u32, u16)]) {
        self.enable();
        for &(f, d) in tone {
            self.set(f);
            utils::T::after_millis(d as _).await;
        }
        self.disable();
    }
}
