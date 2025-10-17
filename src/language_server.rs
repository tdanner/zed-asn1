use std::fs;
use std::path::{Path, PathBuf};

use zed::{
    Architecture, GithubReleaseOptions, LanguageServerId, LanguageServerInstallationStatus, Os,
    current_platform,
};
use zed_extension_api::{self as zed, Result};

pub struct Asn1LanguageServer {
    cached_binary_path: Option<String>,
}

impl Asn1LanguageServer {
    pub const LANGUAGE_SERVER_ID: &'static str = "asn1";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("asn1-lsp") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path
            && fs::metadata(path).is_ok_and(|stat| stat.is_file())
        {
            return Ok(path.clone());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "tdanner/asn1-lsp",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = current_platform();
        let version_dir = format!("{}-{}", Self::LANGUAGE_SERVER_ID, release.version);
        let binary_name = expected_binary_name(platform);

        if let Some(path) = find_existing_binary(&version_dir, binary_name) {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        let asset = select_asset(&release.assets, platform, arch).ok_or_else(|| {
            format!("no release asset found for platform {platform:?} and architecture {arch:?}")
        })?;

        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory {version_dir}: {err}"))?;

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        let download_type = downloaded_file_type(&asset.name);
        match download_type {
            zed::DownloadedFileType::Uncompressed => {
                let download_path = format!("{version_dir}/{binary_name}");
                zed::download_file(&asset.download_url, &download_path, download_type)
                    .map_err(|err| format!("failed to download file: {err}"))?;
            }
            zed::DownloadedFileType::Zip => {
                zed::download_file(&asset.download_url, &version_dir, download_type)
                    .map_err(|err| format!("failed to download file: {err}"))?;
            }
            zed::DownloadedFileType::Gzip => {
                let download_path = format!("{version_dir}/{binary_name}");
                zed::download_file(&asset.download_url, &download_path, download_type)
                    .map_err(|err| format!("failed to download file: {err}"))?;
            }
            zed::DownloadedFileType::GzipTar => {
                zed::download_file(&asset.download_url, &version_dir, download_type)
                    .map_err(|err| format!("failed to download file: {err}"))?;
            }
        }

        let binary_path = find_existing_binary(&version_dir, binary_name).ok_or_else(|| {
            format!("downloaded archive did not contain expected {binary_name} binary for asn1-lsp")
        })?;

        zed::make_file_executable(&binary_path)?;
        remove_outdated_versions(Self::LANGUAGE_SERVER_ID, &version_dir)?;

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

fn expected_binary_name(platform: Os) -> &'static str {
    match platform {
        Os::Windows => "asn1-lsp.exe",
        Os::Mac | Os::Linux => "asn1-lsp",
    }
}

fn select_asset(
    assets: &[zed::GithubReleaseAsset],
    platform: Os,
    arch: Architecture,
) -> Option<&zed::GithubReleaseAsset> {
    let os_keywords: &[&str] = match platform {
        Os::Mac => &["macos", "darwin", "apple"],
        Os::Linux => &["linux"],
        Os::Windows => &["windows", "win32", "win64"],
    };

    let arch_keywords: &[&str] = match arch {
        Architecture::Aarch64 => &["aarch64", "arm64"],
        Architecture::X8664 => &["x86_64", "amd64"],
        Architecture::X86 => &["x86", "i686"],
    };

    assets.iter().find(|asset| {
        let asset_name = asset.name.to_lowercase();
        asset_name.contains("asn1")
            && os_keywords
                .iter()
                .any(|keyword| asset_name.contains(keyword))
            && arch_keywords
                .iter()
                .any(|keyword| asset_name.contains(keyword))
    })
}

fn downloaded_file_type(asset_name: &str) -> zed::DownloadedFileType {
    let name = asset_name.to_ascii_lowercase();
    if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        zed::DownloadedFileType::GzipTar
    } else if name.ends_with(".gz") {
        zed::DownloadedFileType::Gzip
    } else if name.ends_with(".zip") {
        zed::DownloadedFileType::Zip
    } else {
        zed::DownloadedFileType::Uncompressed
    }
}

fn find_existing_binary(version_dir: &str, binary_name: &str) -> Option<String> {
    let version_path = Path::new(version_dir);
    if let Some(path) = try_candidate_path(version_path.join(binary_name)) {
        return Some(path);
    }

    find_binary_recursively(version_path, binary_name)
}

fn try_candidate_path(candidate: PathBuf) -> Option<String> {
    if fs::metadata(&candidate).is_ok_and(|stat| stat.is_file()) {
        candidate.to_str().map(|path| path.to_string())
    } else {
        None
    }
}

fn find_binary_recursively(root: &Path, binary_name: &str) -> Option<String> {
    if !root.exists() {
        return None;
    }

    let Ok(entries) = fs::read_dir(root) else {
        return None;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.file_name().is_some_and(|name| name == binary_name) {
            if let Some(utf8_path) = path.to_str() {
                return Some(utf8_path.to_string());
            }
        } else if path.is_dir()
            && let Some(found) = find_binary_recursively(&path, binary_name)
        {
            return Some(found);
        }
    }

    None
}

fn remove_outdated_versions(language_server_id: &'static str, version_dir: &str) -> Result<()> {
    let entries =
        fs::read_dir(".").map_err(|err| format!("failed to list working directory: {err}"))?;

    for entry in entries {
        let entry = entry.map_err(|err| format!("failed to load directory entry: {err}"))?;
        if entry.file_name().to_str().is_none_or(|file_name| {
            file_name.starts_with(language_server_id) && file_name != version_dir
        }) {
            let _ = fs::remove_dir_all(entry.path());
        }
    }

    Ok(())
}
