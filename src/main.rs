extern crate ansi_term;
extern crate getopts;
extern crate os_info;
extern crate systemstat;

mod args;
mod util;

use std::env;

use args::{add_arg, fetch_argument, FetchResult};
use util::*;

use ansi_term::Color;
use getopts::Options;

const ARGUMENTS: [[&'static str; 2]; 6] = [
    ["trauma", "has the cat been through some stuff?"],
    ["info", "changes what system information gets displayed"],
    ["padding", "changes the padding between the ascii and system info"],
    ["file", "specifies a file to be loaded as ascii. max 4 lines long"],
    [
        "seperator",
        "adds custom seperator characters between the icons and the information",
    ],
    [
        "color",
        "changes the color of the icons and system information that is displayed",
    ],
];

fn main() {
    // Define args
    let mut opts = Options::new();

    for i in ARGUMENTS.iter() {
        add_arg(&mut opts, i[0], i[1]);
    }

    // Get args
    let args: Vec<String> = env::args().collect();
    let trauma: bool = fetch_argument(&args, &opts, "t").unwrap_or(false);

    let color_arg: String =
        fetch_argument(&args, &opts, "c").unwrap_or("blue,yellow,green,purple".to_owned());
    let file_path: FetchResult<String> = fetch_argument(&args, &opts, "f");
    let padding: usize = fetch_argument(&args, &opts, "p").unwrap_or(3);
    let sep: String = fetch_argument(&args, &opts, "s").unwrap_or("".to_owned());
    let stats_arg: String = fetch_argument(&args, &opts, "i").unwrap_or("os,wm,pkgs,mem".to_owned());

    let f = |i: &str, s: String| -> String { format!("{i} {} {s}", sep) };

    let mem = get_memory();
    let distro = get_distro();
    let stats: Vec<String> = stats_arg.split(',').map(|s| s.to_string()).collect();

    let info_map = vec![
        ("os", f(distro.0.as_str(), distro.1.to_string().to_lowercase())),
        ("user", format!("{} {} {}@{}", "", sep, get_host(), get_user())),
        ("mem", format!("{} {} {} / {}", "󰍛", sep, mem[0], mem[1])),
        ("pkgs", f("󰏖", get_packages(&distro.1))),
        ("wm", f("󰖲", get_window_manager())),
        ("editor", f("󰅩", get_editor())),
        ("kernel", f("󰒓", run_command("uname -r"))),
    ];
    let data = map(&info_map, stats);

    // Map colors
    let colors = color_arg.split(',').map(|c| c.to_string()).collect();
    let color_map = vec![
        ("red", Color::Red),
        ("yellow", Color::Yellow),
        ("blue", Color::Blue),
        ("cyan", Color::Cyan),
        ("black", Color::Black),
        ("green", Color::Green),
        ("magenta", Color::Purple),
        ("white", Color::White),
    ];
    let color_map = map(&color_map, colors);

    // Retrieve data
    let ascii = file_path.map_or(get_ascii(trauma), |path| {
        load_ascii(path.as_str()).unwrap_or(get_ascii(trauma))
    });

    let mut fetch = String::new();
    for i in 0..ascii.len() {
        let mut ascii_line = ascii[i].to_owned();

        if i < data.len() {
            // Apply spacing
            for _ in 0..padding {
                ascii_line.push(' ');
            }

            // Render data with color
            let data = data[i].unwrap().as_str();
            fetch += format!(
                " {}{}\n",
                ascii_line,
                color_map[i].unwrap_or(&Color::White).italic().paint(data)
            )
            .as_str();
        }
    }

    println!("\n{}\n", fetch)
}
