extern crate serde;
extern crate serde_json;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct FidoFmt {
    plugin_id: i32,
    plugin_type: i32,
    source_platform: String,
    source_channel:String,
    timestamp:String,
    source_user:String,
    data: Vec<serde_json::Value>,
}

impl FidoFmt {
    pub fn new() -> Self {
        FidoFmt {
            plugin_id: 0,
            plugin_type: 0,
            source_platform: String::new(),
            source_channel: String::new(),
            timestamp: String::new(),
            source_user: String::new(),
            data: Vec::new(),
        }
    }

    pub fn fmt_fido_msg(mut self, id:i32, p_type:i32, sp:String, sc:String, su:String, data: Vec<serde_json::Value>) -> FidoFmt {
        self.plugin_id = id;
        self.plugin_type = p_type;
        self.source_platform = sp;
        self.source_channel = sc;
        self.source_user = su;
        self.data = data;

        let time = SystemTime::now();
        let unix_time = time.duration_since(UNIX_EPOCH).unwrap().as_secs();

        self.timestamp = unix_time.to_string();

        self
    }

    pub fn build(self) -> FidoFmt {
        FidoFmt {
            plugin_id: self.plugin_id,
            plugin_type: self.plugin_type,
            timestamp: self.timestamp,
            source_platform: self.source_platform,
            source_channel: self.source_channel,
            source_user: self.source_user,
            data: self.data,
        }
    }
}

pub fn to_json(msg:FidoFmt) -> String {
    let fmt = FidoFmt {
        plugin_id: msg.plugin_id,
        plugin_type: msg.plugin_type,
        timestamp: msg.timestamp,
        source_platform: msg.source_platform,
        source_channel: msg.source_channel,
        source_user: msg.source_user,
        data: msg.data
    };

    let out = serde_json::to_string(&fmt).expect("error converting fmt to string");

    out
}