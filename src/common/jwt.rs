use crate::common::env::ENV;
use crate::users::model::TokenClaims;
use axum::response::Response;
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::TimeDelta;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use time::Duration;
use axum::http::{header};


#[derive(Deserialize, Serialize)]
pub struct JWT {
   pub access_token: String,
   pub refresh_token: String
}

pub struct TokenGenerate {
    user_id: uuid::Uuid,
    time: TimeDelta,
}
pub trait JWTToken {
   fn token_generate(value: TokenGenerate) -> String;
   fn set_cookie(&self, res: Response<String>) -> Response<String>;
}

impl JWTToken for JWT {
        fn set_cookie(&self, mut res: Response<String>) -> Response<String> {
            let access_token = &self.access_token;
            let refresh_token = &self.access_token;
            
            let access = Cookie::build(format!("access={}", access_token.to_owned()))
            .path("/")
            .max_age(Duration::minutes(60))
            .same_site(SameSite::Lax)
            .http_only(true)
            .finish();  

            let refresh = Cookie::build(format!("refresh={}", refresh_token.to_owned()))
            .path("/")
            .max_age(Duration::minutes(60))
            .same_site(SameSite::Lax)
            .http_only(true)
            .finish();       

            res.headers_mut().append(header::SET_COOKIE, access.to_string().parse().unwrap());
            res.headers_mut().append(header::SET_COOKIE, refresh.to_string().parse().unwrap());

            return res
        }

        fn token_generate(params: TokenGenerate) -> String {
        let now = chrono::Utc::now();
    
        let token = encode(
            &Header::default(),
            &TokenClaims {
                sub: params.user_id.to_string(),
                exp: (now + params.time).timestamp() as usize,
                iat: now.timestamp() as usize,
            },
            &EncodingKey::from_secret(ENV::new().JWT_SECRET.as_ref()),
        )
        .unwrap();

        

        token

    }
}


impl JWT {

  pub  fn new(user_id: uuid::Uuid) -> Self {
        let access_token = <JWT as self::JWTToken>::token_generate( TokenGenerate {
            user_id,
            time: chrono::Duration::minutes(ENV::new().JWT_ACCESS_EXPIRED_IN),
        });

        let refresh_token = <JWT as self::JWTToken>::token_generate(TokenGenerate {
            user_id,
            time: chrono::Duration::minutes(ENV::new().JWT_ACCESS_EXPIRED_IN),
        });

        JWT {
            access_token,
            refresh_token,
        }
    }
}