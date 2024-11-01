use regex::Regex;

pub fn runner(ascii: &str) -> String {
    let mut last_colour: Option<String> = None;
    let response: String = ascii.lines().map(|line| {
        let (updated_line, current_colour) = ansi_to_html(line.to_string(), last_colour.clone());
        last_colour = current_colour;
        updated_line
    }).collect();
    response
}

fn extract_colored_segments(input: &str) -> Vec<(Option<String>, String)> {
    // Regex to capture ANSI escape sequences and the text that follows them
    let re = Regex::new(r"(\x1b\[(?P<code>[0-9;]+)m)?(?P<text>[^\x1b]*)").unwrap();
    let mut segments = Vec::new();
    let mut current_color: Option<String> = None; // Default color

    for caps in re.captures_iter(input) {
        if let Some(code) = caps.name("code") {
            // Update color based on the ANSI code
            current_color = ansi_code_to_color(code.as_str());
        }
        if let Some(text) = caps.name("text") {
            if !text.as_str().is_empty() {
                // Add the segment with the current color
                segments.push((current_color.clone(), text.as_str().to_string()));
            }
        }
    }

    segments
}

fn ansi_code_to_color(code: &str) -> Option<String> {
    match code {
        "0;31" => Some(String::from("red")),
        "0;32" => Some(String::from("green")),
        "0;33" => Some(String::from("yellow")),
        "0;34" => Some(String::from("blue")),
        "0;35" => Some(String::from("magenta")),
        "0;36" => Some(String::from("cyan")),
        "38;5;93" => Some(String::from("#ff5fd7")),
        "38;5;121" => Some(String::from("#87ff00")),
        "38;5;149" => Some(String::from("#afd700")),
        "38;5;176" => Some(String::from("#ff87ff")),
        "38;5;196" => Some(String::from("#fe0000")),
        "38;5;197" => Some(String::from("#fe0032")),
        "38;5;199" => Some(String::from("#ff0198")),
        "38;5;214" => Some(String::from("#fe9900")),
        _ => None, // Default color
    }
}

fn ansi_to_html(input: String, last_colour: Option<String>) -> (String, Option<String>) {
    let segments = extract_colored_segments(&input);
    if segments.len() == 1 {
        let pair = segments.get(0).unwrap();
        let current_colour: String = if let Some(defined_colour) = pair.clone().0 {
            defined_colour
        } else {
            if last_colour.is_none() { "white".to_string() } else { last_colour.unwrap() }
        };
        (format!(
            r#"<pre style="text-wrap: auto; margin: 0; color: {};">{}</pre>"#,
            current_colour,
            pair.1
        ), Some(current_colour))
    } else {
        (r#"<pre></pre>"#.to_string(), last_colour)
    }
}