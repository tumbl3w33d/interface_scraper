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
    fn it_works() {
        let ifaces = ifconfig::get_interfaces();
        assert!(ifaces.is_ok(), "No interfaces have been parsed");
        let ifaces = ifaces.unwrap();
        assert!(ifaces.len() > 0);
        let first_iface = ifaces.first().unwrap();
        assert!(first_iface.name() != "");
        assert!(first_iface.netmask().is_some());
    }
}
