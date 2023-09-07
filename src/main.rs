use std::{env, ffi::OsStr, fs, process::Command};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let path = env::args().nth(1).unwrap_or(".".to_owned());

    match calculate(fs::read_dir(&path)?) {
        Ok(duration) => {
            println!(
                "\x1b[1mPath: \x1b[36m{}\x1b[0;1m\nTotal Duration: \x1b[35m{:02}:{:02}:{:02}\x1b[0m",
                fs::canonicalize(path)?.to_string_lossy().trim_start_matches("\\\\?\\"),
                ((duration / 60) / 60),
                ((duration / 60) % 60),
                (duration % 60)
            );
        }
        Err(e) => println!("\x1b[1;31merror\x1b[0m: \x1b[1m{e}\x1b[0m"),
    };

    Ok(())
}

fn calculate(directories: fs::ReadDir) -> Result<u32> {
    let extensions = ["mkv", "mp4", "webm", "mp3", "flac", "wav", "m4a"];
    let mut total = 0.0;

    for directory in directories {
        let path = directory?.path();
        let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");

        if path.is_file() && extensions.contains(&ext) {
            let output = Command::new("ffprobe")
                .arg(path)
                .args([
                    "-v",
                    "quiet",
                    "-of",
                    "default=nw=1:nk=1",
                    "-show_entries",
                    "format=duration",
                ])
                .output()?;

            let duration = String::from_utf8(output.stdout)?
                .trim_end_matches("\r\n")
                .parse::<f64>()?;

            total += duration;
        } else if path.is_dir() {
            if let Ok(duration) = calculate(fs::read_dir(path)?) {
                total += f64::from(duration);
            }
        }
    }

    if total == 0.0 {
        return Err("No media files found".into());
    }

    Ok(total as u32)
}
