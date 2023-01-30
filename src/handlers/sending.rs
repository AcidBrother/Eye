use crate::conf::NodeConfig;
use crate::db_functions::db_get_calls_ip;
use crate::types::*;

use rocket::{get, State};

use rocket::http::{ContentType, Status};
use rocket_okapi::openapi;



#[openapi(tag = "send")]
#[get("/calls/ip")]
pub async fn get_calls_ip_layer(cfg: &State<NodeConfig>) -> (Status, (ContentType, String)) {
  let dbname = &cfg.clone().dbname;
  let calls = db_get_calls_ip(dbname);
  let r_send_tx:Result<CallsIpLayer, rusqlite::Error> = calls;
  match r_send_tx{
      Err(e) => {
          log::warn!("NodeErr Response {:?}",&e);
          return (Status::InternalServerError,(ContentType::JSON, (&e).to_string()));
      },
      Ok(send_tx) => {
          let json_res = serde_json::to_string(&send_tx);
          return (Status::Ok, (ContentType::JSON, json_res.unwrap()));
      }
  }
}


#[openapi(tag = "send")]
#[get("/calls/ip/calced")]
pub async fn get_calls_calced(cfg: &State<NodeConfig>) -> (Status, (ContentType, String)) {
  let dbname = &cfg.clone().dbname;
  let rcalls = db_get_calls_ip(dbname);
  let c_calls = match rcalls{
      Ok(calls) => {
        let mut calced:Vec<CallCalced> = vec![];
        for call in calls.calls_ip{
            let mut found: bool = false;
            for mut c_calced in & mut calced
            {
                if (c_calced.ip == call.ip) && (c_calced.target_ip == call.target_ip){
                    c_calced.count = c_calced.count + 1;
                    found = true;
                }
            }
            if found == false   {
                let c_c = CallCalced{
                    ip: call.ip,
                    target_ip: call.target_ip,
                    count: 1
                };
                calced.push(c_c);
            }
        }
        let c_calced = CallsCalced{
            calls: calced
        };
        c_calced
      }
      Err(e) => {
          log::warn!("NodeErr Response {:?}",&e);
          return (Status::InternalServerError,(ContentType::JSON, (&e).to_string()));
      }
  };
  let r_send_tx:Result<CallsCalced, rusqlite::Error> = Ok(c_calls);
  match r_send_tx{
      Err(e) => {
          log::warn!("NodeErr Response {:?}",&e);
          return (Status::InternalServerError,(ContentType::JSON, (&e).to_string()));
      },
      Ok(send_tx) => {
          let json_res = serde_json::to_string(&send_tx);
          return (Status::Ok, (ContentType::JSON, json_res.unwrap()));
      }
  }
}
