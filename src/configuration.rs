use serde::{Deserialize, Serialize};
use crate::service::random_alphanumeric;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) struct AWSCredentials {
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) struct DynamoDBConfiguration {
    pub credentials: Option<AWSCredentials>,
    pub end_point: Option<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) struct PostgreSQLDBConfiguration {
    pub connection_string: Option<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) enum DatabaseConfiguration {
    DynamoDB(DynamoDBConfiguration),
    PostgreSQL(PostgreSQLDBConfiguration)
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) struct HomeserverConfiguration {
    pub address: Option<String>,
    pub domain: Option<String>,
    pub as_token: Option<String>,
    pub hs_token: Option<String>,
    pub display_name_template: Option<String>,
    pub device_name: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) struct BindAddressConfiguration {
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) struct ChimeConfiguration {
    pub credentials: Option<AWSCredentials>,
    pub application_name: Option<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) struct Configuration {
    pub chime_configuration: Option<ChimeConfiguration>,
    pub database: Option<DatabaseConfiguration>,
    pub home_server_configuration: Option<HomeserverConfiguration>,
    pub address: Option<String>,
    pub public_address: Option<String>,
    pub bind_address: Option<Vec<BindAddressConfiguration>>,
    pub unique_id: Option<String>,
    pub user_name_template: Option<String>,
}

impl Configuration { 
    pub fn new() -> Configuration {
        Configuration {
            chime_configuration: Option::from(ChimeConfiguration{
                credentials: Option::from(AWSCredentials {
                    access_key_id: None,
                    secret_access_key: None,
                }),
                application_name: Some("chimera".to_string()),
            }),
            database: Option::from(DatabaseConfiguration::PostgreSQL(PostgreSQLDBConfiguration {
                connection_string: Some("postgres://user:password@host/chimera?sslmode=disable".to_string()),
            })),
            home_server_configuration: Option::from(HomeserverConfiguration {
                as_token: Option::from(random_alphanumeric(64)),
                hs_token: Option::from(random_alphanumeric(64)),
                display_name_template: Some("{{.ProfileName}}".to_string()),
                device_name: None,
                address: Some("matrix.example.com".to_string()),
                domain: Some("example.com".to_string())
            }),
            address: Some("http://chimera.example.com".to_string()),
            public_address: Some("https://chimera.example.com".to_string()),
            bind_address: Option::from(vec![
                    BindAddressConfiguration {
                        hostname: Some("0.0.0.0".to_string()),
                        port: Some(80),
                        tls: Some(false),
                    }, BindAddressConfiguration {
                         hostname: Some("0.0.0.0".to_string()),
                         port: Some(443),
                         tls: Some(true),
                     }]),
            unique_id: Some("chimera".to_string()),
            user_name_template: Some("{{ .DisambiguatedName }}".to_string()),
        }
    }
}