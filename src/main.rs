#![feature(proc_macro_hygiene, decl_macro)]

mod application;
mod domain;
mod infrastructure;
mod utils;

extern crate futures;
#[macro_use]
extern crate rocket;
extern crate elasticsearch;
extern crate serde_json;

use application::repositories::SubjectRepository;
use elasticsearch::http::transport::Transport;
use elasticsearch::Elasticsearch;
use infrastructure::gateways::SubjectGateway;

#[get("/")]
fn test() -> &'static str {
    "Test"
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // let mut config = rocket::config::Config::new(Environment::Development);
    // config.set_port(3000);
    // let app = rocket::custom(config);
    // app.mount("/", routes![test]).launch();
    let transport = Transport::single_node("http://10.0.0.2:9200")?;
    let es = Elasticsearch::new(transport);
    let gw = SubjectGateway::new(&es, "kittimetable_subjects");

    println!("{:?}", gw.get_by_id(300).await?);

    Ok(())
}
