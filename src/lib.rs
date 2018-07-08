#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate getset;

pub mod ifconfig;
pub mod interface;
pub mod netmask;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_any_interface() {
        let ifaces = ifconfig::get_interfaces();
        assert!(ifaces.is_ok(), "No interfaces have been parsed");
        let ifaces = ifaces.unwrap();
        assert!(ifaces.len() > 0);
        let first_iface = ifaces.first().unwrap();
        assert!(first_iface.name() != "");
        assert!(first_iface.netmask().is_some());
        assert!(first_iface.ip().is_some());
    }

    #[test]
    fn detects_wan_interface() {
        let ifaces = ifconfig::get_interfaces().expect("Failed to detect interfaces");
        assert!(ifaces.iter().any(|x| *x.is_wan_interface()));
        let wan_iface = ifaces.iter().find(|x| *x.is_wan_interface()).unwrap();
        assert!(wan_iface.gateway().is_some());
    }
}
