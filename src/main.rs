use opencv::{
    highgui::{destroy_all_windows, wait_key_ex},
    prelude::*,
    videoio::{CAP_PROP_FPS, CAP_PROP_FRAME_COUNT, VideoCapture},
};
use terminal_size::{Height, Width, terminal_size};
use video_ascii_rust::{
    config::Config, image_processor::{frame_to_ascii, process_frame}, p2c_mapper::{P2CMapper, P2CMapperType},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
};
use std::io::{Write, stdout};

fn draw(frame: &str) -> std::io::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, MoveTo(0, 0))?;
    stdout.write_all(frame.as_bytes())?;
    stdout.flush()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::init();
    let (width, height) = match get_terminal_size() {
        Ok((w, h)) => (w, h),
        Err(e) => {
            println!("Error: {:?}", e);
            std::process::exit(1);
        }
    };

    let mapper = P2CMapper::new(P2CMapperType::Sigmoid, &config);

    let cap = VideoCapture::from_file(&config.input_file, 0);
    if let Err(e) = cap {
        println!("Error: {}", e);
        std::process::exit(1);
    }
    let mut cap = cap?;
    let mut frame = Mat::default();

    let _frame_count = cap.get(CAP_PROP_FRAME_COUNT)?;
    let fps = cap.get(CAP_PROP_FPS)?;
    // println!("Frame count: {}", frame_count);
    // println!("FPS: {}", fps);

    let delay = 1.0 / fps;

    execute!(stdout(), Hide)?;

    loop {
        let start_time = std::time::Instant::now();
        let res = cap.read(&mut frame).expect("Failed to read frame");
        if res {
            let processed_frame = process_frame(&frame, width.into(), (height).into())?;
            let string = frame_to_ascii(&processed_frame, &mapper, &config)?;
            draw(&string)?;
            // imshow("first frame", &processed_frame)?;

            if (wait_key_ex(1)?) == 27 {
                break;
            }
        } else {
            println!("Video end");
            break;
        }

        let elapsed = start_time.elapsed();
        let sleep_time = delay - elapsed.as_secs_f64();
        if sleep_time > 0.0 {
            std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
        }
    }
    cap.release()?;
    destroy_all_windows()?;
    execute!(stdout(), Show)?;
    Ok(())
}

fn get_terminal_size() -> Result<(u16, u16), Box<dyn std::error::Error>> {
    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        Ok((w, h))
    } else {
        Err("Failed to get terminal size".into())
    }
}
