use std::{ffi::OsStr, sync::mpsc::Sender, fs::File, os::unix::prelude::{AsFd, AsRawFd}};

use inotify::{Event, EventMask, Inotify, WatchMask};

const BRIGHTNESS_FILE: &str = "brightness_hw_changed";
const COLOR_TEMP_FILE: &str = "color_temp_hw_changed";

pub struct Watcher {
    buffer: [u8; 1024],
}

pub struct WatchLed {
    brightness_file: Option<File>,
    color_temp_file: Option<File>,
    epfd: i32
}

/// An inotify event
///
/// A file system event that describes a change that the user previously
/// registered interest in. To watch for events, call [`Inotify::add_watch`]. To
/// retrieve events, call [`Inotify::read_events_blocking`] or
/// [`Inotify::read_events`].
///
/// [`Inotify::add_watch`]: struct.Inotify.html#method.add_watch
/// [`Inotify::read_events_blocking`]: struct.Inotify.html#method.read_events_blocking
/// [`Inotify::read_events`]: struct.Inotify.html#method.read_events
#[derive(Clone, Debug)]
pub struct LedDirEvent<S> {

    /// Indicates what kind of event this is
    pub mask: EventMask,

    /// Connects related events to each other
    ///
    /// When a file is renamed, this results two events: [`MOVED_FROM`] and
    /// [`MOVED_TO`]. The `cookie` field will be the same for both of them,
    /// thereby making is possible to connect the event pair.
    ///
    /// [`MOVED_FROM`]: event_mask/constant.MOVED_FROM.html
    /// [`MOVED_TO`]: event_mask/constant.MOVED_TO.html
    pub cookie: u32,

    /// The name of the file the event originates from
    ///
    /// This field is set only if the subject of the event is a file or directory in a
    /// watched directory. If the event concerns a file or directory that is
    /// watched directly, `name` will be `None`.
    pub name: Option<S>,
}

impl<S> From<inotify::Event<S>> for LedDirEvent<S> {
    fn from(ie: inotify::Event<S>) -> Self {
        LedDirEvent { mask: ie.mask, cookie: ie.cookie, name: ie.name }
    }
}

impl WatchLed {
    pub fn new(brightness_file: Option<File>, color_temp_file: Option<File>) -> WatchLed {
        let epfd = epoll::create(false).unwrap();
        if brightness_file.is_some() {
            epoll::ctl(epfd, epoll::ControlOptions::EPOLL_CTL_ADD, brightness_file.unwrap().as_raw_fd(), epoll::Event::new(epoll::Events::EPOLLIN, 0)).unwrap();
        }
        if color_temp_file.is_some() {
            epoll::ctl(epfd, epoll::ControlOptions::EPOLL_CTL_ADD, color_temp_file.unwrap().as_raw_fd(), epoll::Event::new(epoll::Events::EPOLLIN, 0)).unwrap();
        }
        WatchLed {
            brightness_file,
            color_temp_file,
            epfd,
        }
    }

    pub fn poll(&self) {
        let timeout_ms = 1000;
        let buf = Vec::with_capacity(1024);
        let poll_count = epoll::wait(self.epfd, timeout_ms, &mut buf).unwrap();

        match buf[0].data {
            self.brightness_file.unwrap().as_raw_fd() => SendBrightness(buf[0].data),
            self.color_temp_file.unwrap().as_raw_fd() => SendColorTemp(buf[0].data),
        }
    }
}

impl Drop for WatchLed {
    fn drop(&mut self) {
        epoll::close(self.epfd);
    }
}

impl Watcher {
    pub fn new() -> Watcher {
        Watcher {
            buffer: [0; 1024],
        }
    }

    pub fn get_led_directory_changes<S>(&mut self, dir: String) -> impl Iterator<Item = LedDirEvent<S>>{
        let mut inotify = Inotify::init().expect("Error while initializing inotify instance");

        let wl = WatchLed::new(Some(File::open(BRIGHTNESS_FILE).unwrap()), Some(File::open(COLOR_TEMP_FILE).unwrap()));


        // Watch for modify and close events.
        inotify
            .add_watch(dir, WatchMask::CREATE | WatchMask::DELETE)
            .expect("Failed to add file watch");

        // Read events that were added with `add_watch` above.

        let events = inotify
            .read_events_blocking(&mut self.buffer)
            .expect("Error while reading events");

        let filter = |e: &Event<&OsStr>| -> bool {
            !e.mask.intersects(EventMask::ISDIR) || e.name.is_none()
        };
        events.filter(filter).map(|e| LedDirEvent<String> {
            cookie: e.cookie,
            mask: e.mask,
            name: Some(e.name.unwrap().to_str().expect("Expected valid unicode string").into()),
        })
    }
}
