#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate rayon;
extern crate regex;
extern crate reqwest;
extern crate serde_json;

use rayon::prelude::*;
use regex::Regex;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct Comment {
    by: String,
    text: String,
    id: u32,
}

fn main() {
    println!("Hello, world!");

    // Todo: keep in some kind of hashmap? Persistent across runs?
    if let Ok(ids) = get_comment_ids() {
        let comz = ids.par_iter()
            .map(|&x| get_comment(x).unwrap())
            .collect::<Vec<Comment>>();
        for c in comz {
            println!("{:?}", c);
        }
    }
}

// TODO: de-unwrap this code
// TODO: Parse the DOM instead of regex? html5ever?
fn get_comment_ids() -> Result<Vec<u32>, ()> {
    // TODO: move this regex to lazy static... or remove it
    let re = Regex::new(r"<tr class='athing' id='(\d*)'>").unwrap();

    if let Ok(mut resp) = reqwest::get("https://news.ycombinator.com/newcomments") {
        let mut body = String::new();
        if let Ok(_) = resp.read_to_string(&mut body) {
            return Ok(re.captures_iter(&body)
                .map(|x| x.get(1).unwrap().as_str().parse::<u32>().unwrap())
                .collect::<Vec<u32>>());
        }
    }

    Err(())
}

fn get_comment(id: u32) -> Result<Comment, ()> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    if let Ok(mut resp) = reqwest::get(&url) {
        if let Ok(body) = resp.json::<Comment>() {
            return Ok(body);
        }
    }
    Err(())
}
