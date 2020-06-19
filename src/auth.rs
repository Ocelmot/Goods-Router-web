
use crate as Web;
use actix_web::{HttpRequest, HttpMessage, Result, cookie::Cookie};

use base64::{decode_config};

use hex;

use sha2::Sha256;
use hmac::{Hmac, Mac};

type Hash = Hmac<Sha256>;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AuthData{
	date_created: DateTime<Utc>,
	permissions: Vec<String>,
	pub data: HashMap<String, String>
}

impl AuthData{
	pub fn new() ->Self{
		Self{
			date_created: Utc::now(),
			permissions: vec![],
			data: HashMap::new(),
		}
	}

}


pub fn authenticate<S: Into<String>>(req: &HttpRequest, required_roles: Option<Vec<S>>) -> Result<AuthData, Web::Error>{
	// get token from cookie
	let token = req.cookie("auth_data");
	let token = match token{
		Some(token) => {token},
		None =>{
			if required_roles.is_none(){
				return Ok(AuthData::new());
			}else{
				return Err("Authentication Error".into());
			}
		}
	};
	// split by :
	let parts: Vec<&str>= token.value().split(":").collect();
	if parts.len() != 2{
		return Err("Malformed token! wrong number of segments".into())
	}
	
	// hash first part
	let secret = std::env::var("HMAC_SECRET").unwrap();
	let mut mac = Hash::new_varkey(secret.as_bytes()).expect("invalid hash input?");
	let h = hex::decode(parts[1]).ok().ok_or("Malformed token")?;
	// decode first part base 64
	let decoded = decode_config(parts[0], base64::STANDARD)?;
	mac.input(&decoded);


	mac.verify(&h)?;
		
	// decode first part into AuthData
	let auth_data: AuthData = serde_json::from_slice(&decoded)?;
	// validate expiration
	// if auth_data.date_created < 

	// validate required roles

	// if everything is ok, return Ok(AuthData)
	Ok(auth_data)
}

pub fn generate_cookie<'c>(auth_data: AuthData)-> Cookie<'c>{
	// serialize auth_data
	let serialization = serde_json::to_string(&auth_data).expect("Failure to convert auth_data to string"); 
	// calculate hash of auth_data
	
	let secret = std::env::var("HMAC_SECRET").unwrap();
	let mut mac = Hash::new_varkey(secret.as_bytes()).expect("invalid hash input?");
	mac.input(serialization.as_bytes());
	let hash = mac.result().code();
	let hash = hash.to_vec();
	let hash = hex::encode(hash);
	
	let serialization = base64::encode_config(serialization, base64::STANDARD);
	// token = serialization + ':' + hash
	let token = serialization + ":" + &hash;
	// base64 encode token
	// let encoded_token = base64::encode_config(token, base64::STANDARD);

	let cookie = Cookie::build("auth_data", token);
	// cookie.expires();
	cookie.finish()
}