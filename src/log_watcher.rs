use std::{io, path::{Path, PathBuf}, time::Duration};
use tokio::{fs::read_dir, time::sleep};

/// Finds latest logfile using filename
pub async fn find_latest_log(dir: &Path) -> io::Result<Option<PathBuf>> {
    let mut rd = match read_dir(dir).await {
        Ok(rd) => rd,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e),
    };

    let mut newest: Option<PathBuf> = None;

    while let Some(entry) = rd.next_entry().await? {
        let file_name = entry.file_name();
        let fname = file_name.to_string_lossy();

        if fname.starts_with("output_log_") && fname.ends_with(".txt") {
            let path = entry.path();
            match &newest {
                None => newest = Some(path),
                Some(prev) => {
                    if let (Some(prev_name), Some(curr_name)) =
                        (prev.file_name(), path.file_name())
                    {
                        if curr_name.to_string_lossy() > prev_name.to_string_lossy() {
                            newest = Some(path);
                        }
                    }
                }
            }
        }
    }

    Ok(newest)
}

/// Given a directory `dir` and an optional `current` PathBuf, waits
/// until a strictly newer log file than `current` appears in `dir`.
///
/// If `current` is None, this returns immediately with whichever
/// “latest” file it finds (or errors if none exist).  
/// Otherwise, it loops (sleeping ~1s between checks), and returns
/// the new PathBuf once a filename > `current.file_name()` is found.
pub async fn watch_for_new_log(dir: &Path, current: Option<PathBuf>) -> io::Result<PathBuf> {
    if current.is_none() {
        loop {
            if let Some(latest) = find_latest_log(dir).await? {
                return Ok(latest);
            }
            sleep(Duration::from_secs(1)).await;
        }
    }

    let current_name = current
        .as_ref()
        .and_then(|p| p.file_name().map(|s| s.to_os_string()));
    loop {
        if let Some(latest) = find_latest_log(dir).await? {
            if let (Some(current_nm), Some(new_nm)) =
                (current_name.as_ref(), latest.file_name())
            {
                if new_nm.to_string_lossy() > current_nm.to_string_lossy() {
                    return Ok(latest);
                }
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
}