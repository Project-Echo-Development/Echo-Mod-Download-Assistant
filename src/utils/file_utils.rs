use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::utils::panels::Mod;

pub struct InstallRecord {
    pub installed_files: Vec<std::path::PathBuf>,
    pub created_dirs: Vec<std::path::PathBuf>,
}

#[derive(Deserialize)]
struct GithubRelease {
    assets: Vec<GithubAsset>,
}
#[derive(Deserialize)]
struct GithubAsset {
    browser_download_url: String,
    name: String,
}
pub fn fetch_latest_release_zip(
    repo: &str,
    install_steam: bool,
    install_epic: bool,
) -> anyhow::Result<String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    println!("{}", url);
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(&url)
        .header("User-Agent", "Echo-Download-Assistant")
        .header("Accept", "application/vnd.github+json")
        .send()?
        .error_for_status()?;

    let release: GithubRelease = response.json()?;
    let keyword = if install_steam {
        "steam"
    } else if install_epic {
        "epic"
    } else {
        return Err(anyhow::anyhow!("No install platform selected"));
    };

    let zip = release
        .assets
        .into_iter()
        .find(|a| {
            a.name
                .ends_with(".zip")
                && a.name
                    .to_lowercase()
                    .contains(keyword)
        })
        .ok_or_else(|| anyhow::anyhow!("No {} zip found", keyword))?;
    Ok(zip.browser_download_url)
}
fn download_file(url: &str) -> anyhow::Result<std::path::PathBuf> {
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;
    let mut path = std::env::temp_dir();
    path.push("mod_files.zip");
    std::fs::write(&path, &bytes)?;
    Ok(path)
}
pub fn extract_zip(
    zip_path: &std::path::Path,
    target_dir: &std::path::Path,
) -> anyhow::Result<InstallRecord> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut installed_files: Vec<PathBuf> = Vec::new();
    let mut created_dirs: HashSet<PathBuf> = HashSet::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = target_dir.join(file.name());

        if file.is_dir() {
            std::fs::create_dir_all(&out_path)?;
            created_dirs.insert(out_path.clone());
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
                created_dirs.insert(parent.to_path_buf());
            }

            if out_path.exists() {
                std::fs::remove_file(&out_path)?;
            }

            let mut out_file = std::fs::File::create(&out_path)?;
            std::io::copy(&mut file, &mut out_file)?;

            installed_files.push(out_path);
        }
    }

    Ok(InstallRecord {
        installed_files,
        created_dirs: created_dirs
            .into_iter()
            .collect(),
    })
}

pub fn get_install_path(
    install_steam: bool,
    install_epic: bool,
    install_custom: bool,
    custom_path: &str,
) -> anyhow::Result<std::path::PathBuf> {
    if install_steam {
        if install_custom {
            Ok(custom_path.into())
        } else {
            Ok("C:/Program Files (x86)/Steam/steamapps/common/Among Us".into())
        }
    } else if install_epic {
        if install_custom {
            Ok(custom_path.into())
        } else {
            Ok("C:/Program Files/Epic Games/AmongUs".into())
        }
    } else {
        Ok(custom_path.into())
    }
}
pub fn install_mod(
    repo: &str,
    install_steam: bool,
    install_epic: bool,
    install_custom: bool,
    custom_path: &str,
) -> anyhow::Result<InstallRecord> {
    let zip_url = fetch_latest_release_zip(repo, install_steam, install_epic)?;
    let zip_path = download_file(&zip_url)?;
    let install_dir = get_install_path(
        install_steam,
        install_epic,
        install_custom,
        custom_path,
    )?;

    let record = extract_zip(&zip_path, &install_dir)?;
    Ok(record)
}

pub fn clean_install(
    record: &InstallRecord,
    install_root: &Path,
) -> anyhow::Result<()> {
    for file in &record.installed_files {
        if file.exists() {
            std::fs::remove_file(file)?;
        }
    }

    let mut dirs = record
        .created_dirs
        .clone();
    dirs.sort_by(|a, b| {
        b.components()
            .count()
            .cmp(
                &a.components()
                    .count(),
            )
    });

    for dir in dirs {
        if dir == install_root {
            continue;
        }

        if !dir.starts_with(install_root) {
            continue;
        }

        if dir.exists() {
            let _ = std::fs::remove_dir(dir);
        }
    }

    Ok(())
}
