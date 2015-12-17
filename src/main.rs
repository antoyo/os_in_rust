use std::env::consts::ARCH;
use std::process::Command;

fn main() {
    let file = format!("format=raw,file=target/debug/build/os-{}.iso", ARCH);
    let mut args = vec!["-drive", &file];
    if cfg!(feature = "debug") {
        args.append(&mut vec!["-d", "int", "-no-reboot"]);
    }
    Command::new(format!("qemu-system-{}", ARCH))
        .args(&args)
        .status().unwrap();
}
