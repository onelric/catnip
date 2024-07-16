use std::{fs, path::PathBuf, process::Command};

use ansi_term::ANSIString;
use os_info::Type;
use systemstat::{saturating_sub_bytes, Platform, System};

/// Example:
///let colors = color_arg.split(',').map(|c| c.to_string()).collect();
///let color_map = vec![
///    ("red", Color::Red),
///    ("yellow", Color::Yellow),
///    ("blue", Color::Blue),
///    ("cyan", Color::Cyan),
///    ("black", Color::Black),
///    ("green", Color::Green),
///    ("magenta", Color::Purple),
///    ("white", Color::White),
///];
///let color_map = map(&color_map, colors);
pub fn map<'a, T>(mappings: &'a Vec<(&'static str, T)>, args: Vec<String>) -> Vec<Option<&'a T>>
where
    T: std::fmt::Debug + PartialEq + Clone,
{
    args.into_iter()
        .map(|key| {
            for (k, v) in mappings {
                if k == &key {
                    return Some(v);
                }
            }
            None
        })
        .collect()
}

#[derive(PartialEq, Eq, Debug)]
pub enum PackageManager {
    Dpkg(String),
    Rpm(String),
    Xbps(String),
    Emerge(String),
    Pacman(String),
    Pkg(String),
    Eopkg(String),
    Nix(String),
    Apk(String),
    None,
}

impl PackageManager {
    pub fn get_command(&self) -> String {
        match self {
            PackageManager::Dpkg(c) => c.to_owned(),
            PackageManager::Pacman(c) => c.to_owned(),
            PackageManager::Nix(c) => c.to_owned(),
            PackageManager::Rpm(c) => c.to_owned(),
            PackageManager::Pkg(c) => c.to_owned(),
            PackageManager::Xbps(c) => c.to_owned(),
            PackageManager::Eopkg(c) => c.to_owned(),
            PackageManager::Apk(c) => c.to_owned(),
            PackageManager::Emerge(c) => c.to_owned(),
            PackageManager::None => "Unable to identify package manager.".to_owned(),
        }
    }
}

pub fn get_package_manager(distro: &Type) -> PackageManager {
    match distro {
        Type::Debian | Type::Ubuntu | Type::Pop | Type::Mint | Type::Kali => {
            PackageManager::Dpkg("dpkg-query -f '.\\n' -W".to_string())
        }
        Type::openSUSE
        | Type::Redhat
        | Type::Fedora
        | Type::CentOS
        | Type::AlmaLinux
        | Type::RockyLinux => PackageManager::Rpm("rpm -qa".to_string()),
        Type::Void => PackageManager::Xbps("xbps-query -l".to_string()),
        Type::Gentoo => PackageManager::Emerge("ls -d /var/db/pkg/*/*".to_string()),
        Type::Arch | Type::Manjaro | Type::EndeavourOS => PackageManager::Pacman("pacman -Qq".to_string()),
        Type::FreeBSD => PackageManager::Pkg("pkg info".to_string()),
        Type::Solus => PackageManager::Eopkg("eopkg li".to_string()),
        Type::NixOS => PackageManager::Nix(
            "echo -n '󰣖 ' ; echo -n $(nix-store -qR ~/.nix-profile | wc -l) ; echo -n '  ' ; nix-store -qR /run/current-system/sw".to_string(),
        ),
        Type::Alpine => PackageManager::Apk("apk info".to_string()),
        _ => PackageManager::None,
    }
}

pub fn run_command(cmd: &str) -> String {
    let c = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect(format!("Failed to run command {}", cmd).as_str());

    std::str::from_utf8(&c.stdout).unwrap().trim().to_owned()
}

pub fn get_editor() -> String {
    run_command("echo \"$EDITOR\"")
}

// TODO)) Make this more reliable
pub fn _get_shell() -> String {
    match std::env::var("SHELL") {
        Ok(s) => s,
        Err(_) => "Failed to fetch user's shell".to_owned(),
    }
}

/// Gets amount of installed packages
pub fn get_packages(distro: &Type) -> String {
    let package_command = get_package_manager(distro).get_command();

    run_command((package_command.to_owned() + "| wc -l").as_str())
}

pub fn get_host() -> String {
    run_command("hostname").to_lowercase()
}

pub fn get_user() -> String {
    run_command("whoami").to_lowercase()
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
    let output = run_command("ps -e");

    let mut wm = String::from("Unknown");
    for i in wms.iter() {
        if output.contains(i) {
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

/// Returns default ascii
pub fn get_ascii(trauma: bool) -> Vec<String> {
    vec![
        " ╱|、    ",
        if trauma { "(Oˎ o7   " } else { "(˚ˎ 。7  " },
        " |、˜〵  ",
        " じしˍ,)/",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect()
}
