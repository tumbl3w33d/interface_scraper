
use std::net::IpAddr;

#[derive(Default, Clone, Getters)]
pub struct Interface {
    #[get = "pub"]
    name: String,
    gateway: Option<IpAddr>,
    #[get = "pub"]
    netmask: Option<IpAddr>,
    ip: Option<IpAddr>,
    dns1: Option<IpAddr>,
    dns2: Option<IpAddr>,
    domain: Option<String>
}

pub struct InterfaceBuilder {
    iface: Interface
}

impl Interface {
    pub fn builder() -> InterfaceBuilder {
        InterfaceBuilder::new()
    }
}

impl InterfaceBuilder {
    fn new() -> InterfaceBuilder {
        let iface = Interface { ..Default::default() };
        InterfaceBuilder { iface }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.iface.name = name;
        self
    }

    pub fn gateway(&mut self, ip: IpAddr) -> &mut Self {
        self.iface.gateway = Some(ip);
        self
    }

    pub fn netmask(&mut self, ip: IpAddr) -> &mut Self {
        self.iface.netmask = Some(ip);
        self
    }

    pub fn ip(&mut self, ip: IpAddr) -> &mut Self {
        self.iface.ip = Some(ip);
        self
    }

    pub fn dns1(&mut self, ip: IpAddr) -> &mut Self {
        self.iface.dns1 = Some(ip);
        self
    }

    pub fn dns2(&mut self, ip: IpAddr) -> &mut Self {
        self.iface.dns2 = Some(ip);
        self
    }

    pub fn domain(&mut self, domain: String) -> &mut Self {
        self.iface.domain = Some(domain);
        self
    }

    pub fn build(&self) -> Interface {
        self.iface.to_owned()
    }
}
