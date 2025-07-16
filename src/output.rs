use crate::proc::process::Process;

pub fn display_with_no_para(processes: Vec<Option<Process>>) {
    println!("{:<16} {:<16} {:<16} {}", "PID", "TTY", "TIME", "CMD");
    for process in processes {
        if let Some(proc) = process {
            proc.display_with_no_para();
        } else {
            println!("{:<16} {:<16} {:<16} {}", "N/A", "N/A", "N/A", "N/A");
        }
    }
}
