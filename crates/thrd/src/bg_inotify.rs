// use inotify::{Inotify, WatchMask};

#[allow(unused)]
fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn inotify() {
    let mut inotify = Inotify::init().expect("Error while initializing inotify instance");
    let wd = inotify
        .watches()
        .add(
            "/run/user/1000/ntf",
            WatchMask::CREATE | WatchMask::MODIFY | WatchMask::DELETE,
        )
        .expect("Failed to add directory watch");
    // Read events that were added with `add_watch` above.
    let mut buffer = [0u8; 1024];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Error while reading events");
        for event in events {
            if event.wd == wd {
                println!("Event: {:?}", event);
            }
        }
    }
}
