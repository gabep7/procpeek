use std::{env, thread, time::Duration};
use sysinfo::{CpuRefreshKind, Pid, ProcessRefreshKind, System};

fn main() {
    let mut sort_by = "memory".to_string();
    let mut mode = "show".to_string();
    let mut process_count = 10;
    let mut show_summary = false;
    let mut pid_to_kill_opt: Option<usize> = None;
    let mut refresh_rate_secs = 1;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--sort" | "-s" => {
                if i + 1 < args.len() {
                    sort_by = args[i + 1].to_lowercase();
                    i += 1;
                } else {
                    eprintln!("--sort requires an argument (cpu or memory).");
                }
            }
            "--mode" | "-m" => {
                if i + 1 < args.len() {
                    mode = args[i + 1].to_lowercase();
                    i += 1;
                } else {
                    eprintln!("--mode requires an argument (watch or show).");
                }
            }
            "--count" | "-c" => {
                if i + 1 < args.len() {
                    if let Ok(count) = args[i + 1].parse::<usize>() {
                        process_count = count;
                    } else {
                        eprintln!("Invalid number for --count: {}", args[i + 1]);
                    }
                    i += 1;
                } else {
                    eprintln!("--count requires a number argument.");
                }
            }
            "--summary" | "-u" => {
                show_summary = true;
            }
            "--kill" | "-k" => {
                if i + 1 < args.len() {
                    if let Ok(pid_val) = args[i + 1].parse::<usize>() {
                        pid_to_kill_opt = Some(pid_val);
                    } else {
                        eprintln!("Invalid PID for --kill: {}", args[i + 1]);
                    }
                    i += 1;
                } else {
                    eprintln!("--kill requires a PID argument.");
                }
            }
            "--rate" | "-r" => {
                if i + 1 < args.len() {
                    if let Ok(rate) = args[i + 1].parse::<u64>() {
                        if rate > 0 {
                            refresh_rate_secs = rate;
                        } else {
                            eprintln!("--rate must be a positive number.");
                        }
                    } else {
                        eprintln!("Invalid number for --rate: {}", args[i + 1]);
                    }
                    i += 1;
                } else {
                    eprintln!("--rate requires a number argument (seconds).");
                }
            }
            _ => {}
        }
        i += 1;
    }

    if let Some(pid_val_usize) = pid_to_kill_opt {
        let mut sys_kill = System::new();
        sys_kill.refresh_processes_specifics(ProcessRefreshKind::new());

        let pid_to_kill_sys = Pid::from(pid_val_usize);

        if let Some(process) = sys_kill.process(pid_to_kill_sys) {
            println!(
                "Attempting to kill process {} ({}) with SIGKILL...",
                process.name(),
                pid_to_kill_sys
            );
            if process.kill() {
                println!("Successfully sent SIGKILL to process {}.", pid_to_kill_sys);
            } else {
                eprintln!("Failed to send SIGKILL to process {}. It might require higher privileges or the process may have already exited.", pid_to_kill_sys);
            }
        } else {
            eprintln!("Process with PID {} not found.", pid_to_kill_sys);
        }
        return;
    }

    let mut sys = System::new();
    let watch_mode = mode == "watch";

    sys.refresh_memory();
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    sys.refresh_processes_specifics(ProcessRefreshKind::everything());

    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    sys.refresh_processes_specifics(ProcessRefreshKind::new().with_cpu());

    if watch_mode {
        loop {
            print!("\x1B[2J\x1B[1;1H");
            show_snapshot(&mut sys, &sort_by, process_count, show_summary);
            thread::sleep(Duration::from_secs(refresh_rate_secs));
        }
    } else {
        show_snapshot(&mut sys, &sort_by, process_count, show_summary);
    }
}

fn show_snapshot(sys: &mut System, sort_by: &str, process_count: usize, show_summary: bool) {
    sys.refresh_memory();
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    sys.refresh_processes_specifics(ProcessRefreshKind::everything());

    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    sys.refresh_processes_specifics(ProcessRefreshKind::new().with_cpu());

    if show_summary {
        let total_memory_kb = sys.total_memory();
        let used_memory_kb = sys.used_memory();
        let available_memory_kb = sys.available_memory();

        let global_cpu = sys.global_cpu_info();
        let num_cpus = sys.cpus().len();

        println!("System Summary:");
        println!("------------------------------------------------------------");
        println!(
            "CPU Usage: {:>6.2}% ({} Cores/Threads)",
            global_cpu.cpu_usage(),
            num_cpus
        );
        println!(
            "Memory:    {:>7.2} MB / {:>7.2} MB Used ({:>7.2} MB Avail)",
            used_memory_kb as f64 / 1024.0,
            total_memory_kb as f64 / 1024.0,
            available_memory_kb as f64 / 1024.0
        );

        if cfg!(not(target_os = "windows")) {
            let loads = System::load_average();
            println!(
                "Load Avg:  {:.2}, {:.2}, {:.2} (1m, 5m, 15m)",
                loads.one, loads.five, loads.fifteen
            );
        }

        println!("Processes: {}", sys.processes().len());

        let uptime_secs = System::uptime();
        let days = uptime_secs / (24 * 3600);
        let hours = (uptime_secs % (24 * 3600)) / 3600;
        let minutes = (uptime_secs % 3600) / 60;
        let seconds = uptime_secs % 60;
        print!("Uptime:    ");
        if days > 0 {
            print!("{}d ", days);
        }
        if hours > 0 || days > 0 {
            print!("{}h ", hours);
        }
        if minutes > 0 || hours > 0 || days > 0 {
            print!("{}m ", minutes);
        }
        println!("{}s", seconds);
        println!("------------------------------------------------------------");
        println!();
    }

    let mut processes: Vec<_> = sys.processes().values().collect();

    match sort_by {
        "cpu" => processes.sort_by(|a, b| {
            b.cpu_usage()
                .partial_cmp(&a.cpu_usage())
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
        "memory" | _ => processes.sort_by(|a, b| b.memory().cmp(&a.memory())),
    };

    println!(
        "{:>6}  {:<25}  {:>6}  {:>10}",
        "PID", "Name", "CPU%", "MemoryMB"
    );
    println!("{:-<6}  {:-<25}  {:-<6}  {:-<10}", "", "", "", "");

    for process in processes.iter().take(process_count) {
        let pid_str = process.pid().to_string();
        let name = process.name();
        let display_name = if name.len() > 25 {
            format!("{}...", &name[..22])
        } else {
            name.to_string()
        };
        let cpu = process.cpu_usage();
        let mem_mb = process.memory() as f64 / 1024.0;
        println!(
            "{:>6}  {:<25}  {:>6.2}  {:>10.2}",
            pid_str, display_name, cpu, mem_mb
        );
    }
}
