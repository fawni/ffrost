use std::{env, fs, process::Command};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| ".".to_owned());
    let files = fs::read_dir(path).expect("could not read files in the current directory");

    let total_duration = calculate_total(files).unwrap();
    match total_duration {
        0 => println!("no accepted media files found"),
        _ => println!(
            "{:02}:{:02}:{:02}",
            ((total_duration / 60) / 60),
            ((total_duration / 60) % 60),
            (total_duration % 60)
        ),
    }
}

fn calculate_total(directories: std::fs::ReadDir) -> Option<u32> {
    let accepted_extensions = ["mkv", "mp4", "webm", "mp3", "flac", "wav", "m4a"];
    let mut total_duration: f64 = 0.0;

    for directory in directories {
        let path = directory.ok()?.path();

        if path.is_file() && accepted_extensions.contains(&path.extension()?.to_str()?) {
            let output = Command::new("ffprobe")
                .arg(path)
                .arg("-v")
                .arg("quiet")
                .arg("-of")
                .arg("default=nw=1:nk=1")
                .arg("-show_entries")
                .arg("format=duration")
                .output()
                .expect("process failed to execute")
                .stdout;

            let duration: f64 = String::from_utf8(output)
                .ok()?
                .trim_end_matches("\r\n")
                .parse::<f64>()
                .ok()?;

            total_duration += duration;
        }
    }

    Some(total_duration as u32)
}
