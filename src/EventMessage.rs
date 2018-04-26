extern crate serde;
extern crate serde_json;

use std::time::{SystemTime, UNIX_EPOCH};

use std::option;

#[derive(Serialize, Deserialize)]
pub struct EventMessage {
    pub platform:String,
    pub channel:String,
    pub timestamp:String,
    pub sender:String,
    pub message:String,
    pub split_msg:Vec<String>,
    pub platform_meta: serde_json::Value,
    pub plugin:Option<Plugin>,
}

#[derive(Serialize, Deserialize)]
pub struct Plugin {
    pub plugin_id:i32,
    pub trigger:String,
    pub data: serde_json::Value
}

impl EventMessage {
    pub fn plugin_msg(platform:String, channel:String, sender:String, message:String,
               platform_meta:serde_json::Value, plugin:Option<Plugin>) -> Self {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

        //Because rust is insane
        let m2 = message.clone();
        EventMessage {
            platform,
            channel,
            timestamp: time,
            sender,
            message,
            split_msg: m2.clone().split(" ").map(|s| s.to_string()).collect(),
            platform_meta,
            plugin
        }
    }

    pub fn null_msg(platform:String, channel:String, sender:String, message:String,
                      platform_meta:serde_json::Value) -> Self {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

//        let plug: Plugin ={};

        //Because rust is insane
        let m2 = message.clone();
        EventMessage {
            platform,
            channel,
            timestamp: time,
            sender,
            message,
            split_msg: m2.clone().split(" ").map(|s| s.to_string()).collect(),
            platform_meta,
            plugin: None,
        }
    }
}

pub fn to_json(msg:EventMessage) -> String {
    let fmt = EventMessage {
        platform: msg.platform,
        channel: msg.channel,
        timestamp: msg.timestamp,
        sender: msg.sender,
        message: msg.message,
        split_msg: msg.split_msg,
        platform_meta: msg.platform_meta,
        plugin: msg.plugin

    };

    let out = serde_json::to_string(&fmt).expect("error converting fmt to string");

    out
}