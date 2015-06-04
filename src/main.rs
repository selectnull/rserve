extern crate iron;
extern crate hoedown;
#[macro_use] extern crate mime;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;

use hoedown::Markdown;
use hoedown::renderer::html::Html;
use hoedown::renderer::html;
use hoedown::renderer::Render;
use hoedown::Buffer;


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

fn main() {
    let address = "localhost:8000";
    let cd = env::current_dir().unwrap();
    println!("Listening on {} from {}", address, cd.display());

    Iron::new(|request: &mut Request| {
        Ok(response(request))
    }).http(address).unwrap();
}
