extern crate sysinfo;

use sysinfo::{System, SystemExt};

fn main() {

    let mut mysys = SystemExt::new_all();
    mysys.refresh_all();

	let ps = mysys.get_process_list().iter().filter(|(_, p)| p.name.starts_with("dura"));
	for (pid, p) in ps {
	    println!("{}:{}", pid, p.name);
	}
}

