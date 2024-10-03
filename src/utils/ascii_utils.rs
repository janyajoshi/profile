use std::fmt::Write; // Import write! macro for String
//  http://patorjk.com/software/taag/#p=display&h=1&v=0&f=Doom&t=Janya%20Joshi
//  https://coolors.co/gradient-palette/7c00b1-c40000?number=8
use crate::utils::info;

const NAME: &str = r#"
   ___                                 ___              _      _
  |_  |                               |_  |            | |    (_)
    | |  __ _  _ __   _   _   __ _      | |  ___   ___ | |__   _
    | | / _` || '_ \ | | | | / _` |     | | / _ \ / __|| '_ \ | |
/\__/ /| (_| || | | || |_| || (_| | /\__/ /| (_) |\__ \| | | || |
\____/  \__,_||_| |_| \__, | \__,_| \____/  \___/ |___/|_| |_||_|
                       __/ |
                      |___/
"#;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

// Define ANSI color codes
const _GREEN: &str = "\x1b[0;32m";
const _BLUE: &str = "\x1b[0;34m";
const _RED: &str = "\x1b[0;31m";
const _CYAN: &str = "\x1b[0;36m";
const MAGENTA: &str = "\x1b[0;35m";
const YELLOW: &str = "\x1b[0;33m";

fn build_colored_section(title: &str, content: &str) -> String {
    let mut result = String::new();
    writeln!(result, "{}{}{}", BOLD, MAGENTA, title).unwrap();
    write!(result, "{}", RESET).unwrap();
    writeln!(result, "{}{}", YELLOW, content).unwrap();
    writeln!(result, "{}", RESET).unwrap();
    result
}

fn build_gradient_text(text: &str, colors: &[usize], steps_per_color: usize) -> String {
    let mut result = String::new();
    let lines: Vec<&str> = text.lines().collect();
    let num_lines = lines.len();
    // let num_colors = colors.len();
    // let steps = steps_per_color * (num_colors - 1);

    for (i, line) in lines.iter().enumerate() {
        let segment = i * steps_per_color / num_lines;
        let color_index = segment / steps_per_color;
        let start_color = colors[color_index];
        let end_color = colors[color_index + 1];
        let color_step = segment % steps_per_color;

        let gradient_color = interpolate_color(start_color, end_color, steps_per_color, color_step);
        let color_code = color_code_256(gradient_color);
        writeln!(result, "{}{}", color_code, line).unwrap();
    }
    writeln!(result, "{}", RESET).unwrap();
    result
}

fn color_code_256(n: usize) -> String {
    format!("\x1b[38;5;{}m", n)
}

fn interpolate_color(start: usize, end: usize, steps: usize, current_step: usize) -> usize {
    let step = (end as f64 - start as f64) / steps as f64;
    (start as f64 + step * current_step as f64).round() as usize
}

pub fn contact() -> String {
    let colors = [93, 204]; // Purple (93), Blue (21), Red (204)
    let steps_per_color = 4;  // Number of steps between each color transition

    let mut result = String::new();
    result.push_str(&build_gradient_text(NAME, &colors, steps_per_color));
    result.push_str(&build_colored_section("Contact:\n", info::CONTACT));
    result
}

pub fn detail() -> String {
    // sections


    //  Define color stops for the gradient (Purple -> Blue -> Red)
    //  https://coolors.co/gradient-palette/7c00b1-c40000?number=8
    let colors = [93, 204]; // Purple (93), Blue (21), Red (204)
    let steps_per_color = 4;  // Number of steps between each color transition

    // Build the final resume output
    let mut result = String::new();
    result.push_str(&build_gradient_text(NAME, &colors, steps_per_color));
    result.push_str(&build_colored_section("Contact:\n", info::CONTACT));
    result.push_str(&build_colored_section("Summary:\n", info::SUMMARY));
    result.push_str(&build_colored_section("Highlights:\n", info::HIGHLIGHTS));
    result.push_str(&build_colored_section("Personal Projects:\n", info::PERSONAL_PROJECTS));
    result.push_str(&build_colored_section("Skills:\n", info::SKILLS));
    result.push_str(&build_colored_section("Experience:\n", info::EXPERIENCE));
    result.push_str(&build_colored_section("Education:\n", info::EDUCATION));
    result
}