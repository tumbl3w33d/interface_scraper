#[macro_use]
extern crate lazy_static;

mod ifconfig;
mod interface;
mod netmask;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(ifconfig::get_interfaces().unwrap().len() > 0)
    }
}
