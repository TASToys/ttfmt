extern crate serde;
extern crate serde_json;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct ChatFmt {
    platform:String,
    channel:String,
    timestamp:String,
    sender:String,
    message:String,
    split_msg:Vec<String>,
    platform_meta: serde_json::Value,
}

impl ChatFmt {
    pub fn new(platform:String, channel:String, sender:String, message:String, psm: serde_json::Value) -> Self {

        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

        let m2 = message.clone();
        ChatFmt {
            platform,
            channel,
            timestamp: time,
            sender,
            message,
            split_msg: m2.clone().split(" ").map(|s| s.to_string()).collect(),
            platform_meta: psm,
        }
    }
}

pub fn to_json(msg:ChatFmt) -> String {
    let fmt = ChatFmt {
        platform: msg.platform,
        channel: msg.channel,
        timestamp: msg.timestamp,
        sender: msg.sender,
        message: msg.message,
        split_msg: msg.split_msg,
        platform_meta: msg.platform_meta
    };

    let out = serde_json::to_string(&fmt).expect("error converting fmt to string");

    out
}