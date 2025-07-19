use crate::para::{PsOptions, PsOutputField, PsOutputProcess};
use crate::proc::process::Process;

pub fn display(processes: Vec<Option<Process>>, option: PsOptions) {
    let processes: Vec<Process> = processes.into_iter().flatten().collect::<Vec<Process>>();
    let filtered_processes = filter_processes(&processes, &option);
    print_fields(&option.fields);
    print_process(&option.fields, &filtered_processes);
}

fn filter_processes(processes: &Vec<Process>, options: &PsOptions) -> Vec<Process> {
    let mut result = Vec::new();
    let mut target_tty = None;
    if let Some(PsOutputProcess::Default(pid)) = options.processes.get(0) {
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
    if result.is_empty() {
        eprintln!("No matching processes found.");
    }
    result
}

fn print_fields(fields: &Vec<PsOutputField>) {
    for field in fields {
        let name = match field {
            PsOutputField::Tty => "TTY",
            PsOutputField::Pid => "PID",
            PsOutputField::PPid => "PPID",
            PsOutputField::Stat => "STAT",
            PsOutputField::Time => "TIME",
            PsOutputField::Memory => "MEMORY",
            PsOutputField::Other => panic!("UNKNOWN ARGUMENT"),
            PsOutputField::Path => "PATH",
        };
        print!("{:<14} ", name);
    }
    print!("\n");
}

fn print_process(fields: &Vec<PsOutputField>, filtered_processes: &Vec<Process>) {
    for process in filtered_processes {
        for field in fields {
            let value = match field {
                PsOutputField::Tty => &process.tty,
                PsOutputField::Pid => &process.pid.to_string(),
                PsOutputField::PPid => &process.ppid.to_string(),
                PsOutputField::Stat => &process.status,
                PsOutputField::Path => &process.path,
                PsOutputField::Time => &process.time,
                PsOutputField::Memory => &process.memory.to_string(),
                PsOutputField::Other => "N/A",
            };
            print!("{:<14} ", value);
        }
        print!("\n");
    }
}
