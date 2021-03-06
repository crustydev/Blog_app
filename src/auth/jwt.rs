extern crate hmac;
extern crate jwt;
extern crate sha2;

use hmac::{Hmac, NewMac};
use jwt::{Header, Token, VerifyWithKey, SignWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

use actix_web::HttpRequest;

pub struct JwtToken {
    pub blogger_id: i32,
    pub body: String
}


impl JwtToken {
    pub fn encode (blogger_id: i32) -> String {
        let key: Hmac<Sha256> = Hmac::new_varkey(b"secret").unwrap();
        let  mut claims = BTreeMap::new();
        claims.insert("blogger_id", blogger_id);
        let token_str: String = claims.sign_with_key(&key).unwrap();
        return token_str
    }

    pub fn decode(encoded_token: String) ->
            Result<JwtToken, &'static str> {
        
        let key: Hmac<Sha256> = Hmac::new_varkey(b"secret").unwrap();
        let token_str: &str = encoded_token.as_str();

        let token: Result<Token<Header, BTreeMap<String,
            i32>, _>, _> = VerifyWithKey::verify_with_key(
            token_str, &key);

        match token {
            Ok(token) => {
                let _header = token.header();
                let claims = token.claims();
                return Ok(JwtToken {
                    blogger_id: claims["blogger_id"],
                    body: encoded_token
                })
            }
            Err(_) => return Err("Could not decode")
        }
    }


    pub fn decode_from_request(request: HttpRequest)
    -> Result<JwtToken, &'static str> {
        match request.headers().get("blogger-token") {
            Some(token) => JwtToken::decode(
                String::from(token.to_str().unwrap())),
            None => Err("there is no token")
        }
    }
}



#[cfg(test)]
mod jwt_test {
    use super::JwtToken;
    use actix_web::test;


    #[test]
    fn encode_decode() {
        let encoded_token: String = JwtToken::encode(32);
        let decoded_token: JwtToken = JwtToken::decode(
            encoded_token).unwrap();
        assert_eq!(32, decoded_token.blogger_id);
    }

    #[test]
    fn decode_incorrect_token() {
        let encoded_token: String = String::from("test");

        match JwtToken::decode(encoded_token) {
            Err(message) => assert_eq!("Could not decode",
                message),
            _ => panic!("Incorrect token should not be able to be
                    encoded")
        }
    }

    #[test]
    fn decode_from_request_with_correct_token() {
        let encoded_token: String = JwtToken::encode(32);
        let request = test::TestRequest::with_header(
            "blogger-token", encoded_token).to_http_request();
        let outcome = JwtToken::decode_from_request(request);

        match outcome {
            Ok(token) => assert_eq!(32, token.blogger_id),
            _ => panic!("Token is not returned when it should be")
        }
    }

    #[test]
    fn decode_from_request_with_no_token() {
        let request = test::TestRequest::default().to_http_request();
        let outcome = JwtToken::decode_from_request(request);

        match outcome {
            Err(message) => assert_eq!("there is no token", message),
            _ => panic!("Token should not be returned when it is not
                        present in the header") 
        }
    }

    #[test]
    fn decode_from_request_with_false_token() {
        let request = test::TestRequest::with_header(
            "blogger-token", "test").to_http_request();
        let outcome = JwtToken::decode_from_request(request);
        
        match outcome {
            Err(message) => assert_eq!(message, "Could not decode"),
            _ => panic!("should be an error with a false token")
        }
    }
}