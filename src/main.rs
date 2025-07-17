use std::env;
mod output;
mod para;
mod proc;

fn main() {
    let pid = std::process::id();
    let args: Vec<String> = env::args().collect();
    let option = para::parse_ps_options(&args, pid);
    let processes = proc::utils::list_and_read_proc();
    output::display(processes, option);
}
