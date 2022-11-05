pub mod dbus;

pub mod manager;

fn main() {

    let m = manager::Lights::default();

    dbus::run_server(m);

}


