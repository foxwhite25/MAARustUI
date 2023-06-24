use crate::updater::api::Version;
use futures::SinkExt;

use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use futures::StreamExt;
use log::info;

pub type RepoReleases = Vec<RepoRelease>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepoRelease {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Asset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

pub fn is_nightly_version(version: &str) -> bool {
    if !version.contains('-') {
        return false;
    }
    let last_id = version.split('.').last().unwrap();
    if last_id.starts_with('g') && last_id.len() >= 7 {
        return true;
    }
    false
}

fn is_std_version(version: &str) -> bool {
    let string_check = version == "DEBUG VERSION"
        || version.starts_with('c')
        || version.starts_with("20")
        || version.contains("local")
        || is_nightly_version(version);
    !string_check
}

pub async fn query_repo_releases(version: Version) -> anyhow::Result<RepoRelease> {
    let mut headers = reqwest::header::HeaderMap::new();
    // 'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) '
    //                       'AppleWebKit/537.36 (KHTML, like Gecko) '
    //                       'Chrome/97.0.4692.99 '
    //                       'Safari/537.36 '
    //                       'Edg/97.0.1072.76'
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
        AppleWebKit/537.36 (KHTML, like Gecko) \
        Chrome/97.0.4692.99 \
        Safari/537.36 \
        Edg/97.0.1072.76"
            .parse()
            .unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let k = match version {
        Version::Stable => {
            let url =
                "https://api.github.com/repos/MaaAssistantArknights/MaaRelease/releases/latest";
            client.get(url).send().await?.json().await?
        }
        Version::Nightly => {
            let url = "https://api.github.com/repos/MaaAssistantArknights/MaaRelease/releases";
            let resp: RepoReleases = client.get(url).send().await?.json().await?;

            resp.into_iter()
                .find(|r| !is_std_version(&r.tag_name))
                .unwrap()
        }
    };
    Ok(k)
}

fn split_version(version: &str) -> Vec<u32> {
    println!("version: {}", version);
    let (pre, sub) = if version.contains('-') {
        let mut iter = version.split('-');
        (iter.next().unwrap(), iter.next().unwrap())
    } else {
        (version, "")
    };
    let mut pre = pre.split('.').collect::<Vec<_>>();
    pre[0] = pre[0].strip_prefix('v').unwrap_or(pre[0]);
    if !sub.is_empty() {
        let mut sub = sub.split('.').collect::<Vec<_>>();
        sub.pop();
        match sub[0] {
            "alpha" => sub[0] = "1",
            "beta" => sub[0] = "2",
            "rc" => sub[0] = "3",
            _ => {}
        }
        pre.append(&mut sub);
    }
    pre.iter().map(|s| s.parse().unwrap()).collect()
}

fn compare_version(a: &str, b: &str) -> Ordering {
    let b = split_version(b);
    let a = split_version(a);
    for i in 0..a.len() {
        if a[i] == b[i] {
            continue;
        }
        return a[i].cmp(&b[i]);
    }
    if b.len() > a.len() {
        return Ordering::Less;
    }
    Ordering::Equal
}

pub async fn check_update(version: &str, release_type: Version) -> anyhow::Result<Asset> {
    let resp = query_repo_releases(release_type).await;
    if let Ok(resp) = resp {
        let latest_version = resp.tag_name;
        if version == latest_version && release_type == Version::Nightly {
            return Err(anyhow::anyhow!("Version is equal to latest version"));
        }

        if compare_version(version, &latest_version).is_ge() {
            return Err(anyhow::anyhow!("Version is equal to latest version"));
        }

        Ok(resp
            .assets
            .into_iter()
            .map(|x| (x.name.to_lowercase(), x))
            .find(|(x, _asset)| {
                x.contains("ota")
                    && x.contains("win")
                    && x.contains(&format!("{}_{}", version, latest_version))
            })
            .unwrap()
            .1)
    } else {
        Err(anyhow::anyhow!("Failed to query repo releases"))
    }
}

pub async fn update(version: &str, release_type: Version) -> anyhow::Result<()> {
    let asset = check_update(version, release_type).await?;
    let url = asset.browser_download_url;
    let resp = reqwest::get(&url).await?;
    let mut file = tempfile::tempfile()?;
    let total_size = resp.content_length().unwrap_or(0);
    let mut stream = resp.bytes_stream();
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    while let Some(item) = stream.next().await {
        let item = item?;
        file.write_all(&item)?;
        pb.inc(item.len() as u64);
    }
    let mut zip = zip::ZipArchive::new(file)?;
    // Extract all files
    let unzip_dir = std::env::current_dir()?.join("unzip");
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => unzip_dir.join(path),
            None => continue,
        };

        info!("Extracting {} ...", outpath.display());

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::updater::api::Version;

    #[tokio::test]
    async fn test_check_update() {
        let current_version = "v4.19.1";
        let version = Version::Stable;
        let resp = check_update(current_version, version).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_update() {
        env_logger::builder().filter_level(log::LevelFilter::Info).init();
        let current_version = "v4.19.1";
        let version = Version::Stable;
        let resp = update(current_version, version).await;
        println!("{:?}", resp);
    }


    #[test]
    fn test_compare_version() {
        let a = "v4.19.1";
        let b = "v4.19.2";
        assert_eq!(compare_version(a, b), Ordering::Less);
    }

    #[test]
    fn test_is_std_version() {
        let version = "0.1.0";
        assert!(is_std_version(version));
    }
}
