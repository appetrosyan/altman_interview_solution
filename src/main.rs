use argparse::{ArgumentParser, Store};
use std::io::Read;
use std::error::Error;
use std::time::Duration;

fn ping(interval: u64, url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::blocking::get(url)?;
    while(true){
        match res.status() {
            reqwest::StatusCode::OK => println!("Ok(200)"),
            _ => println!("{}", res.status())
        }
        std::thread::sleep(Duration::new(interval, 0));
    }

    Ok(())
}

fn parse() -> (u64, String) {
    let mut interval : u64 = 1;
    let mut url = String::new();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut interval)
            .add_argument("interval", Store, "ping interval");
        ap.refer(&mut url)
            .add_argument("url", Store, "ping url");
        ap.parse_args_or_exit();
    }
    (interval, url)
}

fn main() -> Result<(), Box<dyn Error>>{
    let (interval, url) = parse();
    let url_parsed = reqwest::Url::parse(url.as_str());
    match url_parsed {
        Ok(url) => {
            let ping_result = ping(interval, url.to_string());
        }
        Err(e) => {
            println!("URL parse error")
        }
    }
    Ok(())
}