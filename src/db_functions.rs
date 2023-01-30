use rusqlite::Connection;

use crate::types::{CallsIpLayer, CallIpLayer};



pub fn db_create(dbname: &str) -> Result<(),rusqlite::Error>{

    let conn = Connection::open(dbname)?;

    conn.execute(
            "CREATE TABLE IF NOT EXISTS 
                calls (
                id    INTEGER PRIMARY KEY,
                ip    TEXT,
                layer TEXT,
                target_ip TEXT,
                description TEXT
            )",
            (), 
        )?;
    Ok(())
}

pub fn db_truncate_calls_ip(dbname: &str) -> Result<(),rusqlite::Error>{
    let conn = Connection::open(dbname)?;
    conn.execute(
            "DELETE FROM calls",
            (), 
        )?;
    Ok(())
}
    
pub fn db_insert_calls_ip(dbname: &str, cil : CallIpLayer) -> Result<(),rusqlite::Error>{

    let conn = Connection::open(dbname)?;

    conn.execute(
            "INSERT INTO calls
            (   ip,
                layer,
                target_ip,
                description) 
            VALUES
            (?1,?2,?3,?4)",
            (cil.ip,cil.layer,cil.target_ip,cil.description), 
        )?;
    Ok(())
}

pub fn db_get_calls_ip(dbname: &str) -> Result<CallsIpLayer,rusqlite::Error>{

    let conn = Connection::open(dbname)?;
    let mut stmt = conn.prepare("SELECT
        id, ip, layer, target_ip, description
        FROM calls")?;
    
    let call_iter = stmt.query_map([], |row| {
        Ok(CallIpLayer {
            id: row.get(0)?,
            ip: row.get(1)?,
            layer: row.get(2)?,
            target_ip: row.get(3)?,
            description: row.get(4)?,
        })
    })?;
    let mut calls = vec![];
    for call in call_iter{
        let c = call?;
        calls.push(c)
    }
    let calls_ip_layer = CallsIpLayer{
        calls_ip: calls
    };
    Ok(calls_ip_layer)
}

