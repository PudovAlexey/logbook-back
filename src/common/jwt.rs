use crate::common::env::ENV;
use crate::users::model::TokenClaims;
use axum::response::Response;
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use time::Duration;
use axum::http::{header};
use utoipa::ToSchema;

enum Token {
    Access,
    Refresh,
}

impl Token {
    fn new(val: Token) -> String {
        match val {
            Token::Access => ENV::new().jwt_access_secret,
            Token::Refresh => ENV::new().jwt_refresh_secret,
        }        
    }
}


#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct JWT {
   pub access_token: String,
   pub refresh_token: String,
   pub access_expired_in: usize,
}

pub struct TokenGenerate {
    user_id: uuid::Uuid,
    time: i64,
    token_type: String,
}

pub struct TokenGeneration {
   pub token: String,
   pub expires_in: usize,
}
pub trait JWTToken {
    fn set_cookie(&self, res: Response<String>) -> Response<String>;
    fn token_generate(value: TokenGenerate) -> TokenGeneration;
}

impl JWTToken for JWT {
        fn set_cookie(&self, mut res: Response<String>) -> Response<String> {
            let access_token = &self.access_token;
            let refresh_token = &self.refresh_token;

            
            let access = Cookie::build(format!("access={}", access_token.to_owned()))
            .path("/")
            .max_age(Duration::minutes(ENV::new().jwt_access_expired_in))
            .same_site(SameSite::Lax)
            .http_only(true);
            // .finish();  

            let refresh = Cookie::build(format!("refresh={}", refresh_token.to_owned()))
            .path("/")
            .max_age(Duration::minutes(ENV::new().jwt_refresh_expired_in))
            .same_site(SameSite::Lax)
            .http_only(true);
            // .finish();       

            res.headers_mut().append(header::SET_COOKIE, access.to_string().parse().unwrap());
            res.headers_mut().append(header::SET_COOKIE, refresh.to_string().parse().unwrap());

            return res
        }

         fn token_generate(params: TokenGenerate) -> TokenGeneration {
        let now = chrono::Utc::now();
        let expire_secs = params.time * 60;
        let time = (now.timestamp() + expire_secs) as usize;

        let token = encode(
            &Header::default(),
            &TokenClaims {
                sub: params.user_id.to_string(),
                exp: time,
                iat: now.timestamp() as usize,
            },
            &EncodingKey::from_secret(params.token_type.as_ref()),
        )
        .unwrap();

        

        TokenGeneration {
            token,
            expires_in: time
        }

    }
}


impl JWT {

  pub  fn new(user_id: uuid::Uuid) -> Self {
        let access_token = <JWT as self::JWTToken>::token_generate( TokenGenerate {
            user_id,
            time: ENV::new().jwt_access_expired_in,
            token_type: Token::new(Token::Access)
        });

        let refresh_token = <JWT as self::JWTToken>::token_generate(TokenGenerate {
            user_id,
            time: ENV::new().jwt_refresh_expired_in,
            token_type: Token::new(Token::Refresh)
        });

        JWT {
            access_token: access_token.token,
            refresh_token: refresh_token.token,
            access_expired_in: access_token.expires_in,
        }
    }
}

pub fn is_valid_token(refresh_token: &str) -> bool {
    let decoding_key = DecodingKey::from_secret(ENV::new().jwt_refresh_secret.as_ref()); // Замените на ваш секретный ключ
    let validation = Validation::default();

    match decode::<TokenClaims>(&refresh_token, &decoding_key, &validation) {
        Ok(_token_data) => {
            true
        },
        Err(_err) => {
            false
        }
    }
}

pub fn remove_jwt_cookie(mut res: Response<String>) -> Response<String> {
    let tokens = ["access", "refresh"];

    for name in tokens.iter() {
        let cookie = Cookie::build(name.to_string())
        .path("/")
        .max_age(Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);
        // .finish();

        res
            .headers_mut()
            .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
        
    }

    res

}