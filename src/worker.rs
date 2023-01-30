use std::time::Duration;
use crate::{types::*, db_functions::{db_insert_calls_ip, db_truncate_calls_ip}};

use rusqlite::{params,Connection, Result};

pub async fn tshark_worker(
    polling_sleep: Duration,
    dbname: String,
    tsharkfile: String
) -> () {
    loop {
        {
            match update_stage(&dbname,&tsharkfile).await {
                Ok(str) => {
                    log::info!("Debug, got Ok(): {str}");
                }
                Err(e) => {
                    log::warn!("Error: Failed to call: {e}");
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            }
        }
        tokio::time::sleep(polling_sleep).await;
    }
}

pub async fn update_stage(dbname: &str, tsharkfile: &str) -> Result<String, rusqlite::Error> {

    log::info!("starting tshark");
    db_truncate_calls_ip(dbname)?;
    // Creates a builder with needed tshark parameters
    let builder = rtshark::RTSharkBuilder::builder()
        .input_path(tsharkfile);

    // Start a new tshark process
    let mut rtshark = builder.spawn()
        .unwrap_or_else(|e| panic!("Error starting tshark: {e}"));

    // read packets until the end of the PCAP file
    while let Some(packet) = rtshark.read().unwrap_or_else(|e| {
        log::warn!("Error parsing tshark output: {e}");
        None
    }) {
        log::info!("packet:",);
        let mut ip : String = "1.1.1.1".to_owned();
        let mut target_ip : String = "2.2.2.2".to_owned();
        let mut description : String = "empty".to_owned();
        for layer in packet {
            //log::info!("Layer: {}", layer.name());

            if layer.name() == "ip" {
            for metadata in layer.clone() {
               if metadata.name() == "ip.src" {
                   ip = metadata.value().to_owned();
               } 
               if metadata.name() == "ip.dst" {
                   target_ip = metadata.value().to_owned();
               } 
            }
            }
            if layer.name() == "dns" {
                for metadata in layer {
                log::info!("\t{}",metadata.display());
                   if metadata.name() == "dns.resp.name"{
                       
                       description = metadata.value().to_owned();
                   } 
                }
            }
        }
        let call = CallIpLayer{
            id: 1,
            ip,
            layer: "ip".to_owned(),
            target_ip,
            description
        };
        db_insert_calls_ip(dbname, call)?;
        ()
    }

    return Ok("".into());
}
