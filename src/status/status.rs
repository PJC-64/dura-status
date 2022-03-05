extern crate sysinfo;

    pub fn is_running() -> bool {
        use sysinfo::{System, SystemExt, ProcessExt, PidExt};
        use std::process;
        let mut dstatus = false;
        let mut s = System::new();
        s.refresh_all();
        let procs = s.processes_by_exact_name("dura");
        println!("In status::is_running");
        for p in procs {
            if p.pid().as_u32() != process::id() {
                dstatus = true;
            } else {
                dstatus = false;
            }
        }
        return dstatus;
    }
