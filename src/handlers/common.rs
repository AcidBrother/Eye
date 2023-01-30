use crate::conf::NodeConfig;

use rocket::{get,State};
use rocket_okapi::openapi;

#[openapi(tag = "version")]
#[get("/version")]
pub fn getversion(cfg: &State<NodeConfig>) -> String {
    let res = "0.0.1"; 
    return format!("Version is {:?}",res);
}
