extern crate iron;
extern crate hoedown;
#[macro_use] extern crate mime;
extern crate getopts;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::net::{SocketAddrV4, Ipv4Addr};

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;

use hoedown::Markdown;
use hoedown::renderer::html::Html;
use hoedown::renderer::html;
use hoedown::renderer::Render;
use hoedown::Buffer;

use getopts::Options;


fn get_filename(path: &Vec<String>) -> String {
    path.connect("/")
}

fn get_mimetype(filename: String) -> Mime {
    let p = Path::new(&filename).extension().unwrap();

    let plain_text = mime!(Text/Plain);
    let html_text = mime!(Text/Html; Charset=Utf8);

    if p.eq("md") {
        html_text
    } else if p.eq("html") {
        html_text
    } else if p.eq("txt") {
        plain_text
    } else if p.eq("ico") {
        "image/x-icon".parse::<Mime>().unwrap()
    } else {
        "application/octet-stream".parse::<Mime>().unwrap()
    }
}

fn get_content(filename: String) -> Buffer {
    let mut file = File::open(filename).unwrap();
    let mut md = String::new();
    file.read_to_string(&mut md).unwrap();
    let doc = Markdown::new(&mut md);
    let mut html = Html::new(html::Flags::empty(), 0);
    html.render(&doc)
}

fn response(request: &mut Request) -> Response {
    println!("{}", request.url);

    let mimetype = get_mimetype(get_filename(&request.url.path));
    let content = get_content(get_filename(&request.url.path));

    Response::with((
        status::Ok, mimetype,
        content.to_str().unwrap()
    ))
}

fn get_address(port: u16) -> SocketAddrV4 {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    SocketAddrV4::new(ip, port)
}

fn main() {
    // define command line arguments
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("p", "port", "Set port", "port");
    opts.optflag("h", "help", "Show help");

    // parse command line arguments
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string())
    };

    if matches.opt_present("h") {
        let u = format!("Usage: {} [options]", &args[0]);
        print!("{}", opts.usage(&u));
        return
    }

    let port: u16 = match matches.opt_default("p", "8000") {
        Some(p) => p.parse().unwrap(),
        None => 8000,
    };

    // run program
    let address = get_address(port);
    let cd = env::current_dir().unwrap();
    println!("Listening on {} from {}", address, cd.display());

    Iron::new(|request: &mut Request| {
        Ok(response(request))
    }).http(address).unwrap();
}
