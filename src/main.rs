use std::env::consts::ARCH;
use std::process::Command;

fn main() {
    let file = format!("format=raw,file=target/debug/build/os-{}.iso", ARCH);
    Command::new(format!("qemu-system-{}", ARCH))
        .args(&["-drive", &file])
        .status().unwrap();
}
