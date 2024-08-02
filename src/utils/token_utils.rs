use actix_web::{web};
use actix_web::cookie::Cookie;
use jsonwebtoken::{DecodingKey, TokenData, Validation};

use crate::{config::db::Pool, constants, models::{
    user::{User, LoginInfoDTO},
    user_token::{UserToken, KEY},
}};

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub fn verify_token(
    token_data: &TokenData<UserToken>,
    pool: &web::Data<Pool>,
) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, &mut pool.get().unwrap()) {
        Ok(token_data.claims.user.clone().to_string())
    } else {
        Err(constants::MESSAGE_INVALID_TOKEN.to_string())
    }
}

pub fn get_data_token_to_login_info(token: Cookie) -> Result<LoginInfoDTO, String> {
    if  let Ok(token_data) = decode_token(token.value().to_string()) {
        let claims = token_data.claims;
        let login_info_dto = LoginInfoDTO {
            username: claims.user.to_string(),
            login_session: claims.login_session.to_string(),
            role: claims.role.to_string(),
            company: Option::from(claims.company.to_string()),
        };
        Ok(login_info_dto)
    } else {
        Err(constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string())
    }
}

/*pub fn is_auth_header_valid(authen_header: &HeaderValue) -> bool {
    if let Ok(authen_str) = authen_header.to_str() {
        return authen_str.starts_with("bearer") || authen_str.starts_with("Bearer");
    }
    false
}*/
