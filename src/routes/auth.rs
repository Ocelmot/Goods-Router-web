use db_connector::GRConnection;
use db_connector::ObjectId;



use r2d2::Pool;
use db_connector::pool::ConnectionManager;
use crate::auth;

use actix_web::{HttpRequest, HttpResponse, HttpMessage, Result, web, http};

#[derive(serde::Deserialize)]
pub struct UserParams{
    username:String,
    password:String
}

pub async fn login(req: HttpRequest, params: web::Query<UserParams>, db: web::Data<Pool<ConnectionManager>>) -> Result<HttpResponse> {
    let connector = db.get().unwrap();
    let user = connector.get_user_by_name(params.username.clone());

    match user {
        Ok(user) => {

            // make sure user matches!

            //set user cookie
            let mut auth_data = auth::AuthData::new();
            auth_data.data.insert("username".into(), params.username.clone());
            let cookie = auth::generate_cookie(auth_data);
            
            let mut response = HttpResponse::build(http::StatusCode::OK);
            response.cookie(cookie);

            // return user
            Ok(response.json(user))// Make sure that password is not returned!
        },
        Err(_e) => {
            Ok(HttpResponse::Ok().json("Error fetching user"))
        }
    }    
}

pub async fn echo_token(req: HttpRequest, db: web::Data<Pool<ConnectionManager>>) -> Result<HttpResponse> {
    let auth_data = auth::authenticate(&req, Some(vec!["test"]));
    
    
    match auth_data {
        Ok(data) => {
            Ok(HttpResponse::Ok().json(data))
        },
        Err(e) => {
            Ok(HttpResponse::Ok().json(e.msg))
        }
    }
}

pub async fn json(req: HttpRequest, db: web::Data<Pool<ConnectionManager>>) -> Result<HttpResponse> {
    // let auth_data = auth::authenticate(&req, Some(vec!["test"]));
    
    let connector = db.get().unwrap();
    
    let id = ObjectId::with_string("5ec45328009f5cc500f8a55a").unwrap();
    let loc = connector.get_location(id);
    match loc {
        Ok(loc) => {
            Ok(HttpResponse::Ok().json(loc))
        },
        Err(_e) => {
            Ok(HttpResponse::Ok().json("Error fetching location"))
        }
    }
}