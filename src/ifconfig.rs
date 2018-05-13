extern crate regex;

use std::io::{Error, ErrorKind};
use std::process::{Command, Output};
use std::net::{IpAddr, Ipv4Addr};
use self::regex::Regex;

use interface::Interface;
use ::netmask::get_netmask_for_cidr;

pub fn get_interfaces() -> Result<Vec<Interface>, Error> {
    let mut cmd = Command::new("/bin/ip");
    cmd.arg("-f");
    cmd.arg("inet");
    cmd.arg("-o");
    cmd.arg("a");
    match cmd.output() {
        Ok(output) => {
            analyze_output(&output)
        },
        Err(e) => Err(e)
    }
}

fn analyze_output(output: &Output) -> Result<Vec<Interface>, Error> {
    if output.status.success() {
        let string_to_analyze = String::from_utf8_lossy(&output.stdout);
        let mut ret: Vec<Interface> = Vec::new();
        let lines = string_to_analyze.split("\n").filter(|l| !l.is_empty());
        for l in lines.into_iter() {
            ret.push(extract_interface_from_line(l))
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

fn extract_interface_from_line(line: &str) -> Interface {
    println!("{:?}", line);
    get_netmask_for_cidr("foo");
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut builder = Interface::builder();
    builder.ip(ip).build()
}
