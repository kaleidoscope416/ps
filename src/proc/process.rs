//todo: 更改vrtime 为 time

#[derive(Clone)]
pub struct Process {
    pub pid: u32,
    pub ppid: u32,
    pub status: String,
    pub tty: String,
    pub path: String,
    pub time: String,
    pub memory: u32,
    //todo: 进程开起时间
}

impl Process {
    pub fn new(
        pid: u32,
        ppid: u32,
        status: String,
        tty: String,
        path: String,
        time: String,
        memory: u32,
    ) -> Process {
        Process {
            pid,
            ppid,
            status,
            tty,
            path,
            time,
            memory,
        }
    }
}
