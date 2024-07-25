use std::collections::HashMap;

use flurl::my_ssh::SshCredentials;

use crate::{settings::SshCredentialsSettingsModel, ssh_certs_cache::SshCacheCerts};

lazy_static::lazy_static! {
    pub static ref SSH_CERTS_CACHE: SshCacheCerts = {
        SshCacheCerts::new()
    };
}

pub async fn parse_url<'s>(
    url: &str,
    ssh_credentials: SshCredentialsSettings<'s>,
) -> (Option<SshCredentials>, String) {
    if !url.starts_with("ssh") {
        return (None, url.to_string());
    }

    let mut url_left_part = None;
    let mut url_right_part = None;

    for itm in url.split("->").map(|itm| itm) {
        if url_left_part.is_none() {
            url_left_part = Some(itm);
            continue;
        }

        if url_right_part.is_none() {
            url_right_part = Some(itm);
            continue;
        }

        panic!("Invalid ssh url '{}'", url);
    }

    if url_right_part.is_none() {
        panic!("Invalid ssh url '{}'", url);
    }

    let ssh_id = extract_ssh_id(url_left_part.unwrap());
    println!("ssh_id: {}", ssh_id);

    let ssh_items = url_left_part.unwrap().split("@").collect::<Vec<_>>();

    let mut left_part: Vec<_> = ssh_items[0].split(":").collect();

    let mut right_part: Vec<_> = ssh_items[1].split(":").collect();

    let ssh_remote_host = right_part.remove(0).to_string();
    let ssh_remote_port = right_part.remove(0).to_string().parse().unwrap();
    let ssh_user_name = left_part.remove(1).to_string();

    if let Some(ssh_credentials) = ssh_credentials.get(ssh_id) {
        let private_key = SSH_CERTS_CACHE
            .get_cert(ssh_credentials.cert_path.as_str())
            .await;
        return (
            Some(SshCredentials::PrivateKey {
                ssh_remote_host,
                ssh_remote_port,
                ssh_user_name,
                private_key,
                passphrase: Some(ssh_credentials.cert_pass_prase.to_string()),
            }),
            url_right_part.unwrap().to_string(),
        );
    }

    let ssh_credentials = SshCredentials::SshAgent {
        ssh_remote_host,
        ssh_remote_port,
        ssh_user_name,
    };

    (Some(ssh_credentials), url_right_part.unwrap().to_string())
}

fn extract_ssh_id(ssh_part: &str) -> &str {
    match ssh_part.find(":") {
        Some(index) => &ssh_part[index + 1..],
        None => ssh_part,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SshCredentialsSettings<'s> {
    data: Option<&'s HashMap<String, SshCredentialsSettingsModel>>,
}

impl<'s> SshCredentialsSettings<'s> {
    pub fn new(data: Option<&'s HashMap<String, SshCredentialsSettingsModel>>) -> Self {
        SshCredentialsSettings { data }
    }

    pub fn get(&self, id: &str) -> Option<&SshCredentialsSettingsModel> {
        let data = self.data?;

        if let Some(value) = data.get(id) {
            return Some(value);
        }

        if let Some(value) = data.get("*") {
            return Some(value);
        }

        None
    }
}

impl<'s> Into<SshCredentialsSettings<'s>>
    for Option<&'s HashMap<String, SshCredentialsSettingsModel>>
{
    fn into(self) -> SshCredentialsSettings<'s> {
        SshCredentialsSettings::new(self)
    }
}
