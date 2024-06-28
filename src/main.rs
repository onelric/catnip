extern crate ansi_term;
extern crate getopts;
extern crate os_info;
extern crate systemstat;

use std::env;

use ansi_term::Color;
use getopts::Options;
use util::*;

mod util;

fn main() {
    // Define args
    let mut opts = Options::new();

    opts.optopt("f", "file", "loads an ascii file from path", "FILE");
    opts.optopt(
        "p",
        "padding",
        "sets padding between ascii and data",
        "PADDING",
    );
    opts.optopt(
        "s",
        "seperator",
        "changes the symbol between the ascii and statistics",
        "SEPERATOR",
    );
    // Get argument -c distro=red split at =
    opts.optopt("c", "color", "changes color for element", "COLOR");

    // Get args
    let args: Vec<String> = env::args().collect();

    let color_arg: String =
        fetch_argument(&args, &opts, "c").unwrap_or("blue,yellow,green,purple".to_owned());
    let file_path: FetchResult<String> = fetch_argument(&args, &opts, "f");
    let padding: usize = fetch_argument(&args, &opts, "p").unwrap_or(3);
    let sep: String = fetch_argument(&args, &opts, "s").unwrap_or("".to_owned());

    // Map colors
    let colors: Vec<&str> = color_arg.split(',').collect();
    let mut color_map: Vec<Color> = vec![];
    for i in 0..4 {
        let color_str = colors[i];
        let color = match color_str {
            "red" => Color::Red,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "cyan" => Color::Cyan,
            "black" => Color::Black,
            "green" => Color::Green,
            "purple" => Color::Purple,
            "white" | _ => Color::White,
        };
        color_map.push(color);
    }

    // Retrieve data
    let ascii = file_path.map_or(get_ascii(), |path| {
        load_ascii(path.as_str()).unwrap_or(get_ascii())
    });
    let mem = get_memory();
    let distro = get_distro();

    // Create data array in order
    let data = [
        format!("{} {} {}", distro.0, sep, distro.1),
        format!("{} {} {}", "󰖲", sep, get_window_manager()),
        format!("{} {} {}", "󰏖", sep, get_packages(&distro.1)),
        format!("{} {} {} / {}", "󰍛", sep, mem[0], mem[1]),
    ];

    // Get longest line in the ascii to properly space the information
    let longest = ascii.iter().map(|x| x.len()).max().unwrap();

    let mut fetch = String::new();
    for i in 0..ascii.len() {
        let mut ascii_line = ascii[i].to_owned();

        if i < data.len() {
            // Add spacing elements
            for _ in 0..longest + padding - ascii_line.len() {
                ascii_line.push(' ');
            }

            // Render data with color
            let data = data[i].as_str();
            fetch += format!(" {}{}\n", ascii_line, color_map[i].italic().paint(data)).as_str();
        }
    }

    println!("\n{}\n", fetch)
}
