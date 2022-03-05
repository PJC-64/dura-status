mod status;
use status::status as dstat;

fn main() {
    println!("In Main function\n");
    if dstat::is_running() == true {
    println!("Dura is running\n");
    }
}
