extern crate sysinfo;
use sysinfo::{System, SystemExt};

fn main() {
    let s = System::new();
    for process in s.processes_by_name("dura") {
        println!("Process found: {:?}", process);
    }

}

