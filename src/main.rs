use std::{collections::HashMap, process::Command};

use os_info::Type;
use systemstat::{saturating_sub_bytes, Platform, System};

use ansi_term::{ANSIString, Color};

fn main() {
    // Add window managers as needed
    let wms = ["i3", "openbox", "awesome", "bspwm"];

    // DON'T REMOVE ME ;-;
    let art = ANSIString::from(
        "
     ╱|、      
    (˚ˎ 。7    
     |、˜〵    
     じしˍ,)ノ 
        ",
    );

    let mut os_icons: HashMap<&str, Type> = HashMap::default();

    // Yes
    os_icons.insert("", Type::Alpine);
    os_icons.insert("", Type::Kali);
    os_icons.insert("", Type::RockyLinux);
    os_icons.insert("󰣭", Type::Mint);
    os_icons.insert("", Type::AlmaLinux);
    os_icons.insert("", Type::Arch);
    os_icons.insert("", Type::CentOS);
    os_icons.insert("", Type::Debian);
    os_icons.insert("", Type::EndeavourOS);
    os_icons.insert("", Type::Fedora);
    os_icons.insert("", Type::FreeBSD);
    os_icons.insert("", Type::Gentoo);
    os_icons.insert("", Type::Manjaro);
    os_icons.insert("", Type::NixOS);
    os_icons.insert("", Type::openSUSE);
    os_icons.insert("", Type::Redhat);
    os_icons.insert("", Type::Solus);
    os_icons.insert("", Type::Ubuntu);
    os_icons.insert("", Type::Void);
    os_icons.insert("", Type::Pop);

    let system = System::new();

    let distro = os_info::get().os_type();

    let mem = system.memory().unwrap();

    let art_lines = art.lines();
    let mut art_array = vec![];
    for i in art_lines {
        art_array.push(i);
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg("pacman -Qq | wc -l")
        .output()
        .unwrap();
    let packages = std::str::from_utf8(&output.stdout).unwrap().trim();

    let output = Command::new("sh").arg("-c").arg("ps -e").output().unwrap();
    let output_str = std::str::from_utf8(&output.stdout).unwrap();

    let mut wm = "";
    for i in &wms {
        if output_str.contains(i) {
            wm = i;
        }
    }

    let mut d = String::default();
    for (k, v) in os_icons.iter() {
        if &distro == v {
            d = format!("{} {}", k, distro.to_string())
        }
    }

    let w = format!("{} {}", "󰖲", wm);
    let p = format!("{} {}", "󰏖", packages);
    let m = format!(
        "{} {} / {}",
        "󰍛",
        saturating_sub_bytes(mem.total, mem.free),
        mem.total
    );

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
            fetch += format!("{}  {}\n", art_array[i], text).as_str();
        } else {
            fetch += format!("{}\n", art_array[i]).as_str()
        }
    }
    println!("\n{}\n", fetch)
}
