use pibuzzer::{melodies, Buzzer, Melody};
use rppal::gpio::Gpio;

const PIN_BUZZER_NUM: u8 = 23;
const PIN_LED_NUM: u8 = 24;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pin_led = Gpio::new()?.get(PIN_LED_NUM)?.into_output();
    let pin_buzzer = Gpio::new()?.get(PIN_BUZZER_NUM)?.into_output();
    let mut buzzer = Buzzer::new(pin_buzzer, true, Some(pin_led));

    // buzzer.play_metronome(120, 4);

    let melody1 = Melody::from_str(
        melodies::the_kesh::TEMPO,
        melodies::the_kesh::MELODY,
    );

    buzzer.play_melody(&melody1);

    Ok(())
}
