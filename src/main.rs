use std::process::Command;

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

    let system = System::new();

    let distro = os_info::get().os_type().to_string();
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

    let d = format!("{} {}", "󰣇", distro);
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
