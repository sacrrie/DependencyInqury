//maybe the short demo can use LINE API to send/reply messages.
extern crate postgres;
extern crate udp_messenging;
use std::net;
use std::str;
use postgres::{Connection,TlsMode};
fn main() {
    let dsn = "postgresql://rust:rust@localhost/rust";
    //set up local listening port for inquiry messages
    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let listen_addr = net::SocketAddrV4::new(ip, 8888);
    let send_addr = net::SocketAddrV4::new(ip, 8889);
    let conn = match Connection::connect(dsn,postgres::TlsMode::None){
        Ok(conn)=> conn,
        Err(e)=> {
            println!("Failed to connect! error :{}\n",e);
            return;
        }
    };
    println!("Connection to DB is established. Dependencies inquiring service is on.\n");
    loop {
        let capture = udp_messenging::listen(net::SocketAddr::V4(listen_addr));
        let received = capture.join().unwrap();
        //u8数列信息，第一个数位标识模式，后面padding上字符串转码
        //u8 array as payload, [0] indicates inqury mode, the rest is inqury package name
        println!("{:?}", received);
        
        let p_name= str::from_utf8(&received[1..]).unwrap();
        println!("{:?}", p_name);
        match received[0]{
            1 => find_dependencies(p_name,listen_addr,send_addr,&conn),
            2 => find_whos_depending_on(p_name,listen_addr,send_addr,&conn),
            _ => panic!("Illegal msg[0]")
        };
    };
}

fn find_dependencies(name: &str, source: net::SocketAddrV4,destn: net::SocketAddrV4, conn: &postgres::Connection){
    let c1="SELECT * FROM dependencies WHERE package_name = '".to_string();
    let c3="'".to_string();
    let c=c1+&name + &c3;
    //let c=c[..];
    let mut k: Vec<String> = Vec::new();
    println!("{}", c);
    for row in &conn.query(&c,&[]).unwrap(){
        k.push(row.get(2));
    }
    let k = k.join(",");
    println!("{:?}", k);
    //send out the records
    let message=k.into_bytes();
    udp_messenging::send_message(net::SocketAddr::V4(source),net::SocketAddr::V4(destn),message);
}

fn find_whos_depending_on(name: &str,source: net::SocketAddrV4,destn: net::SocketAddrV4,  conn: &postgres::Connection){
    
    let c1="SELECT * FROM dependencies WHERE depend_on = '".to_string();
    let c3="'".to_string();
    let c=c1+&name + &c3;
    //let c=c[..];
    let mut k: Vec<String> = Vec::new();
    for row in &conn.query(&c,&[]).unwrap(){
        k.push(row.get(2));
    }
    let k = k.join(",");
    println!("{:?}", k);
    //send out the records
    let message=k.into_bytes();
    udp_messenging::send_message(net::SocketAddr::V4(source),net::SocketAddr::V4(destn),message);
}