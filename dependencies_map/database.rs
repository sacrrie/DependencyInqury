extern crate postgres;
use postgres::{Connection,TlsMode};
pub fn dbconn_initial(dblink: &str) -> T {// establish and check table
    let dsn = dblink;//right now it's: "postgresql://rust:rust@localhost/rust"
    let conn = match Connection::connect(dsn,postgres::TlsMode::None){
        Ok(conn)=> conn,
        Err(e)=> {
            println!("Failed to connect! error :{}\n",e);
            return;
        }
    conn.execute("create table if not exists dependencies (
        record_id serial PRIMARY KEY,
        package_name varchar (50) NOT NULL ,
        depend_on varchar (25) NOT NULL)", &[])
        .ok().expect("Table creation failed");
    //****TODO: incoporate this dependencies into above execution.
    conn.execute("ALTER TABLE dependencies
        ADD CONSTRAINT unique_records
        UNIQUE (package_name,depend_on)", &[]).ok().expect("Failed at enforce unique constraints on colunms");
    return conn;
    };
    //TODO: insert , update, delete, or just using SQL in the main.rs