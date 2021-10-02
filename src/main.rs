// #![allow(dead_code)]
use clap::{App, Arg};
use std::fs;
use std::path::Path;
use terminal_player::run;

mod ffmpeg;
use ffmpeg::generate_im_from_gif;

fn main() {
    let matches = App::new("Gif Terminal Player")
        .version("1.0.0")
        .author("wasabiegg")
        .arg(
            Arg::new("input")
                // .short('i')
                // .long("input")
                // .value_name("INPUT")
                .about("media file path [gif or video]")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("dir")
                .short('d')
                .long("dir")
                .value_name("DIR")
                .about("data dir to store images")
                .takes_value(true),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .value_name("WIDTH")
                .about("resize image width")
                .takes_value(true),
        )
        .arg(
            Arg::new("height")
                .short('h')
                .long("height")
                .value_name("HEIGHT")
                .about("resize image height")
                .takes_value(true),
        )
        .arg(
            Arg::new("fps")
                .long("fps")
                .value_name("FPS")
                .about("specify fps to play gif")
                .takes_value(true)
                .default_value("30"),
        )
        .get_matches();

    let media_path_str = match matches.value_of("input") {
        Some(i) => i,
        None => {
            eprintln!("could get gif path");
            return;
        }
    };

    let data_dir = match matches.value_of("dir") {
        Some(i) => Path::new(i),
        None => Path::new("./data"),
    };

    // Crate data dir if not existed
    if !data_dir.is_dir() {
        fs::create_dir(data_dir).unwrap();
    }

    let width: Option<usize> = match matches.value_of("width") {
        Some(i) => match i.parse::<usize>() {
            Ok(r) => Some(r),
            Err(_) => {
                eprintln!("can't parse width to int, width should be a int");
                return;
            }
        },
        None => None,
    };
    let height: Option<usize> = match matches.value_of("height") {
        Some(i) => match i.parse::<usize>() {
            Ok(r) => Some(r),
            Err(_) => {
                eprintln!("can't parse height to int, height should be a int");
                return;
            }
        },
        None => None,
    };

    let fps: usize = match matches.value_of("fps").unwrap().parse() {
        Ok(i) => i,
        Err(_) => {
            eprintln!("can't parse fps to int, fps should be a int");
            return;
        }
    };

    let gif_path = Path::new(media_path_str);
    if !gif_path.is_file() {
        eprintln!("gif file not found: {:?}", &gif_path);
        return;
    }

    println!("{:?}", &gif_path);

    let output_dir = generate_im_from_gif(gif_path, data_dir).unwrap();

    run(output_dir, width, height, fps);
}
