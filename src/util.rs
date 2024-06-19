use core::panic;
use std::{fs, path::PathBuf, process::Command, str::FromStr};

use ansi_term::ANSIString;
use getopts::Options;
use os_info::Type;
use systemstat::{saturating_sub_bytes, Platform, System};

pub type FetchResult<T> = Result<T, String>;

pub fn fetch_argument<T>(args: &Vec<String>, opts: &Options, alias: &str) -> FetchResult<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let provided_args = match opts.parse(&args[1..]) {
        Ok(a) => a,
        Err(_) => panic!("Error parsing."),
    };

    if let Some(p) = provided_args.opt_str(alias) {
        p.parse::<T>()
            .map_err(|err| format!("Failed to parse argument: {:?}", err))
    } else {
        Err(format!("No argument called: \"{}\"", alias))
    }
}

// Get packages
pub fn get_packages() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("pacman -Qq | wc -l")
        .output()
        .expect("Failed to run pacman command!");

    std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .to_owned()
}

pub fn get_distro() -> [String; 2] {
    let distro = os_info::get().os_type();

    // Get icon
    [
        match distro {
            Type::Arch => "",
            Type::Debian => "",
            Type::Ubuntu => "",
            Type::Void => "",
            Type::Gentoo => "",
            Type::Mint => "󰣭",
            Type::Pop => "",
            Type::Manjaro => "",
            Type::openSUSE => "",
            Type::Redhat => "",
            Type::FreeBSD => "",
            Type::Solus => "",
            Type::NixOS => "",
            Type::Fedora => "",
            Type::EndeavourOS => "",
            Type::CentOS => "",
            Type::AlmaLinux => "",
            Type::RockyLinux => "",
            Type::Kali => "",
            Type::Alpine => "",
            _ => "",
        }
        .to_owned(),
        distro.to_string(),
    ]
}

pub fn get_window_manager() -> String {
    let wms = vec![
        "i3", "openbox", "awesome", "bspwm", "qtile", "hyprland", "sway", "xmonad", "dwm",
    ];

    // Get window manager
    let output = Command::new("sh")
        .arg("-c")
        .arg("ps -e")
        .output()
        .expect("Failed to run command!");
    let output_str = std::str::from_utf8(&output.stdout).unwrap();

    let mut wm = String::from("Unknown");
    for i in wms.iter() {
        if output_str.contains(i) {
            wm = String::from(*i)
        }
    }

    wm
}

pub fn get_memory() -> [String; 2] {
    let system = System::new();

    let mem = system.memory().expect("Failed to retrieve memory!");

    [
        saturating_sub_bytes(mem.total, mem.free).to_string(),
        mem.total.to_string(),
    ]
}

pub fn load_ascii(p: &str) -> Option<Vec<String>> {
    let path = PathBuf::from(std::env::var("HOME").unwrap()).join(p);
    if !path.exists() {
        return None;
    }

    Some(
        ANSIString::from(fs::read_to_string(&path).unwrap())
            .split('\n')
            .map(|x| x.to_string())
            .collect(),
    )
}

pub fn get_ascii() -> Vec<String> {
    vec![" ╱|、", "(˚ˎ 。7", " |、˜〵", " じしˍ,)/"]
        .into_iter()
        .map(|x| x.trim_end().to_string())
        .collect()
}
