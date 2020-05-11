extern crate reqwest;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use self::reqwest::header::{Authorization, Basic};
pub fn url_content(x: &str) -> String {
    println!("Get {}",x);
    let link=x;
    let client = reqwest::Client::new();
    let credentials=Basic {
        //crude way of imbedding credentials in the source code
        //but it's a short demo after all
        username:"DEMO_account".to_string(),
        password:Some("DEMO_pswd".to_string()),
        };
    let mut res2=client.get(link).header(Authorization(credentials)).send().expect("failed to get res2!");
    println!("{:?}",res2.status());
//    let mut repos_list =File::create("raw_data.txt").expect("failed create file");
//    res2.copy_to(&mut repos_list).expect("failed to get raw data");
    let mut resbody = String::new();
    res2.read_to_string(&mut resbody).expect("failed to read in string");
    
    resbody
}