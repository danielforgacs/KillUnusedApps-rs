use sysinfo::{ProcessExt, System, SystemExt};
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    let app_name = match std::env::args().nth(1) {
        Some(text) => text,
        _ => {
            println!("Use with an app name.");
            return;
        },
    };
    let delay_secs = match std::env::args().nth(2) {
        Some(secs) => {
            match secs.parse::<i32>() {
                Ok(num) => num,
                _ => {
                    println!("Could not get kill delat seconfd: \"{}\"", secs);
                    return;
                }
            }
        },
        _ => {
            println!("Set kill time limit.");
            return;
        },
    };
    println!("--> watching app: {}", app_name);
    println!("--> kill time: {}", delay_secs);
    let mut sys = System::new_all();
    let check_delay = Duration::new(2, 0);
    let kill_delay = Duration::new(30, 0);
    let mut last_running = Instant::now();

    loop {
        sys.refresh_all();

        for (_pid, process) in sys.processes() {
            if !process.name().to_lowercase().contains(&app_name.to_lowercase()) {
                continue;
            }

            // println!("{}: status: {}", process.name(), process.status());

            if process.status() == sysinfo::ProcessStatus::Run {
                last_running = Instant::now();
            } else {
                println!("sleeping: {:?}", Instant::now() - last_running);

                if  Instant::now() - last_running > kill_delay {
                    // process.kill();
                }
            }
        }

        sleep(check_delay);
    }
}
