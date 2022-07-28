use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };
    let files = fs::read_dir(path).unwrap();

    let total_duration = calculate_total(files);
    match total_duration {
        0 => println!("no accepted media files found"),
        _ => println!(
            "{}:{}:{}",
            ((total_duration / 60) / 60),
            ((total_duration / 60) % 60),
            (total_duration % 60)
        ),
    }
}

fn calculate_total(paths: std::fs::ReadDir) -> u32 {
    let accepted_extensions = ["mkv", "mp4", "webm", "mp3", "flac", "wav", "m4a"];
    let mut total_duration: f64 = 0.0;

    for path in paths {
        // dont count if:
        // its a folder
        if path.as_ref().unwrap().path().is_dir()
            // or if its a dotfile
            || path
                .as_ref()
                .unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(".")
            // or if it ends with an extension that isnt accepted
            || !accepted_extensions.contains(
                &path
                    .as_ref()
                    .unwrap()
                    .path()
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            )
        {
            continue;
        };

        let output = Command::new("ffprobe")
            .arg(path.unwrap().path())
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
            .unwrap()
            .trim_end_matches("\r\n")
            .parse::<f64>()
            .unwrap();

        total_duration += duration;
    }

    total_duration as u32
}
