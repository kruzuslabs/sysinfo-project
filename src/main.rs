use sysinfo::{CpuRefreshKind, Pid, RefreshKind, System};

fn main() {
    let mut s = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    );



    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    s.refresh_all();
    match s.process(Pid::from(4608)) {
        Some(process) => {
            println!("Name: {}", process.name());
            let memory_mb = process.memory() as f64 / 1_048_576.0;

            //convert to GB
            if memory_mb >= 1024f64 {
                println!("Memory Usage: {:.2} GB", memory_mb / 1024f64);
            } else {
                //MB
                println!("Memory Usage: {:.2} MB", memory_mb);
            }

            println!("CPU Usage: {:.2}%", process.cpu_usage());
        }
        None => println!("Process not found"),
    }


    // for (pid, process) in s.processes() {
    //     println!("[{pid}] {} {}", process.name(), process.cpu_usage() * 100f32);
    // }



// Refresh CPUs again.

    // No need to perform another refresh for accurate CPU usage calculation here
}
