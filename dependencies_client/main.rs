extern crate udp_messenging;
use std::io;
use std::net;
use std::str;
fn main() {
    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let send_addr = net::SocketAddrV4::new(ip, 8888);
    let listen_addr = net::SocketAddrV4::new(ip, 8889);
    loop {
        println!("Type in the question number please:\n 
        2 Find out which packages are depending on your inqury package.
        1 Find out which packages your inqury package is depending on.");
        let mut modeNumber= String::new();
        io::stdin().read_line(&mut modeNumber)
            .expect("failed to read number");
        let modeNumber: u8 = modeNumber.trim().parse()
            .expect("Please type a number!");
        let mut msg= vec![modeNumber];
        println!("Type in the package name you wish to inqury:\n");
        let mut packageName=String::new();
        io::stdin().read_line(&mut packageName)
            .expect("failed to read package name");
        let mut packageName= packageName.trim().to_string();
        let mut codedName = packageName.into_bytes();
        msg.append(&mut codedName);
        println!("{:?}", msg);
        udp_messenging::send_message(net::SocketAddr::V4(listen_addr),net::SocketAddr::V4(send_addr),msg);
        //look for reply
        //thread.sleep(200);
        let capture = udp_messenging::listen(net::SocketAddr::V4(listen_addr));
        let received = capture.join().unwrap();
        let answer=str::from_utf8(&received[..]).unwrap();
        println!("As required, the related packages are as follows:\n {}",answer);
    }
}
