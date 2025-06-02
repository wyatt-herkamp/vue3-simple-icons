use std::{
    fs,
    io::{self, Cursor},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Version {0} not found")]
    VersionNotFound(String),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
    #[error("Other error: {0}")]
    Other(String),
}
pub async fn download_icons(
    version: &str,
    target_location: impl AsRef<std::path::Path>,
) -> Result<(), DownloadError> {
    let url =
        format!("https://github.com/simple-icons/simple-icons/archive/refs/tags/{version}.zip");
    let client = reqwest::ClientBuilder::new()
        .user_agent("vue3-simple-icons-builder/0.1.0")
        .build()?;

    let head = client.head(&url).send().await?;
    if !head.status().is_success() {
        if head.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(DownloadError::VersionNotFound(version.to_string()));
        }
        return Err(DownloadError::Other(format!(
            "Unexpected status code: {}",
            head.status()
        )));
    }
    let content_size = head
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let mut response_buffer = Vec::with_capacity(content_size as usize);
    let mut response = client.get(&url).send().await?.error_for_status()?;

    let progress_bar = indicatif::ProgressBar::new(content_size);
    progress_bar.set_message("Downloading icons...");

    while let Some(chunk) = response.chunk().await? {
        progress_bar.inc(chunk.len() as u64);
        response_buffer.extend_from_slice(&chunk);
    }
    progress_bar.finish_with_message("Download complete");

    // Unzip the downloaded content to the target location
    let target_path = target_location.as_ref();
    if !target_path.exists() {
        std::fs::create_dir_all(target_path)?;
    }
    let mut zip = zip::ZipArchive::new(Cursor::new(response_buffer))?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => {
                // Remove the first part of the path
                let path = path.into_iter().skip(1).collect::<std::path::PathBuf>();
                path
            }
            None => continue,
        };
        let outpath = target_path.join(outpath);
        // Remove the leading directory of the zip file
        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::downloader::download_icons;

    #[tokio::test]
    async fn test_download_icons() -> Result<(), Box<dyn std::error::Error>> {
        let version = "15.0.0";
        let target_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("downloaded_icons");
        if target_location.exists() {
            std::fs::remove_dir_all(&target_location).unwrap();
        }
        download_icons(version, &target_location).await?;
        Ok(())
    }
}
