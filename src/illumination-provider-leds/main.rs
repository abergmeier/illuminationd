use std::{sync::mpsc, thread};

use inotify::EventMask;
use service::{Entities, org::freedesktop::illumination::v1::entities_service_server::EntitiesServiceServer};
use tonic::transport::Server;
use watch::LedDirEvent;

pub mod dirs;
pub mod watch;
pub mod service;

fn main() {
    let addr = "[::1]:50051".parse().unwrap();

    let (sender, receiver) = mpsc::channel();

    let service = Entities::default();
    
    Server::builder().add_service(EntitiesServiceServer::new(service))
    .serve(addr)
    .await?;

    let watch_thread = thread::spawn(move || {
        let mut watcher = watch::Watcher::new();

        watcher
            .get_led_directory_changes("/sys/class/leds".to_string())
            .for_each(|e| sender.send(e).unwrap());
    });

    dirs::get_leds("/sys/class/leds").for_each(|led| {
        sender
            .send(LedDirEvent {
                mask: EventMask::CREATE,
                cookie: 0,
                name: Some(led),
            })
            .unwrap()
    });

    //let leds = dirs::find_leds("/sys/class/leds");
}
