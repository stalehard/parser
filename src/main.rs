#![feature(io)]

extern crate hyper;
extern crate mysql;

use std::default::Default;
use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::{from_value, from_row};



struct News {
    id: String,
    title: String
}

fn main() {
    let opts = MyOpts {
        user: Some("root".to_string()),
        db_name: Some("news18".to_string()),
//        tcp_addr: Some("127.0.0.1".to_string()),
        ..Default::default()
    };

    let pool = MyPool::new(opts)
        .ok()
        .expect("Connect to db failed");


//    for row in pool.prep_exec("select id from news limit 1", ()).unwrap() {
//
//    }

//    let collect_news: Vec<News> = pool.prep_exec("SELECT id, title from news limit 5", ()).map(|result| {
//        result.map(|row| {
//            let (id, title) = from_row(row.unwrap());
//            News {
//                id: id,
//                title: title,
//            }
//        }).collect()
//    }).unwrap();

//    println!("{}", collect_news[1].title);


    // Create a client.
    let client = Client::new();

    // Creating an outgoing request.
    let mut res = client.get("http://myudm.ru/news/archive/6").send()
        .ok()
        .expect("Failed to get data");

    // Read the Response.
//    let mut buf: Vec<u8> = Vec::new();
    let mut s = String::new();

//    match res.read_to_string(&mut s) {
//        Ok(r) => println!("{}", s),
//        Err(e) => println!("Error, {}", e)
//    };

    let ch = res.chars();
    for c in ch {
        println!("{:?}", c);
    }



//    loop {
//        let c = res.
//    }
//
//    let mut x = 0;
//    for c in res.chars() {
//        println!("{}", x);
//        x += 1;
//    }





//    let s = String::from_utf8(buf).err().unwrap();
//    let err = s.utf8_error();

//    let s = match String::from_utf8(buf) {
//        Ok(s) => s,
//        Err(err) =>  {
//            println!("{}", err);
//            return;
//        }
//    };



//    let s = String::from_utf8(buf)
//        .ok()
//        .expect("Failed to get data");


//    println!("{:?}", size);



//    println!("{}", body);


    //println!("Response: {:?}", rss.);
}



