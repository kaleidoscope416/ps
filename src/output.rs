use crate::para::{PsOptions, PsOutputField, PsOutputProcess};
use crate::proc::process::Process;

pub fn display(processes: Vec<Option<Process>>, option: PsOptions) {
    let processes: Vec<Process> = processes.into_iter().flatten().collect();
}

fn filter_processes(processes: &Vec<Process>, options: &PsOptions) -> Vec<Process> {
    let mut result = Vec::new();
    let mut target_tty = None;
    if let PsOutputProcess::Default(pid) = options.processes.get(0).unwrap() {
        target_tty = processes
            .iter()
            .find(|p| p.pid == *pid)
            .map(|p| p.tty.clone());
    }
    for process in processes {
        let mut matched = false;
        for cond in &options.processes {
            match cond {
                PsOutputProcess::ALL => return processes.clone(),
                PsOutputProcess::Default(_pid) => {
                    if &process.tty == target_tty.as_ref().unwrap() {
                        matched = true;
                    }
                }
                PsOutputProcess::Pid(pids) => {
                    if pids
                        .iter()
                        .any(|p| p.parse::<u32>().ok() == Some(process.pid))
                    {
                        matched = true;
                    }
                }
                PsOutputProcess::Tty(ttys) => {
                    if ttys.contains(&process.tty) {
                        matched = true;
                    }
                }
            }
            if matched {
                break;
            }
        }
        if matched {
            result.push(process.clone());
        }
    }
    result
}
