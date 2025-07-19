use core::panic;
use std::vec;

// 支持的输出字段
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PsOutputField {
    Tty,
    Pid,
    PPid,
    Stat,
    Path,
    Time,
    Memory,
    Other,
    // ...可扩展其它字段...
}

// 支持的参数选项
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PsOutputProcess {
    Pid(Vec<String>), // 进程ID列表
    Tty(Vec<String>), // 终端列表
    Default(u32),     // 默认选项
    ALL,              // 其他
}

pub struct PsOptions {
    pub processes: Vec<PsOutputProcess>,
    pub fields: Vec<PsOutputField>,
}
impl Default for PsOptions {
    fn default() -> Self {
        PsOptions {
            processes: Vec::new(),
            fields: vec![
                PsOutputField::Pid,
                PsOutputField::Tty,
                PsOutputField::Time,
                PsOutputField::Path,
            ],
        }
    }
}

/// 解析 ps 命令参数
pub fn parse_ps_options(args: &[String], pid: u32) -> PsOptions {
    let mut opts = PsOptions::default();
    opts.processes.push(PsOutputProcess::Default(pid)); //传入当前进程ID
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-e" | "-A" | "-a" => {
                opts.processes.push(PsOutputProcess::ALL);
            }
            "-p" => {
                opts.processes.pop(); // 清除默认进程ID
                if i + 1 < args.len() {
                    let pids = args[i + 1]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    opts.processes.push(PsOutputProcess::Pid(pids));
                    i += 1;
                }
            }
            "-t" => {
                opts.processes.pop(); // 清除默认进程ID
                if i + 1 < args.len() {
                    let ttys = args[i + 1]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    opts.processes.push(PsOutputProcess::Tty(ttys));
                    i += 1;
                }
            }
            "-o" => {
                if i + 1 < args.len() {
                    let fields = args[i + 1]
                        .split(',')
                        .map(|s| match s.trim() {
                            "tty" => PsOutputField::Tty,
                            "pid" => PsOutputField::Pid,
                            "ppid" => PsOutputField::PPid,
                            "stat" => PsOutputField::Stat,
                            "path" => PsOutputField::Path,
                            "time" => PsOutputField::Time,
                            "memory" => PsOutputField::Memory,
                            _ => PsOutputField::Other,
                        })
                        .collect();
                    opts.fields = fields;
                    i += 1;
                }
            }
            "-l" => {
                opts.fields = vec![
                    PsOutputField::Pid,
                    PsOutputField::Tty,
                    PsOutputField::PPid,
                    PsOutputField::Stat,
                    PsOutputField::Time,
                    PsOutputField::Memory,
                    PsOutputField::Path,
                ];
            }
            _ => {panic!("Unsupported option: {}", args[i]);}
        }
        i += 1;
    }
    opts
}
