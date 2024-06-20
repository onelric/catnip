use std::{fs, path::PathBuf, process::Command, str::FromStr};

use ansi_term::ANSIString;
use getopts::Options;
use os_info::Type;
use systemstat::{saturating_sub_bytes, Platform, System};

pub type FetchResult<T> = Result<T, String>;

/// Returns the result of fetching a commandline argument's value by id.
///
/// # Examples
/// User input:
/// `./app -f ~/filepath`
///
/// ```
/// use getopts::Options;
///
/// let args: Vec<String> = env::args().collect();
/// let mut opts = Options::new();
///
/// opts.optopt("f", "file", "loads an ascii file from path", "FILE");
///
/// let file_path: FetchResult<String> = fetch_argument(&args, &opts, "f");
/// ```
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
        Type::Gentoo => "equery list | wc -l",
        Type::Arch | Type::Manjaro | Type::EndeavourOS => "pacman -Q | wc -l",
        Type::FreeBSD => "pkg info | wc -l",
        Type::Solus => "eopkg li | wc -l",
        Type::NixOS => "nix-env -q | wc -l",
        Type::Alpine => "apk info | wc -l",
        _ => "Unable to identify package manager",
    };

    let output = Command::new("sh")
        .arg("-c")
        //.arg("pacman -Qq | wc -l")
        .arg(command)
        .output()
        .expect("Failed to run pacman command!");

    std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .to_owned()
}

pub fn get_distro() -> (String, Type) {
    let distro = os_info::get().os_type();

    // Get icon
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
