//TODO: program should check if there's already a repo list cache (update cycle: every day; or no need to check if there's no change in the repo dependencies)
//TODO: string manipulation: indexed by all the repos use Rust, request their Cargo.toml contents, decode and stored as a repo dependencies data.
extern crate rustc_serialize;
extern crate reqwest;
extern crate base64;
extern crate postgres;
use postgres::{Connection,TlsMode};
use rustc_serialize::json::{self,Encoder};
use rustc_serialize::json::Json;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use base64::{encode, decode};
mod http_processor;
//mod database;
fn main() {
    let k="https://api.github.com/orgs/DEMO_inc/repos";
    let dsn = "postgresql://rust:rust@localhost/rust";
    database_init(k,dsn);
    //let mut repos_list =File::create("repos_list.txt").expect("failed create file");
    //write!(&mut repos_list,"{}",raw_info).expect("failed to write ");
    }


fn database_init(k: &str,dsn: &str){
    
    let conn = match Connection::connect(dsn,postgres::TlsMode::None){
        Ok(conn)=> conn,
        Err(e)=> {
            println!("Failed to connect! error :{}\n",e);
            return;
        }
        };
    //initialize repo list that uses Rust
    let raw_info: String = http_processor::url_content(k);
    let s: String = raw_info.to_owned();
    let  content: &str = &s[..];
    let test_string = json::Json::from_str(content).expect("failed to parse as string");
    let test_string= test_string.as_array().expect("failed to turn to array");
    //this iteration can be turned
    for e in test_string.iter()
    {
        let t = e.find_path(&["language"]).expect("fail language");
        if !t.is_null() {
            let t = t.as_string().expect("failed to convert to string");
            if  t == "Rust" {
                //get every Rust repo's Cargo.toml contents
                let repos = e.find_path(&["name"]).unwrap();
                let repos = repos.as_string().unwrap();
                let k1 : String= "https://api.github.com/repos/DEMO_inc/".to_string();
                let mut r1 = k1 + &repos;
                r1.push_str("/contents/Cargo.toml");
                let  r: &str = &r1[..];
                let repo_cargo : String = http_processor::url_content(r);
                let repo_cargo: &str = &repo_cargo[..];
                let repo_cargo = json::Json::from_str(repo_cargo).expect("failed to parse cargo html");
                let repo_cargo = repo_cargo.find_path(&["content"]).unwrap();
                let repo_cargo = repo_cargo.as_string().unwrap();
                let  repo: &str = &repo_cargo[..];
                let configs = base64::Config::new(base64::CharacterSet::Standard,false,true,base64::LineWrap::NoWrap);
                let decoded_cargo = base64::decode_config(repo, configs).expect("failed to decode base64");
                let string_cargo = std::str::from_utf8(&decoded_cargo).expect("failed to transform utf8 to string");
                let cargo = String::from(string_cargo);
                toml_parse(&cargo,&repos, &conn);
                //println!("{}", string_cargo);
                //write!(&mut dependencies,"{}",string_cargo).expect("failed to record cargo info");
                //write!(&mut dependencies,"---\n");
            };
        };
    };
}

fn toml_parse (page: &String,name: &str,database: &postgres::Connection) {//reminder: ownership is taken and returns a vector,or should it be an iteratable?
    let mut lever = false;
    //println!("function called\n");
    for line in page.lines() {
        let l = line;
        if l=="[dependencies]" {
            lever = true ;
            //println!("lever was pulled\n");
            continue;
        };
        if lever {
            let depends = l.split(" ");
            let dp: Vec<&str> = depends.collect();
            if (dp.len()>4)&&(dp[2]=="{"){
            //insert into database
            println!("{}",dp[0]);
            let c1: String ="INSERT INTO dependencies (package_name,depend_on) VALUES('".to_string();
            let c3: String ="','".to_string();
            let c5: String ="')".to_string();
            let mut ct=c1+&name+&c3+&dp[0]+&c5;
            let c: &str = &ct[..];
            database.execute(c,&[]).ok().expect("failed to insert record");
            };
        };
    };
}