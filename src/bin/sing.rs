// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            Sing your own tune                             |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_mar_2025::music::{Note, OCTAVE};
use embassy_rp::pwm::{self, Pwm};
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;

// PWM config
use embassy_rp::{
    adc::InterruptHandler,
    config,
    pwm::{Config as ConfigPwm, SetDutyCycle},
};

// ADC config
use embassy_rp::adc::{Adc, Channel, Config as ConfigAdc};
// Use the `panic_probe` crate to provided the panic handler and the
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    /// Beats per minute.
    const TEMPO: u64 = 100;
    /// A whole note duration in milliseconds.
    const WHOLE_NOTE: u64 = 4 * (60_000 / TEMPO);
    /// The microcontroller clock frequency
    const CLOCK_FREQ: u64 = 150_000_000;
    /// PWM clock divider
    const PWM_DIV: u64 = 64;

    let peripherals = embassy_rp::init(Default::default());

    let mut config: ConfigPwm = Default::default();
    config.top = 0x9088; // in HEX, equals 37000 in decimal
    config.compare_b = config.top;

    let mut pwm = Pwm::new_output_b(peripherals.PWM_SLICE1, peripherals.PIN_3, config.clone());

    // TODO: Configure the PWM pin.
    let mut buzzer_cfg: ConfigPwm = Default::default();
    buzzer_cfg.top = 0x9088;
    buzzer_cfg.divider = PWM_DIV.to_fixed();
    buzzer_cfg.compare_a = buzzer_cfg.top / 2;
    let mut buzzer = Pwm::new_output_a(peripherals.PWM_SLICE6, peripherals.PIN_28, buzzer_cfg.clone());
    buzzer.set_config(&buzzer_cfg);

    for (note, length) in OCTAVE {
        // TODO: Compute the note's duration based on
        // the length variable.
        let duration = ();

        match note {
            Some(note) => {
                // TODO: Configure the `top` and `compare_X` registers
                // based on the note's type and change the PWM's config.
                // Keep in mind that we are aiming for a 50% duty cycle.
                // "Play" the note for 90% of the duration, then insert
                // a 10% pause before playing the next note.
            }
            None => {
                // TODO: Just wait the whole duration.
            }
        };
    }
}
