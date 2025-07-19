use super::process::Process;
use std::fs;

//todo: 更改vrtime 为 time
fn format_vrtime(vrtime: u64) -> String {
    let total_seconds = vrtime / 1_000_000_000; // 转换为秒
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn read_process_status(pid: &str) -> Option<Process> {
    let path = format!("/proc/{}/status", pid);
    match fs::read_to_string(&path) {
        Ok(content) => {
            let pid = pid.parse::<u32>().unwrap_or(0); 
            let mut ppid = 0;
            let mut path = String::new();
            let mut status = String::new();
            let mut tty = String::new();
            let mut time = String::new();
            let mut memory = 0;
            //todo: 更改vrtime 为 time
            for line in content.lines() {
                if line.starts_with("Name:") {
                    path = line.split(':').nth(1).unwrap_or("").trim().to_string();
                } else if line.starts_with("State:") {
                    status = line.split(':').nth(1).unwrap_or("").trim().to_string();
                } else if line.starts_with("Tty:") {
                    tty = line.split(':').nth(1).unwrap_or("").trim().to_string();
                } else if line.starts_with("Ppid:") {
                    ppid = line
                        .split(':')
                        .nth(1)
                        .unwrap_or("")
                        .trim()
                        .to_string()
                        .parse::<u32>()
                        .unwrap_or(0);
                } else if line.starts_with("vrtime:") {
                    let vrtime = line
                        .split(':')
                        .nth(1)
                        .unwrap_or("")
                        .trim()
                        .to_string()
                        .parse::<u64>()
                        .unwrap_or(0);
                    //todo: 更改vrtime 为 time
                    time = format_vrtime(vrtime); // 格式化时间
                } else if line.starts_with("VmData:") {
                    memory = memory
                        + line
                            .split(':')
                            .nth(1)
                            .unwrap_or("")
                            .trim()
                            .to_string()
                            .parse::<u32>()
                            .unwrap_or(0);
                } else if line.starts_with("VmExe:") {
                    memory = memory
                        + line
                            .split(':')
                            .nth(1)
                            .unwrap_or("")
                            .trim()
                            .to_string()
                            .parse::<u32>()
                            .unwrap_or(0);
                }
            }
            Some(Process::new(pid, ppid, status, tty, path, time, memory))
        }
        Err(_) => {
            println!("Error reading process status for PID: {}", pid);
            None
        }
    }
}

pub fn list_and_read_proc() -> Vec<Option<Process>> {
    let mut processes: Vec<Option<Process>> = Vec::new();
    match fs::read_dir("/proc") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        if let Some(name) = e.file_name().to_str() {
                            // 检查文件名是否为数字
                            if name.chars().all(|c| c.is_numeric()) {
                                processes.push(read_process_status(name));
                            }
                        }
                    }
                    Err(e) => println!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => println!("Error reading /proc directory: {}", e),
    }
    processes
}
