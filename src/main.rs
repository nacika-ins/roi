extern crate env_logger;
extern crate futures;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;

use std::io;

use futures::future;
use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;
use tokio_core::reactor::Core;
extern crate url;
use std::collections::HashMap;

extern crate tokio_io;

use futures::{stream, Future, Stream, Sink};

use tokio_io::AsyncRead;
use std::io::prelude::*;
use std::net::TcpStream;

struct HelloWorld;
use std::{thread, time};
extern crate time as time2;

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, _request: Request) -> Self::Future {
        let mut resp = Response::new();
        resp.body("");

        let url2 = url::Url::parse(&format!("http://dummy.com{}", _request.path())).unwrap();
        let hash_query: HashMap<_, _> = url2.query_pairs().into_owned().collect();
        let roi = hash_query.get("roi").unwrap_or(&"".to_owned()).to_string();
        let k = hash_query.get("k").unwrap_or(&"".to_owned()).to_string();
        let v = hash_query.get("v").unwrap_or(&"".to_owned()).to_string();

        // roi追加モード
        match roi {
          _ if roi == "" => {
          }
          _ => {
            thread::spawn(move || {
                let now_unixtime: i64 = time2::now().to_timespec().sec;
                let mut stream = TcpStream::connect("127.0.0.1:8203").unwrap();
                let f = format!("fx.roi {} {}\n", roi, now_unixtime);
                let _ = stream.write_all(&f.as_bytes());
                print!("{}", f);
            });
          }
        }
        
        // kv追加モード
        if k != "" && v != "" {
            thread::spawn(move || {
                let now_unixtime: i64 = time2::now().to_timespec().sec;
                let mut stream = TcpStream::connect("127.0.0.1:8203").unwrap();
                let f = format!("{} {} {}\n", k, v, now_unixtime);
                let _ = stream.write_all(&f.as_bytes());
                print!("{}", f);
            });
        }

        future::ok(resp)
    }
}

fn main() {
    drop(env_logger::init());
    let addr = "0.0.0.0:8444".parse().unwrap();
    TcpServer::new(Http, addr)
        .serve(|| Ok(HelloWorld));
}
