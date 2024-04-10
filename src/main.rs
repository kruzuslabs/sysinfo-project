use std::{env, usize};

use sysinfo::{Components, CpuRefreshKind, Pid, RefreshKind, System};

fn main() {
    let mut s =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    // let mut s = System::new_all();

    s.refresh_all();

    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    let pid = Pid::from(query.parse::<usize>().unwrap());

    //refershing twice, to geet
    s.refresh_processes();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    s.refresh_processes();

    match s.process(Pid::from(pid)) {
        Some(process) => {
            println!("Name: {}", process.name());
            let memory_mb = process.memory() as f64 / 1_048_576.0;

            //convert to GB
            if memory_mb >= 1024f64 {
                println!("Memory Usage: {:.2} GB", memory_mb / 1024.0);
            } else {
                //MB
                println!("Memory Usage: {:.2} MB", memory_mb);
            }

            println!("CPU Usage: {:.2}%", process.cpu_usage());

            println!("DEBUG:: >>");
            println!("{:#?}", process);
        }
        None => println!("Process not found"),
    }

    let components = Components::new_with_refreshed_list();
    for component in &components {
        println!("{} {}°C", component.label(), component.temperature());
    }
}
