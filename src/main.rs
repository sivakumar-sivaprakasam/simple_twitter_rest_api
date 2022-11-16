#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate diesel;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};
use actix_web::web::Data;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

mod constants;
mod schema;
mod tweet;
mod like;
mod response;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,r2d2=debug,diesel=debug");
	env_logger::init();
	
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
	println!("DB_URL: {}", database_url);
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	println!("Connection manager initialized successfully!!!");
	let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");
	println!("Pool created successfully!!!");
	
	HttpServer::new(move || {
		App::new()
			.app_data(Data::new(pool.clone()))
			.wrap(middleware::Logger::default())
			.service(tweet::list)
			.service(tweet::get)
			.service(tweet::create)
			.service(tweet::delete)
			.service(like::list)
			.service(like::plus_one)
			.service(like::minus_one)
	}).bind("0.0.0.0:9090")?.run().await
}
