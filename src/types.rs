use serde::{Deserialize, Serialize};
use schemars::JsonSchema;


#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CallIpLayer{
  pub id          : u64
 ,pub ip          : String
 ,pub layer       : String
 ,pub target_ip   : String
 ,pub description : String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CallCalced{
  pub ip          : String
 ,pub target_ip   : String
 ,pub count       : u64
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CallsIpLayer{
    pub calls_ip : Vec<CallIpLayer>
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CallsCalced{
    pub calls : Vec<CallCalced>
}
