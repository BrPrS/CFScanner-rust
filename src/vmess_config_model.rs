use serde_derive::{Deserialize, Serialize};

/*

Inbound Structs

*/

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct VmessConfig{
    pub inbounds:Vec<Inbound>,
    pub outbounds:Vec<Outbound>,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Inbound{
    pub port:u16,
    pub listen:String,
    pub tag:String,
    pub protocol:String,
    pub settings:InboundSetting,
    pub sniffing:Snifing
}
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct InboundSetting{
    pub auth:String,
    pub udp:bool,
    pub ip:String
}
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Snifing{
    pub enabled:bool,
    pub destOverride:Vec<String>
}

/*

Outbound Structs

*/
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Outbound {
    pub protocol:String,
    pub settings:OutboundSetting,
    pub streamSettings:StreamSetting
}
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct OutboundSetting{
    pub vnext:Vec<Vnext>
}
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Vnext{
    pub address:String,
    pub port:u16,
    pub users:Vec<User>
}
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct User{
    pub id:String
}
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct StreamSetting{

    pub network:String,
    pub security:String,
    pub wsSettings:WsSetting,
    pub tlsSettings:TlsSetting
}
#[derive(Deserialize, Serialize, Debug,Clone)]

pub struct WsSetting{
    pub headers:Headers,
    pub path:String
}
#[derive(Deserialize, Serialize, Debug,Clone)]

pub struct Headers{
    pub host:String
}
#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct TlsSetting{
    pub serverName:String,
    pub allowInsecure:bool
}