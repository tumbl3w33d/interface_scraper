
use std::net::IpAddr;

#[derive(Default, Clone, Getters, Debug)]
pub struct Interface {
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    gateway: Option<IpAddr>,
    #[get = "pub"]
    netmask: Option<IpAddr>,
    #[get = "pub"]
    ip: Option<IpAddr>,
    #[get = "pub"]
    dns1: Option<IpAddr>,
    #[get = "pub"]
    dns2: Option<IpAddr>,
    #[get = "pub"]
    domain: Option<String>,
    #[get = "pub"]
    is_wan_interface: bool
}

pub struct InterfaceBuilder {
    iface: Interface
}

impl Interface {
    pub fn builder() -> InterfaceBuilder {
        InterfaceBuilder::new()
    }

    pub fn as_wan_interface(&self) -> Interface {
        let mut ret = self.clone();
        ret.is_wan_interface = true;
        ret
    }

    pub fn with_gateway(&self, gateway: IpAddr) -> Interface {
        let mut ret = self.clone();
        ret.gateway = Some(gateway);
        ret
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

    pub fn is_wan_interface(&mut self, is_wan_iface: bool) -> &mut Self {
        self.iface.is_wan_interface = is_wan_iface;
        self
    }

    pub fn build(&self) -> Interface {
        self.iface.to_owned()
    }
}
