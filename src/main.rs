use std::thread;
use std::time::Duration;

fn main() {
    println!("ZenCompositor initializing DRM backend...");

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
