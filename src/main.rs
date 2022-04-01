use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let app_name = "eog";
    let mut sys = System::new_all();
    let check_delay = Duration::new(0, 50000);

    loop {
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            if !process.name().to_lowercase().contains(&app_name.to_lowercase()) {
                continue;
            }
            println!("{}: status: {}", process.name(), process.status());
        }

        sleep(check_delay);
    }
}
