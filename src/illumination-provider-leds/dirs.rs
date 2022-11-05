use std::{
    fs::{self, DirEntry, File},
    io::{Error, Read},
    path::{Path, PathBuf},
};

pub struct LedEntry {
    pub ino: u64,
}

pub struct LedDir {
    path: String,
    has_color_temperature: bool,
}

impl LedDir {
    pub fn get_on(&self) -> bool {
        self.get_u32_from_file("brightness") != 0
    }

    pub fn get_max_brightness(&self) -> Option<u32> {
        let max_brightness = self.get_u32_from_file("max_brightness");
        if max_brightness == 1 {
            None
        } else {
            Some(max_brightness - 1)
        }
    }

    pub fn get_brightness(&self) -> Option<u32> {
        if self.get_max_brightness().is_none() {
            None
        } else {
            let raw_brightness = self.get_u32_from_file("brightness");
            Some(raw_brightness - 1)
        }
    }

    pub fn get_color_temperature(&self) -> Option<u32> {
        let max_color_temperature = self.get_max_color_temperature();
        if max_color_temperature.is_none() {
            return None
        }
        if max_color_temperature.unwrap() - self.get_min_color_temperature().unwrap() == 0 {
            return None
        }
        Some(self.get_u32_from_file("color_temp"))
    }

    pub fn get_min_color_temperature(&self) -> Option<u32> {
        self.try_get_u32_from_file("min_color_temp")
    }

    pub fn get_max_color_temperature(&self) -> Option<u32> {
        self.try_get_u32_from_file("max_color_temp")
    }

    fn get_u32_from_file(&self, filename: &str) -> u32 {
        let path = Path::new(&self.path).join(filename);
        let f = File::open(path).unwrap();
        let mut buf = [0; 4];
        f.read(&mut buf);
        u32::from_ne_bytes(buf)
    }

    fn try_get_u32_from_file(&self, filename: &str) -> Option<u32> {
        let path = Path::new(&self.path).join(filename);
        let f = File::open(path);
        if f.is_err() {
            return None
        }
        let mut buf = [0; 4];
        f.unwrap().read(&mut buf);
        Some(u32::from_ne_bytes(buf))
    }
}

pub struct Leds<I: Iterator<Item = LedEntry>> {
    pub dirs: I,
}

pub fn get_leds<P>(path: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let paths = fs::read_dir(path).unwrap();
    let map = |dir_res: Result<DirEntry, Error>| -> Option<PathBuf> {
        if dir_res.is_err() {
            return None;
        }
        let p = dir_res.unwrap();
        let file_type = p.file_type();
        if file_type.is_err() {
            return None;
        }
        if !file_type.unwrap().is_dir() {
            return None;
        }

        return Some(p.path());
    };
    paths
        .filter_map(map)
        .map(|p| -> String { p.into_os_string().into_string().unwrap() })
}
