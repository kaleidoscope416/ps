use crate::output::display_with_no_para;
mod output;
mod proc;

fn main() {
    display_with_no_para(proc::utils::list_and_read_proc());
}
