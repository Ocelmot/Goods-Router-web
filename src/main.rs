use dotenv::dotenv;

use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_files as fs;

mod auth;

mod routes;

use r2d2::Pool;
use db_connector::pool::ConnectionManager;

async fn api_error() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("No such API path!"))
}

async fn spa() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::var("HMAC_SECRET").expect("HMAC_SECRET not set!");
    std::env::var("AUTH_TIMEOUT").expect("AUTH_TIMEOUT not set!");

    let manager = ConnectionManager::new("mongodb://localhost:27017");
    let pool = Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(
                web::scope("/api")
                    .route("/json", web::get().to(routes::auth::json))
                    .route("/login", web::get().to(routes::auth::login))
                    .route("/echo_token", web::get().to(routes::auth::echo_token))
                    .default_service(web::route().to(api_error))
            )
            .service(fs::Files::new("/static", "./static"))
            .default_service(web::route().to(spa))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}



#[derive(Debug)]
pub struct Error {msg:String, source: Option<Box<dyn std::error::Error>>}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match &self.source{
			Some(err) => {
				write!(f, "Error {}\nFrom {}", self.msg, &*err)
			},
			None => {
				write!(f, "Error {}", self.msg)
			}
		}
    }
}

impl std::error::Error for Error{
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
    }
}

impl From<String> for Error{
	fn from(error: String) -> Self{
		Error{msg: error.into(), source: None}
	}
}

impl From<&str> for Error{
	fn from(error: &str) -> Self{
		Error{msg: error.into(), source: None}
	}
}

impl From<Box<dyn std::error::Error>> for Error{
    fn from(error: Box<dyn std::error::Error>) -> Self{
        Error{msg: format!("{:?}", error), source: None}
    }
}

// std::convert::From<base64::decode::DecodeError>
impl From<base64::DecodeError> for Error{
    fn from(error: base64::DecodeError) -> Self{
		Error{msg: "Base 64 decode error".into(), source: None}
	}
}

//std::str::Utf8Error
impl From<std::str::Utf8Error> for Error{
    fn from(error: std::str::Utf8Error) -> Self{
		Error{msg: "UTF8 decode error".into(), source: None}
	}
}

use hmac::crypto_mac::MacError;
//crypto_mac::errors::MacError
impl From<MacError> for Error{
    fn from(error: MacError) -> Self{
		Error{msg: "Mac verification error".into(), source: None}
	}
}

// serde_json::error::Error
impl From<serde_json::error::Error> for Error{
    fn from(error: serde_json::error::Error) -> Self{
        Error{msg: "Deserialization error".into(), source: None}
    }
}