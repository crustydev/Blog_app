use actix_web::dev::ServiceRequest;
use super::jwt;

pub fn check_token(token: String) -> Result<String, &'static str> {
    match jwt::JwtToken::decode(token) {
        Ok(_token) => Ok(String::from("passed")),
        Err(message) => Err(message)
    }
}

pub fn extract_header_token(request: &ServiceRequest) -> 
            Result<String, &'static str>
{
    match request.headers().get("blogger-token") {
        Some(token) => {
            match token.to_str() {
                Ok(processed_token) => Ok(
                    String::from(processed_token)),
                Err(_processed_token) => Err(
                    "there was an error processing token")
            }
        },
        None => Err("there is no token")
    }
}


#[cfg(test)]
mod check_credentials_test {
    use super::check_token;
    use super::extract_header_token;
    use super::super::jwt::JwtToken;
    use actix_web::test;

    #[test]
    fn correct_check_token() {
        let encoded_token: String = JwtToken::encode(32);

        match check_token(encoded_token) {
            Ok(message) => assert_eq!(message, String::from("passed")),
            _ => panic!("Correct password should be passed")
        }
    }

    #[test]
    fn incorrect_check_token() {
        let encoded_token: String = String::from("test");

        match check_token(encoded_token) {
            Err(message) => assert_eq!(message, "Could not decode"),
            _ => panic!("Incorrect password should return an error")
        }
    }

    #[test]
    fn no_token_in_extract_header_token() {
        let mock_request = test::TestRequest::with_header(
                            "test", "test").to_srv_request();
        let result = extract_header_token(&mock_request);

        match result {
            Err(message) => assert_eq!(message, "there is no token"),
            _ => panic!("should return an error since blogger-token 
                            doesn't exist in the header")
        }
    }

    #[test]
    fn correct_token_in_header_token() {
        let mock_request = test::TestRequest::with_header(
            "blogger-token", "expected").to_srv_request();
        let result = extract_header_token(&mock_request);

        match result {
            Ok(token) => assert_eq!(token, String::from("expected")),
            _ => panic!("token must be present in header")
        }
    }
}