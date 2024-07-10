use std::{fs, path::PathBuf, process::Command};

use ansi_term::ANSIString;
use os_info::Type;
use systemstat::{saturating_sub_bytes, Platform, System};

/// Gets amount of installed packages
pub fn get_packages(distro: &Type) -> String {
    let command = match distro {
        Type::Debian | Type::Ubuntu | Type::Pop | Type::Mint | Type::Kali => {
            "dpkg --get-selections | grep -v deinstall | wc -l"
        }
        Type::openSUSE
        | Type::Redhat
        | Type::Fedora
        | Type::CentOS
        | Type::AlmaLinux
        | Type::RockyLinux => "rpm -qa | wc -l",
        Type::Void => "xbps-query -l | wc -l",
        Type::Gentoo => "ls -d /var/db/pkg/*/* | wc -l",
        Type::Arch | Type::Manjaro | Type::EndeavourOS => "pacman -Q | wc -l",
        Type::FreeBSD => "pkg info | wc -l",
        Type::Solus => "eopkg li | wc -l",
        Type::NixOS => "nix-store -qR ~/.nix-profile | wc -l",
        Type::Alpine => "apk info | wc -l",
        _ => "Unable to identify package manager",
    };

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to run pacman command!");

    std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .to_owned()
}

/// Gets distribution
pub fn get_distro() -> (String, Type) {
    let distro = os_info::get().os_type();

    (
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
        distro,
    )
}

/// Gets window manager
pub fn get_window_manager() -> String {
    let wms = vec![
        "i3", "openbox", "awesome", "bspwm", "qtile", "hyprland", "sway", "xmonad", "dwm", "bery",
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

/// Returns used memory and total memory
pub fn get_memory() -> [String; 2] {
    let system = System::new();

    let mem = system.memory().expect("Failed to retrieve memory!");

    [
        saturating_sub_bytes(mem.total, mem.free).to_string(),
        mem.total.to_string(),
    ]
}

/// Loads ascii file from home directory
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

/// Retunrs default ascii
pub fn get_ascii() -> Vec<String> {
    vec![" ╱|、    ", "(˚ˎ 。7  ", " |、˜〵  ", " じしˍ,)/"]
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}
