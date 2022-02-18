extern crate sysinfo;
use sysinfo::{System, SystemExt, ProcessExt};

fn main() {
    let mut s = System::new();
    s.refresh_all();
    let procs = s.processes_by_exact_name("dura");
    for p in procs {
        println!("Process found: {}, PID: {}\n", p.name(), p.pid());
    }
}

