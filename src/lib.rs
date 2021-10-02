use image;
use image::imageops;
use image::DynamicImage;
use image::GenericImageView;
// use libc;
// use std::os::raw::c_char;
use std::path::Path;
use std::path::PathBuf;
// use std::process::Command;
use std::str::from_utf8;
// use std::thread;
use std::io::{stdout, Write};
use std::time;
use terminal_size::{terminal_size, Height, Width};
use termion::screen::AlternateScreen;

// fn clear() {
//     // unsafe {
//     //     libc::system("clear \x00".as_ptr() as *const c_char);
//     // }
//     let clear_command = if cfg!(target_os = "windows") {
//         "cls"
//     } else {
//         "clear"
//     };
//     std::process::Command::new(clear_command).status().unwrap();
// }

#[derive(Debug)]
struct Frame {
    content: String,
}

impl Frame {
    fn show(&self) {
        let mut screen = AlternateScreen::from(stdout());
        write!(screen, "{}", &self.content).unwrap();
        screen.flush().unwrap();
        // println!("{}", &self.content);
    }
}

struct Stream {
    frames: Vec<Frame>,
}

fn resize_img(
    img: &DynamicImage,
    dims: (u32, u32),
    target_size: (Option<usize>, Option<usize>),
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let aspect_ratio: f64 = img.height() as f64 / img.width() as f64;
    let w_just_ratio = dims.0 as f64;
    let h_just_ratio = dims.1 as f64;

    let (target_width, target_height) = target_size;

    if let Some(width) = target_width {
        if let Some(height) = target_height {
            let img = img.resize_exact(width as u32, height as u32, imageops::FilterType::Nearest);
            return Ok(img);
        } else {
            let height = (width as f64 * aspect_ratio * 0.55 as f64) as i32;
            let img = img.resize_exact(width as u32, height as u32, imageops::FilterType::Nearest);
            return Ok(img);
        }
    }

    if let Some(height) = target_height {
        if let Some(width) = target_width {
            let img = img.resize_exact(width as u32, height as u32, imageops::FilterType::Nearest);
            return Ok(img);
        } else {
            let width = (height as f64 / aspect_ratio / 0.55) as u32;
            let img = img.resize_exact(width as u32, height as u32, imageops::FilterType::Nearest);
            return Ok(img);
        }
    }

    if w_just_ratio < h_just_ratio {
        let width = dims.0;
        let height = (width as f64 * aspect_ratio * 0.55 as f64) as i32;
        let img = img.resize_exact(width, height as u32, imageops::FilterType::Nearest);
        return Ok(img);
    } else {
        let height = dims.1;
        let width = (height as f64 / aspect_ratio / 0.55) as u32;
        let img = img.resize_exact(width, height as u32, imageops::FilterType::Nearest);
        return Ok(img);
    }
    // let min_width = 120;
    // let height = (min_width as f64 * aspect_ratio * 0.55 as f64) as i32;
    // // let height = (min_width as f64 * aspect_ratio) as i32;
    // let img = img.resize_exact(min_width, height as u32, imageops::FilterType::Nearest);
    // Ok(img)
}

fn img_to_frame(
    img_path: &PathBuf,
    dims: (u32, u32),
    target_size: (Option<usize>, Option<usize>),
) -> Result<Frame, Box<dyn std::error::Error>> {
    let img = image::open(&img_path)?;

    let img = resize_img(&img, dims, target_size).unwrap();
    println!("image size: {:?} {:?}", &img.width(), &img.height());

    let img_buf = img.to_luma8();

    let left_padding = (dims.0 - img_buf.width()) / 2;
    let top_padding = (dims.1 - img_buf.height()) / 2;

    let ascii_art = img_buf
        .pixels()
        .map(|p| intensity_to_ascii(&p.0[0]))
        .fold(String::new(), |s, p| s + p);

    // let subs = ascii_art
    //     .as_bytes()
    //     .chunks(img_buf.width() as usize)
    //     .map(from_utf8)
    //     .collect::<Result<Vec<&str>, _>>()
    //     .unwrap()
    //     .into_iter()
    //     .map(|i| i.to_string())
    //     .collect::<Vec<String>>();

    let subs = ascii_art
        .as_bytes()
        .chunks(img_buf.width() as usize)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .into_iter()
        .map(|i| " ".repeat(left_padding as usize) + i)
        .collect::<Vec<String>>()
        .join("\n");

    // let tops = if top_padding > 1 {
    //     String::new() + "github.com/wasabiegg\n" + &"\n".repeat((top_padding - 1) as usize)
    // } else {
    //     "\n".repeat(top_padding as usize)
    // };

    let tops = "\n".repeat(top_padding as usize);
    let subs = tops + &subs;

    // let subs = ascii_art
    //     .as_bytes()
    //     .chunks(img_buf.width() as usize)
    //     .map(from_utf8)
    //     .collect::<Result<Vec<&str>, _>>()
    //     .unwrap();

    // let mut screen: Vec<String> = Vec::new();
    // for i in 0..dims.1 {
    //     if i < (&subs).len() as u32 {
    //         // if i < 0 {
    //         let s: String =
    //             subs[i as usize].to_string() + &" ".repeat((dims.0 - img.width() - 1) as usize);
    //         screen.push(s);
    //     } else {
    //         screen.push(" ".repeat((dims.0 - 1) as usize));
    //     }
    // }

    // let screen: String = screen.join("\n") + " ";

    Ok(Frame { content: subs })
}

fn get_stream_from_dir(
    src_path: &Path,
    dims: (u32, u32),
    target_size: (Option<usize>, Option<usize>),
) -> Result<Stream, Box<dyn std::error::Error>> {
    // let frames = src_path
    //     .read_dir()?
    //     .map(|entry| entry.unwrap().path())
    //     .map(|p| img_to_frame(&p, dims).unwrap())
    //     .collect::<Vec<Frame>>();

    let mut paths = src_path
        .read_dir()?
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<PathBuf>>();

    paths.sort_by(|a, b| {
        (&a.to_str().unwrap()[5..])
            .partial_cmp(&b.to_str().unwrap()[5..])
            .unwrap()
    });
    let frames = paths
        .into_iter()
        .map(|p| img_to_frame(&p, dims, target_size).unwrap())
        .collect::<Vec<Frame>>();

    Ok(Stream { frames })
}

fn intensity_to_ascii(value: &u8) -> &str {
    // changes an intensity into an ascii character
    // this is a central step in creating the ascii art
    let ascii_chars = [
        " ", ".", "^", ",", ":", "_", "=", "~", "+", "O", "o", "*", "#", "&", "%", "B", "@", "$",
    ];

    let n_chars = ascii_chars.len() as u8;
    let step = 255u8 / n_chars;
    for i in 1..(n_chars - 1) {
        let comp = &step * i;
        if value < &comp {
            let idx = (i - 1) as usize;
            return ascii_chars[idx];
        }
    }

    ascii_chars[(n_chars - 1) as usize]
}

pub fn run(target_dir: PathBuf, width: Option<usize>, height: Option<usize>, fps: usize) {
    let t_size = terminal_size();

    let (w, h) = match t_size {
        Some((Width(w), Height(h))) => (w, h),
        None => {
            eprintln!("Unable to get terminal size");
            return;
        }
    };

    let dims = (w as u32, h as u32);
    let src_path = &target_dir;
    // println!("{:?}", Path::new(".").canonicalize());

    let stream = get_stream_from_dir(src_path, dims, (width, height)).unwrap();
    // println!("frames: {}", &stream.frames.len());

    let duration = time::Duration::from_millis(1000 / fps as u64);

    let mut next = time::Instant::now();
    loop {
        for frame in stream.frames.iter() {
            loop {
                let now = time::Instant::now();
                if now > next {
                    next = now + duration;
                    // clear();
                    frame.show();
                    // println!("{}", &frame.content.len());
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {}
