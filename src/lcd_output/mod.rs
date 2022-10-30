#![no_std]
#![no_main]

mod pico_lib;

use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};
// use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;
use rp2040_hal as hal;

use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    watchdog::Watchdog,
    Sio, prelude::_rphal_pio_PIOExt,
};


#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;


fn lcd_main() -> ! {
    
    
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let mut delay_lcd = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().raw());

    
    let mut current_lcd_address: u8 = 0;
    
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // let mut led_pin = pins.gpio25.into_push_pull_output();

    // let mut i2c_lcd = i2c_pio::I2C::new(&mut pio, pins.gpio26, pins.gpio27, sm0, fugit::HertzU32::Hz(100_000), clocks.system_clock.freq());

    // let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().raw());
    loop {

        // let lcd_result = lcd_lcm1602_i2c::Lcd::new(&mut i2c_lcd, &mut delay_lcd)
        // .address(current_lcd_address)
        // .cursor_on(false) // no visible cursos
        // .rows(2) // two rows
        // .init();
        
        // if let Ok(mut lcd) = lcd_result {
        //     if let Ok(()) = lcd.set_cursor(0, 0) {
        //         let _ = lcd.write_str("Hello!");
        //     }
            
        // }

        // lcd.init();
        
        // delay.delay_ms(500);
        // led_pin.set_low().unwrap();
        // lcd.write_str("low!");
        
        // delay.delay_ms(500);

        // we add debug output to the lcd display.

        
        if led_pin.is_set_high().unwrap() {
            led_pin.set_low().unwrap();
        } else {
            led_pin.set_high().unwrap();
        }

        delay.delay_ms(500);
        
        // i2c has 128 adresses
        // https://www.robot-electronics.co.uk/i2c-tutorial#:~:text=All%20I2C%20addresses%20are%20either,be%20from%200%20to%20127.
        // we iterate the whole address space so we possible find the correct onw.
        // if current_lcd_address < 127 {
        //     current_lcd_address += 1;
        // } else {
        //     current_lcd_address = 0;
        // }

    }
}
