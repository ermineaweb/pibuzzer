use rppal::gpio::OutputPin;
use std::{ thread::sleep, time::Duration};

pub mod melodies;

pub struct Buzzer {
    pub pin_buzzer: OutputPin,
    pub pin_led: Option<OutputPin>,
    pub sound: bool,
    pub light: bool,
}

impl Buzzer {
    pub fn new(pin_buzzer: OutputPin, sound: bool, pin_led: Option<OutputPin>) -> Buzzer {
        let light = match &pin_led {
            Some(_) => true,
            None => false,
        };

        Buzzer {
            pin_buzzer,
            pin_led,
            sound,
            light,
        }
    }

    pub fn play_melody(&mut self, melody: &Melody) {
        // the duration of notes changes with the tempo
        // each note have a relative duration to the whole note
        let whole_note_duration = 60.0 * 4.0 / melody.tempo as f32;

        for sound in melody.sounds.iter() {
            let duration = sound.note_value.relative_value() * whole_note_duration;
            self.play_note(&sound.note, duration);
        }
    }

    pub fn play_note(&mut self, note: &Note, duration: f32) {
        let delay_between_notes_ms = 20;

        println!("{} {:.2}", note.to_str(), duration);

        let led = self.pin_led.as_mut().unwrap();

        if self.light {
            led.set_high();
        }

        if self.sound {
            let duty_cycle = if note.is_rest { 0.0 } else { 0.5 };
            self.pin_buzzer
                .set_pwm_frequency(note.freq(), duty_cycle)
                .expect("error");
        }

        sleep(Duration::from_millis(
            (duration * 1000.0) as u64 - delay_between_notes_ms,
        ));

        if self.light {
            led.set_low();
        }

        if self.sound {
            self.pin_buzzer.set_pwm_frequency(50.0, 0.0).expect("error");
        }

        sleep(Duration::from_millis(delay_between_notes_ms));
    }

    pub fn play_metronome(&mut self, tempo: u16, count: u16) {
        let sound_high = Sound::new(Note::new(Head::A, Octave::O3), NoteValue::Quarter);
        let sound_low = Sound::new(Note::new(Head::A, Octave::O4), NoteValue::Quarter);
        let whole_note_duration = 60.0 * 4.0 / tempo as f32;
        let note_duration = sound_high.note_value.relative_value() * whole_note_duration;
        let mut n = 0;
        let delay_between_notes_ms = 50;

        loop {
            if n % count == 0 {
                let freq = sound_high.note.freq();

                println!("metronome - {}hz - {}s", freq, note_duration);
                self.pin_buzzer.set_pwm_frequency(freq, 0.5).expect("error");
                sleep(Duration::from_millis(delay_between_notes_ms));

                self.pin_buzzer.set_pwm_frequency(50.0, 0.0).expect("error");
                sleep(Duration::from_millis(
                    (note_duration * 1000.0) as u64 - delay_between_notes_ms,
                ));
            } else {
                let freq = sound_low.note.freq();

                println!("metronome - {}hz - {}s", freq, note_duration);
                self.pin_buzzer.set_pwm_frequency(freq, 0.5).expect("error");
                sleep(Duration::from_millis(delay_between_notes_ms));

                self.pin_buzzer.set_pwm_frequency(50.0, 0.0).expect("error");
                sleep(Duration::from_millis(
                    (note_duration * 1000.0) as u64 - delay_between_notes_ms,
                ));
            }
            n += 1;
        }
    }
}

pub struct Melody {
    pub tempo: u16,
    pub sounds: Vec<Sound>,
}

impl Melody {
    pub fn new(tempo: u16, sounds: Vec<Sound>) -> Melody {
        Melody { tempo, sounds }
    }

    pub fn from_arrays(tempo: u16, notes: Vec<Note>, notes_value: Vec<NoteValue>) -> Melody {
        let mut sounds = vec![];

        for (index, note) in notes.into_iter().enumerate() {
            sounds.push(Sound::new(note, notes_value[index]));
        }
        Melody { tempo, sounds }
    }

    pub fn from_str(tempo: u16, str: &str) -> Melody {
        let clean_string = str
            .trim()
            .replace(" ", "")
            .replace("\n", "")
            .replace("\t", "");

        let notes_splitted = clean_string.split(",");

        let mut notes = vec![];
        let mut notes_value = vec![];

        for (index, parse) in notes_splitted.enumerate() {
            if index % 2 == 0 {
                notes.push(Note::from_str(parse));
            } else {
                notes_value.push(NoteValue::from_str(parse));
            }
        }

        Self::from_arrays(tempo, notes, notes_value)
    }
}

pub struct Sound {
    pub note: Note,
    pub note_value: NoteValue,
}

impl Sound {
    pub fn new(note: Note, note_value: NoteValue) -> Sound {
        Sound { note, note_value }
    }
}

#[derive(Clone, Copy)]
pub enum NoteValue {
    Large,
    Long,
    DoubleWhole,
    DoubleWholeDotted,
    Whole,
    WholeDotted,
    Half,
    HalfDotted,
    Quarter,
    QuarterDotted,
    Eighth,
    EighthDotted,
    Sixteenth,
    SixteenthDotted,
    ThirtySecond,
    ThirtySecondDotted,
    SixtyFourth,
    HundredTwentyEighth,
    TwoHundredFiftySixth,
}

impl NoteValue {
    fn relative_value(&self) -> f32 {
        match *self {
            NoteValue::Large => 8.0,
            NoteValue::Long => 4.0,
            NoteValue::DoubleWhole => 2.0,
            NoteValue::DoubleWholeDotted => 2.0 * 1.5,
            NoteValue::Whole => 1.0,
            NoteValue::WholeDotted => 1.0 * 1.5,
            NoteValue::Half => 1.0 / 2.0,
            NoteValue::HalfDotted => 1.0 / 2.0 * 1.5,
            NoteValue::Quarter => 1.0 / 4.0,
            NoteValue::QuarterDotted => 1.0 / 4.0 * 1.5,
            NoteValue::Eighth => 1.0 / 8.0,
            NoteValue::EighthDotted => 1.0 / 8.0 * 1.5,
            NoteValue::Sixteenth => 1.0 / 16.0,
            NoteValue::SixteenthDotted => 1.0 / 16.0 * 1.5,
            NoteValue::ThirtySecond => 1.0 / 32.0,
            NoteValue::ThirtySecondDotted => 1.0 / 32.0 * 1.5,
            NoteValue::SixtyFourth => 1.0 / 64.0,
            NoteValue::HundredTwentyEighth => 1.0 / 128.0,
            NoteValue::TwoHundredFiftySixth => 1.0 / 256.0,
        }
    }

    fn from_str(str: &str) -> NoteValue {
        match str {
            "1" => NoteValue::Whole,
            "-1" => NoteValue::WholeDotted,
            "2" => NoteValue::Half,
            "-2" => NoteValue::HalfDotted,
            "4" => NoteValue::Quarter,
            "-4" => NoteValue::QuarterDotted,
            "8" => NoteValue::Eighth,
            "-8" => NoteValue::EighthDotted,
            "16" => NoteValue::Sixteenth,
            "-16" => NoteValue::SixteenthDotted,
            "32" => NoteValue::ThirtySecond,
            "-32" => NoteValue::ThirtySecondDotted,
            "64" => NoteValue::SixtyFourth,
            _ => todo!("NoteValue from_str"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Head {
    // relative position from A
    C = -9,
    Cd,
    D,
    Dd,
    E,
    F,
    Fd,
    G,
    Gd,
    A,
    Ad,
    B,
}

impl Head {
    fn from_str(str: &str) -> Head {
        match str {
            "C" => Head::C,
            "Cd" => Head::Cd,
            "Db" => Head::Cd,
            "D" => Head::D,
            "Dd" => Head::Dd,
            "Eb" => Head::Dd,
            "E" => Head::E,
            "F" => Head::F,
            "Fd" => Head::Fd,
            "Gb" => Head::Fd,
            "G" => Head::G,
            "Gd" => Head::Gd,
            "Ab" => Head::Gd,
            "A" => Head::A,
            "Ad" => Head::Ad,
            "Bb" => Head::Ad,
            "B" => Head::B,
            _ => todo!("Head from_str"),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Head::C => "C",
            Head::Cd => "Cd",
            Head::D => "D",
            Head::Dd => "Dd",
            Head::E => "E",
            Head::F => "F",
            Head::Fd => "Fd",
            Head::G => "G",
            Head::Gd => "Gd",
            Head::A => "A",
            Head::Ad => "Ad",
            Head::B => "B",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Octave {
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
}

impl Octave {
    fn from_str(str: &str) -> Octave {
        match str {
            "0" => Octave::O0,
            "1" => Octave::O1,
            "2" => Octave::O2,
            "3" => Octave::O3,
            "4" => Octave::O4,
            "5" => Octave::O5,
            "6" => Octave::O6,
            "7" => Octave::O7,
            "8" => Octave::O8,
            "9" => Octave::O9,
            _ => todo!("Octave from_str"),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Octave::O0 => "0",
            Octave::O1 => "1",
            Octave::O2 => "2",
            Octave::O3 => "3",
            Octave::O4 => "4",
            Octave::O5 => "5",
            Octave::O6 => "6",
            Octave::O7 => "7",
            Octave::O8 => "8",
            Octave::O9 => "9",
        }
    }
}

pub struct Note {
    head: Head,
    octave: Octave,
    is_rest: bool,
}

impl Note {
    pub fn new(head: Head, octave: Octave) -> Note {
        Note {
            head,
            octave,
            is_rest: false,
        }
    }

    pub fn new_rest() -> Note {
        // todo find a better way to describe a REST
        Note {
            head: Head::A,
            octave: Octave::O0,
            is_rest: true,
        }
    }

    pub fn freq(&self) -> f64 {
        // A octave 0 = 27.50 hz
        let position_relative_to_ao0 = (self.head as i16 + (self.octave as i16) * 12) as f64;
        27.50 * 2.0_f64.powf(position_relative_to_ao0 / 12.0)
    }

    pub fn from_str(str: &str) -> Note {
        let note = match str.chars().count() {
            2 => Note::new(Head::from_str(&str[..1]), Octave::from_str(&str[1..])),
            3 => Note::new(Head::from_str(&str[..2]), Octave::from_str(&str[2..])),
            _ => Note::new_rest(),
        };

        note
    }

    pub fn to_str(&self) -> String {
        if self.is_rest {
            return "REST".to_string();
        }
        format!("{}{}", self.head.to_str(), self.octave.to_str())
    }
}
