use std::{env, fs::File, io::Read};

use reqwest::{blocking::Client, Method, StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct ConfigDataPut {
	data: Vec<u8>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 6 {
        println!("too few args: \n\t1 - config file path\n\t2 - firm\n\t3 - point\n\t4 - program\n\t5 - config");
        return;
    }
    let config_path = args[1].clone();
    let firm = args[2].clone();
    let point = args[3].clone();
    let program = args[4].clone();
    let config = args[5].clone();
    println!("config_path: {}", config_path);
    println!("firm: {}", firm);
    println!("point: {}", point);
    println!("program: {}", program);
    println!("config: {}", config);
    let mut file = File::open(config_path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let req_data = ConfigDataPut {data: buf};
    let req_data_raw = rmp_serde::to_vec(&req_data).unwrap();
    let url = format!("http://srv04.elpi-tech.ru:25000/firms/{firm}/points/{point}/programs/{program}/configs/{config}/data");
    println!("url put: {}", url);
    let resp = Client::new().request(Method::PUT, &url).header("Authorization", "FF00FF00").body(req_data_raw).send().unwrap();
    if resp.status() != StatusCode::OK {
        println!("fail put data: ({:?}) {}", resp.status(), resp.text().unwrap());
        return;
    }
    let url = format!("http://srv04.elpi-tech.ru:25000/firms/{firm}/points/{point}/programs/{program}/configs/{config}/apply");
    println!("url apply: {}", url);
    let resp = Client::new().request(Method::POST, &url).header("Authorization", "FF00FF00").send().unwrap();
    if resp.status() != StatusCode::OK {
        println!("fail apply data: ({:?}) {}", resp.status(), resp.text().unwrap());
        return;
    }
}
