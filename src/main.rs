//! Discover Bluetooth devices and list them.

use bluer::{AdapterEvent, Address};
use futures::{pin_mut, StreamExt};
use std::{
    collections::{HashMap, HashSet},
    env,
    time::Duration,
};
use tokio::time;

#[tokio::main]
async fn main() -> bluer::Result<()> {
    tracing_subscriber::fmt::init();
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    println!(
        "Discovering devices using Bluetooth adapater {}\n",
        &adapter_name
    );
    let adapter = session.adapter(adapter_name)?;
    adapter.set_powered(true).await?;

    let mut device_events = adapter.discover_devices().await?;

    println!("bruh");

    let mut devices = HashMap::new();

    while let Some(event) = device_events.next().await {
        match event {
            AdapterEvent::DeviceAdded(addr) => {
                let device = adapter.device(addr)?;
                if let Some(rssi) = device.rssi().await? {
                    devices.insert(addr, rssi);
                }
            }
            AdapterEvent::DeviceRemoved(addr) => {
                devices.remove(&addr);
            }
            _ => {}
        }

        if let Some((addr, rssi)) = devices.iter().max_by_key(|(_, rssi)| *rssi) {
            let device = adapter.device(*addr)?;
            println!("{}", device.alias().await?);
        }

        println!("{} devices", devices.len());
    }

    // while let Some(event) = device_events.next().await {
    //     match event {
    //         AdapterEvent::DeviceAdded(addr) => {
    //             devices.insert(addr, );
    //         }
    //         AdapterEvent::DeviceRemoved(addr) => {
    //             devices.remove(&addr);
    //         }
    //         _ => {}
    //     }

    //     println!("{}", devices.len());
    // }

    // loop {
    //     if let Some(AdapterEvent::DeviceAdded(addr)) = device_events.next().await {
    //         if !filter_addr.is_empty() && !filter_addr.contains(&addr) {
    //             continue;
    //         }

    //         let device = adapter.device(addr).unwrap();
    //         let rssi = device.rssi().await.unwrap();

    //         println!("{} {:?} dB", addr, rssi);
    //     }
    // }

    Ok(())
}
