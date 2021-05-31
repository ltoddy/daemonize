use std::process;
use std::thread::sleep;
use std::time::Duration;

use daemonize::Daemonize;

fn main() {
    let daemonize = Daemonize::new();

    if let Err(e) = daemonize.start() {
        eprintln!("start daemonize failed: {}", e);
        process::exit(-1);
    }

    for i in 0..10 {
        println!("number: {}", i);
        sleep(Duration::from_secs(1));
    }
}
