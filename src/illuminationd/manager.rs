use std::{
    collections::HashMap,
    fs,
    io::{Read, Seek, SeekFrom},
    path::Path,
    sync::Mutex,
};

use crate::dirs::LedEntry;

pub struct LedBrightnessSettings {}

pub struct LedColorTemperatureSettings {}

pub struct LedSettings {
    path: String,
    on: bool,
    brightness: u32,
    color_temperature: Option<u32>,
}

#[derive(Default)]
pub struct Lights {
    leds: Mutex<HashMap<u64, LedSettings>>,
}

fn read_u32<P>(filename: P) -> Option<u32>
where
    P: AsRef<Path>,
{
    let mut ret = std::fs::File::open(filename);
    if ret.is_err_and(|b| -> bool {}) {
        return None;
    }
    let mut file = ret.unwrap();
    let mut buffer = [0; 4];
    file.seek(SeekFrom::Start(0)).unwrap();
    file.read(&mut buffer[..]).unwrap() as u32;

    return Some(u32::from_be_bytes(buffer));
}

impl Lights {
    pub fn new() -> Lights {
        /*
        let map = |d: LedEntry| -> (u64, LedSettings) {
            let brightness = read_u32(Path::new(d.path).join("brightness")).unwrap();
            let color_temperature = read_u32(Path::new(d.path).join("color_temp"));

            let on = brightness != 0;
            (
                d.ino,
                LedSettings {
                    path: path,
                    on: on,
                    brightness: brightness + 1,
                    color_temperature: color_temperature,
                },
            )
        };
        */
        Lights {
            leds: Mutex::new(leds.dirs.map(map).collect()),
        }
    }

    pub fn get_settings(&self, light: u128) -> &LedSettings {
        let m = self.leds.lock().unwrap();
        let led = m.get(&(light as u64));
        if led.is_none() {
            // TODO: Try reload from filesystem
        }
        led.unwrap()
    }

    pub fn set_settings(
        &self,
        light: u128,
        on: Option<bool>,
        brightness: Option<u32>,
        color_temperature: Option<u32>,
    ) {
        self.set_led(light as u64, on, brightness, color_temperature);
    }

    fn set_led(
        &self,
        light: u64,
        on: Option<bool>,
        brightness: Option<u32>,
        color_temperature: Option<u32>,
    ) {
        let m = self.leds.lock().unwrap();
        let led_opt = m.get(&light);
        if led_opt.is_none() {
            // TODO: Try reload from filesystem
        }

        let led = led_opt.unwrap();
        // Store values to settings.
        if brightness.is_some() {
            led.brightness = brightness.unwrap();
        }
        if color_temperature.is_some() {
            led.color_temperature = color_temperature;
        }

        let led_path = Path::new(led.path.as_str());
        // sync state to filesystem
        if on == Some(false) {
            // turn device off

            fs::write(led_path.join("brightness"), "0");
        } else {
            fs::write(led_path.join("brightness"), led.brightness.to_string());
        }

        if led.color_temperature.is_some() {
            fs::write(
                led_path.join("/color_temp"),
                led.color_temperature.unwrap().to_string(),
            );
        }
    }
}
