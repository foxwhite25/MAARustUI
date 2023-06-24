use crate::updater::api::Version;
use anyhow::Result;
use futures::SinkExt;

use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;

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

pub async fn query_repo_releases(version: Version) -> Result<RepoRelease> {
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

pub async fn check_update(version: &str, release_type: Version) -> Result<Asset> {
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
