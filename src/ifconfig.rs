extern crate regex;

use std::io::{Error, ErrorKind};
use std::process::{Command, Output};
use std::net::{IpAddr, Ipv4Addr};
use self::regex::Regex;

use interface::Interface;
use ::netmask::get_netmask_for_cidr;



pub fn get_interfaces() -> Result<Vec<Interface>, Error> {
    let mut cmd = Command::new("/bin/ip");
    cmd.args(&["-f", "inet", "-o", "address"]);
    let output = cmd.output()?;
    analyze_output(&output)
}

fn analyze_output(output: &Output) -> Result<Vec<Interface>, Error> {
    if output.status.success() {
        let string_to_analyze = String::from_utf8_lossy(&output.stdout);
        let mut ret: Vec<Interface> = Vec::new();
        let lines = string_to_analyze.split("\n").filter(|l| !l.is_empty());
        for l in lines.into_iter() {
            ret.push(extract_interface_from_line(l)?)
        }
        Ok(ret)
    } else {
        if let Some(code) =  output.status.code() {
            Err(Error::from_raw_os_error(code))
        } else {
            Err(Error::new(ErrorKind::Other, "Cannot get exit code of ip command execution (process must have been killed by signal)"))
        }
    }
}

fn extract_interface_from_line(line: &str) -> Result<Interface, Error> {
    //println!("{:?}", line);
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(?P<number>\d+):\s+(?P<name>[^\s]+)\s+(?P<family>[^\s]+)\s+(?P<cidr>[^\s]+).*"
        ).unwrap();
    }
    if let Some(captures) = RE.captures(line) {
        let mut octets: Vec<&str> = captures["cidr"].split(r".").collect();
        let last_oct_split: Vec<&str> = octets.last().unwrap().split("/").collect();
        let cidr_suffix = last_oct_split.last().unwrap();
        octets[3] = last_oct_split[0];
        let octets: Vec<u8> = octets.iter().map(|x| x.parse().unwrap()).collect();
        let ip = IpAddr::V4(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]));
        let mut builder = Interface::builder();
        builder.ip(ip)
               .name(captures["name"].to_owned());
        if let Some(netmask) = get_netmask_for_cidr(cidr_suffix) {
            let nm_octets: Vec<u8> = netmask.split(".").map(|x| x.parse().unwrap()).collect();
            builder.netmask(IpAddr::V4(Ipv4Addr::new(nm_octets[0], nm_octets[1], nm_octets[2], nm_octets[3])));
        }
        Ok(builder.build())
    } else {
        Err(Error::new(ErrorKind::Other, "Unable tokenize input line. Has the output format changed?"))
    }
}
