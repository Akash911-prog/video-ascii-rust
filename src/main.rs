use terminal_size::{Height, Width, terminal_size};
use video_ascii_rust::config::Config;

fn main() {
    let config = Config::init();
    let (width, height) = match get_terminal_size() {
        Ok((w, h)) => (w, h),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1)
        }
    };
}

fn get_terminal_size() -> Result<(u16, u16), Box<dyn std::error::Error>> {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        Ok((w, h))
    } else {
        Err("Failed to get terminal size".into())
    }
}
