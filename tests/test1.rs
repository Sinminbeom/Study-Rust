trait Device {
    fn get_ip(&self) -> &String;
    fn get_port(&self) -> &String;
    fn get_protocol(&self) -> &String;
}
struct BaseProperty {
    ip: String,
    port: String,
    protocol: String
}
struct CPU {
    base_property: BaseProperty
}
struct VPU1 {
    base_property: BaseProperty
}
struct VPU2 {
    base_property: BaseProperty
}
struct VPU3 {
    base_property: BaseProperty
}
impl CPU {
    pub fn new(base_property: BaseProperty) -> Self {
        Self { base_property }
    }
}
impl VPU1 {
    pub fn new(base_property: BaseProperty) -> Self {
        Self { base_property }
    }
}
impl VPU2 {
    pub fn new(base_property: BaseProperty) -> Self {
        Self { base_property }
    }
}
impl VPU3 {
    pub fn new(base_property: BaseProperty) -> Self {
        Self { base_property }
    }

}
impl Device for CPU {
    fn get_ip(&self) -> &String { &self.base_property.ip }
    fn get_port(&self) -> &String { &self.base_property.port }
    fn get_protocol(&self) -> &String { &self.base_property.protocol }
}

impl Device for VPU1 {
    fn get_ip(&self) -> &String { &self.base_property.ip }
    fn get_port(&self) -> &String { &self.base_property.port }
    fn get_protocol(&self) -> &String { &self.base_property.protocol }
}

impl Device for VPU2 {
    fn get_ip(&self) -> &String { &self.base_property.ip }
    fn get_port(&self) -> &String { &self.base_property.port }
    fn get_protocol(&self) -> &String { &self.base_property.protocol }
}

impl Device for VPU3 {
    fn get_ip(&self) -> &String { &self.base_property.ip }
    fn get_port(&self) -> &String { &self.base_property.port }
    fn get_protocol(&self) -> &String { &self.base_property.protocol }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cpu = CPU{
            base_property: BaseProperty {
                ip: "192.168.1.92".to_string(),
                port: "3030".to_string(),
                protocol: "udp".to_string(),
            }
        };
        // assert_eq!(cpu.base_property.ip, cpu.get_ip());
        println!("{}", cpu.get_ip());
    }
}