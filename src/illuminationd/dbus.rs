use std::{error::Error, future::pending};
use zbus::{dbus_interface, ConnectionBuilder};

struct ColorTemperatureSettings {
    lower: u32,
    value: u32,
    upper: u32,
}

// BrightnessSettings expose the light interface.
// For LEDs, it maps value = brightness+1 and
// max = max_brightness+1 since LED interface
// designates 0 to turning device off.
struct BrightnessSettings {
    value: u32,
    max: u32,
}

struct LightSettings {
    on: Option<bool>,
    brightness: Option<BrightnessSettings>,
    color_temperature: Option<ColorTemperatureSettings>,
}

struct LightsServer {
    manager: crate::manager::Lights,
}

#[dbus_interface(name = "org.freedesktop.LightsManager")]
impl LightsServer {
    // Can be `async` as well.
    fn get_lights(&mut self, name: &str) -> &[String] {
        self.count += 1;
        format!("Hello {}! I have been called {} times.", name, self.count)
    }

    fn get_light_settings(&self, light: u128) -> LightSettings {
        let settings = self.manager.get_settings(light);
        LightSettings {
            on: Some(settings.on),
            brightness: Some(settings.brightness),
            color_temperature: settings.color_temperature,
        }
    }

    fn set_light_settings(&mut self, light: u128, settings: LightSettings) {
        self.manager.set_settings(
            light = light,
            on = settings.on,
            brightness = settings.brightness,
            color_temperature = settings.color_temperature,
        );
    }
}

#[async_std::main]
pub async fn run_server(manager: crate::manager::Lights) -> Result<(), Box<dyn Error>> {
    let manager = LightsServer { manager };
    let _ = ConnectionBuilder::session()?
        .name("org.freedesktop.IlluminationManager")?
        .serve_at("/org/freedesktop/IlluminationManager", manager)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
