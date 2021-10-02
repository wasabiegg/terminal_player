use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn generate_im_from_gif(
    gif_path: &Path,
    data_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let gif_name = gif_path.file_stem().unwrap();
    let output_dir = data_dir.join(gif_name);

    if !output_dir.is_dir() {
        fs::create_dir(&output_dir).unwrap();
    } else {
        return Ok(output_dir);
    }

    let mut command = Command::new("ffmpeg");
    command
        .arg("-i")
        .arg(gif_path.as_os_str())
        .arg(output_dir.join("frame%04d.png"));

    command.status().unwrap();

    return Ok(output_dir);
}
