use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

fn main() {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        if !process.name().to_lowercase().contains("firefox") {
            continue;
        }
        println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }
}
