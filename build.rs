use std::env;
use std::env::consts::ARCH;
use std::fs::{copy, create_dir_all};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let arch_output_dir = format!("{}/arch/{}", out_dir, ARCH);
    let files = ["multiboot_header", "boot", "long_mode_init"];
    let arch_path = format!("src/arch/{}", ARCH);
    let kernel = format!("{}/kernel-{}.bin", out_dir, ARCH);

    build_ros();
    assemble(&arch_output_dir, &files, &arch_path);
    link(&arch_output_dir, &files, &arch_path, &kernel);
    iso(&out_dir, &arch_path, &kernel);
}

fn assemble(arch_output_dir: &str, files: &[&str], arch_path: &str) {
    create_dir_all(arch_output_dir).unwrap();

    for file in files.iter() {
        let assembly_file = format!("{}/{}.asm", arch_path, file);
        let output_file = format!("{}/{}.o", arch_output_dir, file);
        Command::new("nasm")
            .args(&["-felf64", &assembly_file, "-o", &output_file])
            .status().unwrap();
    }
}

fn build_ros() {
    Command::new("cargo")
        .current_dir("ros")
        .args(&["rustc", "--", "-Z", "no-landing-pads", "-C", "no-redzone"])
        .status().unwrap();
}

fn iso(out_dir: &str, arch_path: &str, kernel: &str) {
    let isofiles_dir = format!("{}/isofiles", out_dir);
    println!("1");
    let grub_dir = format!("{}/isofiles/boot/grub", out_dir);
    create_dir_all(&grub_dir).unwrap();
    let grub_kernel = format!("{}/isofiles/boot/kernel.bin", out_dir);
    copy(kernel, &grub_kernel).unwrap();
    let config = format!("{}/grub.cfg", arch_path);
    let grub_config = format!("{}/grub.cfg", grub_dir);
    copy(&config, &grub_config).unwrap();
    let iso = format!("target/debug/build/os-{}.iso", ARCH);
    Command::new("grub-mkrescue")
        .args(&["-o", &iso, &isofiles_dir])
        .status().unwrap();
}

fn link(arch_output_dir: &str, files: &[&str], arch_path: &str, kernel: &str) {
    let mut output_files = vec![];
    for file in files.iter() {
        output_files.push(format!("{}/{}.o", arch_output_dir, file));
    }
    output_files.push("ros/target/debug/libros.a".to_owned());
    let linker_script = arch_path.to_owned() + "/linker.ld";
    Command::new("ld")
        .args(&["-n", "--gc-sections", "-T", &linker_script, "-o", kernel])
        .args(&output_files)
        .status().unwrap();
}
