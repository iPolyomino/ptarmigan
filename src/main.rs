extern crate hyper;
extern crate hyper_tls;

use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;

fn main() {
    let uri = "https://polyomino.jp".parse().unwrap();

    rt::run(rt::lazy(|| {
        // 4 is number of blocking DNS threads
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, hyper::Body>(https);

        client
            .get(uri)
            .and_then(|res| {
                println!("status: {}", res.status());
                res.into_body().concat2()
            })
            .map(|body| println!("body: {}", String::from_utf8(body.to_vec()).unwrap()))
            .map_err(|e| println!("error: {}", e))
    }));
}
