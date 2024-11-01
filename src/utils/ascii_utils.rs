use std::fmt::Write; // Import write! macro for String
//  https://talyian.github.io/ansicolors/
//  http://patorjk.com/software/taag/#p=display&h=1&v=0&f=Doom&t=Janya%20Joshi
//  https://coolors.co/gradient-palette/7c00b1-c40000?number=8
use crate::utils::info;

// const _NAME: &str = r#"
//    ___                                 ___              _      _
//   |_  |                               |_  |            | |    (_)
//     | |  __ _  _ __   _   _   __ _      | |  ___   ___ | |__   _
//     | | / _` || '_ \ | | | | / _` |     | | / _ \ / __|| '_ \ | |
// /\__/ /| (_| || | | || |_| || (_| | /\__/ /| (_) |\__ \| | | || |
// \____/  \__,_||_| |_| \__, | \__,_| \____/  \___/ |___/|_| |_||_|
//                        __/ |
//                       |___/
// "#;

// const _RESET: &str = "\x1b[0m";
// const _BOLD: &str = "\x1b[1m";

// Define ANSI color codes
// const _GREEN: &str = "\x1b[0;32m";
// const _BLUE: &str = "\x1b[0;34m";
// const _RED: &str = "\x1b[0;31m";
// const _CYAN: &str = "\x1b[0;36m";
const MAGENTA: &str = "\x1b[0;35m";
const YELLOW: &str = "\x1b[0;33m";

fn build_colored_section(title: &str, content: &str) -> String {
    let mut result = String::new();
    writeln!(result, "{}{}", MAGENTA, title).unwrap();
    writeln!(result, "{}{}\n", YELLOW, content).unwrap();
    // writeln!(result, "{}", RESET).unwrap();
    result
}

// fn rgb_to_ansi(r: u8, g: u8, b: u8) -> String {
//     format!("\x1b[38;2;{};{};{}m", r, g, b)
// }

//  https://talyian.github.io/ansicolors/
fn get_title() -> String {
    let title = format!(r#"
{}   ___                                 ___              _      _
{}  |_  |                               |_  |            | |    (_)
{}    | |  __ _  _ __   _   _   __ _      | |  ___   ___ | |__   _
{}    | | / _` || '_ \ | | | | / _` |     | | / _ \ / __|| '_ \ | |
{}/\__/ /| (_| || | | || |_| || (_| | /\__/ /| (_) |\__ \| | | || |
{}\____/  \__,_||_| |_| \__, | \__,_| \____/  \___/ |___/|_| |_||_|
{}                       __/ |
{}                      |___/
"#, "\x1b[38;5;196m", "\x1b[38;5;197m", "\x1b[38;5;199m", "\x1b[38;5;214m", "\x1b[38;5;214m", "\x1b[38;5;199m", "\x1b[38;5;197m", "\x1b[38;5;196m");
    title
}

// fn color_code_256(n: usize) -> String {
//     format!("\x1b[38;5;{}m", n)
// }

// fn interpolate_color(start: usize, end: usize, steps: usize, current_step: usize) -> usize {
//     let step = (end as f64 - start as f64) / steps as f64;
//     (start as f64 + step * current_step as f64).round() as usize
// }

pub fn contact() -> String {
    let mut result = String::new();
    result.push_str(&get_title());
    result.push_str(&build_colored_section("Contact:\n", info::CONTACT));
    result
}

pub fn detail() -> String {
    // sections


    //  Define color stops for the gradient (Purple -> Blue -> Red)
    //  https://coolors.co/gradient-palette/7c00b1-c40000?number=8

    // Build the final resume output
    let mut result = String::new();
    result.push_str(&get_title());
    result.push_str(&build_colored_section("Contact:\n", info::CONTACT));
    result.push_str(&build_colored_section("Summary:\n", info::SUMMARY));
    result.push_str(&build_colored_section("Highlights:\n", info::HIGHLIGHTS));
    result.push_str(&build_colored_section("Personal Projects:\n", info::PERSONAL_PROJECTS));
    result.push_str(&build_colored_section("Skills:\n", info::SKILLS));
    result.push_str(&build_colored_section("Experience:\n", info::EXPERIENCE));
    result.push_str(&build_colored_section("Education:\n", info::EDUCATION));
    result
}