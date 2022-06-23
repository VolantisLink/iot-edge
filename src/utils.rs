use pnet::datalink::{self, NetworkInterface};


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
