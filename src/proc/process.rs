pub struct Process {
    pub pid: u32,
    pub status: String,
    pub tty: String,
    pub path: String,
    pub time: String,
    pub memory: u32,
}

impl Process {
    pub fn new(
        pid: u32,
        status: String,
        tty: String,
        path: String,
        time: String,
        memory: u32,
    ) -> Process {
        Process {
            pid,
            status,
            tty,
            path,
            time,
            memory,
        }
    }

    pub fn display_with_no_para(&self) {
        println!(
            "{:<16} {:<16} {:<16} {}",
            self.pid, self.tty, self.time, self.path
        );
    }
}
