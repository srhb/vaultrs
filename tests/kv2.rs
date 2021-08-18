mod common;

use common::VaultServer;
use serde::{Deserialize, Serialize};
use vaultrs::api::kv2::requests::SetSecretMetadataRequest;
use vaultrs::error::ClientError;
use vaultrs::kv2;

#[test]
fn test_delete_latest() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    let res = kv2::delete_latest(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
    );
    assert!(res.is_ok());
}

#[test]
fn test_delete_metadata() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    let res = kv2::delete_metadata(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
    );
    assert!(res.is_ok());
}

#[test]
fn test_delete_versions() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    let res = kv2::delete_versions(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    );
    assert!(res.is_ok());
}

#[test]
fn test_destroy_versions() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    let res = kv2::destroy_versions(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    );
    assert!(res.is_ok());
}

#[test]
fn test_list() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    let res = kv2::list(&server.client, endpoint.path.as_str(), "");
    assert!(res.is_ok());
    assert!(!res.unwrap().is_empty());
}

#[test]
fn test_read() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    let res = kv2::set(
        &server.client,
        endpoint.path.as_str(),
        "test",
        &endpoint.secret,
    );
    assert!(res.is_ok());

    let res = kv2::read::<TestSecret>(&server.client, endpoint.path.as_str(), "test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap().key, endpoint.secret.key);
}

#[test]
fn test_read_metadata() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    let res = kv2::read_metadata(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
    );
    assert!(res.is_ok());
    assert!(!res.unwrap().versions.is_empty());
}

#[test]
fn test_read_version() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let mut endpoint = setup(&server).unwrap();

    let res = kv2::set(
        &server.client,
        endpoint.path.as_str(),
        "test",
        &endpoint.secret,
    );
    assert!(res.is_ok());

    let old_value = endpoint.secret.key.clone();
    endpoint.secret.key = "newkey".to_string();
    let res = kv2::set(
        &server.client,
        endpoint.path.as_str(),
        "test",
        &endpoint.secret,
    );
    assert!(res.is_ok());

    let res = kv2::read_version::<TestSecret>(&server.client, endpoint.path.as_str(), "test", 1);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().key, old_value);
}

#[test]
fn test_set() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    let res = kv2::set(
        &server.client,
        endpoint.path.as_str(),
        "test",
        &endpoint.secret,
    );
    assert!(res.is_ok());
}

#[test]
fn test_set_metadata() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    let res = kv2::set_metadata(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        Some(SetSecretMetadataRequest::builder().cas_required(true)),
    );
    assert!(res.is_ok());
}

#[test]
fn test_undelete_versions() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    let res = kv2::delete_versions(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    );
    assert!(res.is_ok());

    let res = kv2::undelete_versions(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    );
    assert!(res.is_ok());
}

mod config {
    use crate::{setup, VaultServer};
    use vaultrs::{api::kv2::requests::SetConfigurationRequest, kv2::config};

    #[test]
    fn test_read() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let resp = config::read(&server.client, endpoint.path.as_str());

        assert!(resp.is_ok());
        assert_eq!(resp.unwrap().max_versions, 0);
    }

    #[test]
    fn test_set() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let versions: u64 = 100;
        let resp = config::set(
            &server.client,
            endpoint.path.as_str(),
            Some(
                SetConfigurationRequest::builder()
                    .max_versions(versions)
                    .delete_version_after("768h"),
            ),
        );

        assert!(resp.is_ok());
    }
}

#[derive(Debug)]
struct SecretEndpoint {
    pub path: String,
    pub name: String,
    pub secret: TestSecret,
}

#[derive(Debug, Deserialize, Serialize)]
struct TestSecret {
    key: String,
    password: String,
}

fn setup(server: &VaultServer) -> Result<SecretEndpoint, ClientError> {
    let path = "secret_test";
    let name = "test";
    let secret = TestSecret {
        key: "mykey".to_string(),
        password: "supersecret".to_string(),
    };
    let endpoint = SecretEndpoint {
        path: path.to_string(),
        name: name.to_string(),
        secret,
    };

    // Mount the PKI engine
    server.mount(path, "kv-v2")?;

    // Create a test secret
    kv2::set(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        &endpoint.secret,
    )?;

    Ok(endpoint)
}
