use scraper::Html;

pub fn print_response_info(response: &str) {
    println!("Debug - Response length: {} bytes", response.len());
}

pub fn print_html_sample(response: &str, max_chars: usize) {
    println!("Debug - First {} characters of response:", max_chars);
    println!("{}", &response[..response.len().min(max_chars)]);
}

pub fn print_element_found<T: std::fmt::Display>(element_name: &str, value: &T) {
    println!("Debug - {} element found: {}", element_name, value);
}

pub fn print_section_found(section_name: &str, found: bool) {
    if found {
        println!("Found {} section!", section_name);
    } else {
        println!("Could not find {} section!", section_name);
    }
} 