use std::process::Command;

use os_info::Type;
use systemstat::{saturating_sub_bytes, Platform, System};

use ansi_term::{ANSIString, Color};

// Get packages
fn get_packages() -> String {
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

fn get_distro() -> [String; 2] {
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

fn get_window_manager(wms: Vec<&str>) -> String {
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

fn get_memory() -> [String; 2] {
    let system = System::new();

    let mem = system.memory().expect("Failed to retrieve memory!");

    [
        saturating_sub_bytes(mem.total, mem.free).to_string(),
        mem.total.to_string(),
    ]
}

fn get_ascii() -> Vec<String> {
    // DON'T REMOVE ME ;-;
    let art = ANSIString::from(
        "
     ╱|、      
    (˚ˎ 。7    
     |、˜〵    
     じしˍ,)ノ 
        ",
    );

    let art_lines = art.lines();
    let mut art_array = vec![];
    for i in art_lines {
        art_array.push(String::from(i));
    }
    art_array
}

fn main() {
    // Add window managers as needed
    let wms = vec![
        "i3", "openbox", "awesome", "bspwm", "qtile", "hyprland", "sway", "xmonad", "dwm",
    ];

    let mem = get_memory();
    let distro = get_distro();
    let ascii = get_ascii();

    let d = format!("{} {}", distro[0], distro[1]);
    let w = format!("{} {}", "󰖲", get_window_manager(wms));
    let p = format!("{} {}", "󰏖", get_packages());
    let m = format!("{} {} / {}", "󰍛", mem[0], mem[1]);
    let data = vec![d.as_str(), w.as_str(), p.as_str(), m.as_str()];

    let mut fetch = String::new();
    for i in 1..5 {
        let di = i - 1;
        if di < data.len() {
            let data_text = data[di];
            let text = match di {
                0 => Color::Blue.italic().paint(data_text),
                1 => Color::Yellow.italic().paint(data_text),
                2 => Color::Green.italic().paint(data_text),
                3 => Color::Purple.italic().paint(data_text),
                _ => ANSIString::from(""),
            };
            fetch += format!("{}  {}\n", ascii[i], text).as_str();
        } else {
            fetch += format!("{}\n", ascii[i]).as_str()
        }
    }
    println!("\n{}\n", fetch)
}
