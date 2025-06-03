use std::path::PathBuf;
use std::time::Duration;
use tokio::{io, sync::broadcast};
use regex::Regex;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader};
use crate::log_watcher::{watch_for_new_log};

pub async fn tail_multiple_logs(dir: &PathBuf, patterns: &[Regex], tx: broadcast::Sender<String>) -> io::Result<()> {
    let mut current_path: Option<PathBuf> = None;

    loop {
        // find latest log
        let next_path = watch_for_new_log(dir.as_path(), current_path.clone()).await?;
        println!("Switching to log file: {:?}", &next_path);

        // tail log till eof
        tail_one_file(&next_path, patterns, tx.clone()).await?;

        // mark as current log
        current_path = Some(next_path);
    }
}

async fn tail_one_file(path: &PathBuf, patterns: &[Regex], tx: broadcast::Sender<String>) -> io::Result<()> {
    let file = loop {
        match File::open(path).await {
            Ok(f) => break f,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                println!("File {:?} not found, retrying in 1s...", path);
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    };

    // go to eof
    let mut file = file;
    file.seek(io::SeekFrom::End(0)).await?;
    let mut reader = BufReader::new(file);

    loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf).await? {
            0 => {
                // wait at eof for a few ms and retry
                tokio::time::sleep(Duration::from_millis(300)).await;
                continue;
            }
            _n => {
                if patterns.iter().any(|r| r.is_match(&buf)) {
                    print!("{}", buf);
                    let _ = tx.send(buf.clone());
                }
            }
        }
    }
}