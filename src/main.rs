use sysinfo::{ProcessExt, System, SystemExt};
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    let app_name = "eog";
    let mut sys = System::new_all();
    let check_delay = Duration::new(2, 0);
    let kill_delay = Duration::new(30, 0);
    let mut last_running = Instant::now();

    loop {
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            if !process.name().to_lowercase().contains(&app_name.to_lowercase()) {
                continue;
            }

            // println!("{}: status: {}", process.name(), process.status());

            if process.status() == sysinfo::ProcessStatus::Run {
                last_running = Instant::now();
                println!("running...")
            } else {
                println!("sleeping: {:?}", Instant::now() - last_running);
                if  Instant::now() - last_running > kill_delay {
                    process.kill();
                }
            }
        }

        sleep(check_delay);
    }
}
