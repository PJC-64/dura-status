extern crate sysinfo;

fn main() {
    use sysinfo::{System, SystemExt, ProcessExt, PidExt};
    use std::process;

    let mut s = System::new();
    s.refresh_all();
    let procs = s.processes_by_exact_name("dura");
    for p in procs {
        if p.pid().as_u32() != process::id() {
            println!("Process found: {}, PID: {}\n", p.name(), p.pid().as_u32());
        }
    }
}

