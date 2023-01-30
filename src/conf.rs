use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeConfig{
    pub dbname                : String,
    pub api_call_timeout      : u64,
    pub tsharkfile            : String 
}

impl NodeConfig {
    pub fn longcall(self) -> u64{
        let long_timeout = self.api_call_timeout * 2;
        return long_timeout;
    }
}


impl ::std::default::Default for NodeConfig {
    fn default() -> Self { Self { dbname:"accs.db".into()
                                , api_call_timeout:30
                                , tsharkfile:"/tmp/my.pcap".into()
                        }}
}


pub fn load_config(confname: &str) -> NodeConfig {
    let mcontent = std::fs::read_to_string(confname);
    let dcfg: NodeConfig = Default::default();
    match mcontent {
        Ok(cont) => {
            let mcfg = serde_json::from_str(&cont);
            match mcfg {
                Ok(cfg) => { return cfg;}
                Err(_)  => { println!("file parsing err, using default config"); return dcfg;}
            }
        }
        Err(_)   => { println!("file loading err, using default config"); return dcfg;}
    }
}
