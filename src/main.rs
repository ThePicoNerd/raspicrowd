use std::time::Duration;

use bluez::{
    client::{AddressTypeFlag, BlueZClient},
    interface::event::Event,
};

#[tokio::main]
async fn main() {
    let mut client = BlueZClient::new().unwrap();

    let controllers = client.get_controller_list().await.unwrap();
    let controller = controllers
        .first()
        .expect("no bluetooth controllers available");

    client.set_handler(|controller, event| match event {
        Event::DeviceFound {
            address,
            address_type,
            flags,
            rssi,
            ..
        } => {
            println!(
                "[{:?}] found device {} ({:?})",
                controller, address, address_type
            );
            println!("\tflags: {:?}", flags);
            println!("\trssi: {:?}", rssi);
        }
        _ => (),
    });

    client
        .start_discovery(
            *controller,
            AddressTypeFlag::BREDR | AddressTypeFlag::LEPublic | AddressTypeFlag::LERandom,
        )
        .await
        .unwrap();

    for _ in 0usize..5000usize {
        client.process().await.unwrap();
        std::thread::sleep(Duration::from_millis(50));
    }
}
