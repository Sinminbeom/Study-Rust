trait Device {
    fn get_ip() -> String;
    fn get_port() -> String;
    fn get_protocol() -> String;
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
impl Device for CPU {
    fn get_ip() -> String {todo!()}
    fn get_port() -> String {todo!()}
    fn get_protocol() -> String {todo!()}
}

impl Device for VPU1 {
    fn get_ip() -> String {todo!()}
    fn get_port() -> String {todo!()}
    fn get_protocol() -> String {todo!()}
}

impl Device for VPU2 {
    fn get_ip() -> String {todo!()}
    fn get_port() -> String {todo!()}
    fn get_protocol() -> String {todo!()}
}

impl Device for VPU3 {
    fn get_ip() -> String {todo!()}
    fn get_port() -> String {todo!()}
    fn get_protocol() -> String {todo!()}
}
