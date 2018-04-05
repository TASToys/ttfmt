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
    platform_meta:Vec<serde_json::Value>,
}

impl ChatFmt {
    pub fn new(platform:String) -> Self {
        ChatFmt {
            platform: platform,
            channel: String::new(),
            timestamp: String::new(),
            sender: String::new(),
            message: String::new(),
            split_msg: Vec::new(),
            platform_meta: Vec::new(),
        }
    }

    pub fn fmt_chat_msg(mut self, channel:String, sender:String, msg:String, psm:Vec<serde_json::Value>) -> ChatFmt {
        self.channel = channel;
        let time = SystemTime::now();
        let unix_time = time.duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.timestamp = unix_time.to_string();
        self.sender = sender;
        self.message = msg;
        self.split_msg = self.message.split(" ").map(|s| s.to_string()).collect();
        self.platform_meta = psm;

        self
    }

    pub fn build(self) -> ChatFmt {
        ChatFmt {
            platform: self.platform,
            channel: self.channel,
            timestamp: self.timestamp,
            sender: self.sender,
            message: self.message,
            split_msg: self.split_msg,
            platform_meta: self.platform_meta,
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