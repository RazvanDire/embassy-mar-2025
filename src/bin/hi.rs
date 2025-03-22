// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            Hi, I am new here!                             |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

use embassy_executor::Spawner;
// Use the `panic_probe` crate to provided the panic handler and the 
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;
use embassy_rp::gpio::Input;
use embassy_rp::gpio::Pull;
use embassy_futures::select;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.

    let peripherals = embassy_rp::init(Default::default());

    let mut sw4_butt = Input::new(peripherals.PIN_0, Pull::Up);
    let mut sw5_butt = Input::new(peripherals.PIN_1, Pull::Up);
    let mut sw6_butt = Input::new(peripherals.PIN_2, Pull::Up);
    let mut sw7_butt = Input::new(peripherals.PIN_3, Pull::Up);

    loop {
        select::select4(sw4_butt.wait_for_falling_edge(), sw5_butt.wait_for_falling_edge(),
                        sw6_butt.wait_for_falling_edge(), sw7_butt.wait_for_falling_edge()).await;
        info!("We are team Creier on Steroids !");
    }
}
