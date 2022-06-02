use pnet::datalink::{self, NetworkInterface};
use mac_address::get_mac_address;


pub fn can_devices() -> Vec<String> {
    let iface_match =
    |iface: &NetworkInterface| {
        !iface.is_loopback() & iface.is_up() & iface.name.contains("can")
    };

    datalink::linux::interfaces()
        .into_iter()
        .filter(iface_match)
        .map(|iface| iface.name)
        .collect()
}

pub fn sn() -> [u8; 8] {
    let mut data: [u8; 8] = [0; 8];
    if let Some(mac) = get_mac_address().unwrap() {
        for (i, v) in mac.bytes().iter().enumerate() {
            data[i] = *v;
        }
    }

    data
}

#[test]
fn test_mac() {
    let mac = get_mac_address().unwrap().unwrap();
    println!("{:#?}", mac.bytes());
}