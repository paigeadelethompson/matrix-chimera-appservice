use std::convert::Infallible;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Read, Write};
use std::iter;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use aws_config::SdkConfig;
use aws_sdk_chimesdkmessaging::Client;
use aws_sdk_chimesdkmessaging::config::{AppName, Credentials, SharedCredentialsProvider};
use clap::Parser;
use hyper::{http, Uri};
use ruma::api::appservice::{Registration, RegistrationInit};
use ruma::events::AnyEphemeralRoomEvent;
use ruma::serde::Raw;
use ruma::ServerName;
use serde::de::IntoDeserializer;
use serde::Serialize;
use crate::configuration::Configuration;
use crate::service::{ApplicationService, serve};


mod configuration;
mod service;
mod request;
mod matrix;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "chimera",
    long_about = "Amazon Chime Matrix bridging service"
)]
pub(crate) struct Arguments {
    #[clap(long = "generate-configuration", short = 'g', action)]
    generate_configuration: bool,
    #[clap(long = "generate-registration", short = 'G', action)]
    generate_registration: bool,
    #[clap(short = 'c', long, default_value = None)]
    configuration_file: Option<String>,
}

#[::tokio::main]
async fn main()  {
    let args: Arguments = Arguments::parse();

    if args.generate_configuration {
        let file_name: String = match args.configuration_file {
            None => "configuration.toml".to_string(),
            Some(c) => c.to_string()
        };

        let default_config = Configuration::new();

        let serialized = match toml::to_string(&default_config) {
            Ok(r) => r,
            Err(_) => todo!()
        };

        match File::open(file_name.clone()) {
            Ok(mut f) => {
               match f.write_all(serialized.as_bytes()) {
                   Ok(_) => {
                       println!("default configuration file written to {}", file_name);
                   },
                   Err(_) => todo!()
               }
            }
            Err(_) => todo!()
        }
    } else if args.generate_registration {
        let mut config = get_configuration(args).await;

        let reg = serde_json::to_string(&Registration::from(RegistrationInit {
            id: config.clone().unique_id.unwrap(),
            url: Option::from(config.clone().public_address.unwrap()),
            as_token: config.clone().home_server_configuration.unwrap().as_token.unwrap(),
            hs_token: config.clone().home_server_configuration.unwrap().hs_token.unwrap(),
            sender_localpart: config.clone().home_server_configuration.unwrap().domain.unwrap(),
            namespaces: Default::default(),
            rate_limited: None,
            protocols: None,
        })).expect("serializing registration");

        println!("\n{reg}\n", reg=reg);
    } else if args.configuration_file.is_some() {
        let config = get_configuration(args).await;

        /// XXX also need the TLS listeners
        let bind_addresses: Box<[SocketAddr]> = config.bind_address.clone().unwrap().iter().filter(
            |b| {
                b.tls.is_none()
            }).map(|b| {
            SocketAddr::new(
                IpAddr::from_str(&*b.clone().hostname.unwrap()).unwrap(),
                b.clone().port.unwrap())
        }).collect();

        let matrix_waiter = serve(
            &*bind_addresses, serve_handler);

        let chime = Client::new(
            &SdkConfig::builder().app_name(AppName::new(
                config.chime_configuration.clone().unwrap().application_name
                    .unwrap()).unwrap()).credentials_provider(
                SharedCredentialsProvider::new(Credentials::new(
                    config.clone().chime_configuration.clone().unwrap()
                        .credentials.unwrap().access_key_id.unwrap(),
                    config.clone().chime_configuration.clone().unwrap()
                        .credentials.unwrap().secret_access_key.unwrap(),
                    None,
                    None,
                    "ChimeProvider"))).build());
        // https://github.com/aws-samples/amazon-chime-sdk/tree/main/apps/chat#sending-messages
        todo!()
    } else {
        todo!()
    }
}

async fn serve_handler(s: String, event: Vec<Raw<AnyEphemeralRoomEvent>>)
    -> Result<String, Infallible> {
    todo!()
}

async fn get_configuration(args: Arguments) -> Configuration {
    match File::open(args.configuration_file.unwrap()) {
        Ok(mut f) => {
            let mut serialized: Vec<u8> = Vec::new();

            match f.read_to_end(&mut serialized) {
                Ok(_) => match toml::from_str::<Configuration>(
                    &String::from_utf8(serialized).unwrap()) {
                    Ok(c) => c,
                    Err(_) => todo!()
                },
                Err(_) => todo!()
            }
        }
        Err(_) => todo!()
    }
}