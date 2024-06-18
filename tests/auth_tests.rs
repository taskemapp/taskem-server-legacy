mod common;

#[cfg(test)]
mod auth_tests {
    mod auth {
        tonic::include_proto!("auth");
    }

    use std::error::Error;

    use auth::*;
    use insta::{assert_debug_snapshot, assert_yaml_snapshot};

    #[tokio::test]
    async fn sign_up() -> Result<(), Box<dyn Error>> {
        let channel = tonic::transport::Channel::from_static("http://0.0.0.0:50051")
            .connect()
            .await
            .unwrap();
        let mut client = auth_client::AuthClient::new(channel);

        let sign_up_req = SignupRequest {
            email: String::from("test@test.com"),
            user_name: String::from("test"),
            password: String::from("test"),
        };

        let response = client.sign_up(sign_up_req).await.unwrap();
        let inner = response.into_inner();
        assert_eq!(inner.message, "User successfully created");
        assert_debug_snapshot!(inner);

        Ok(())
    }

    #[tokio::test]
    async fn login() -> Result<(), Box<dyn Error>> {
        let channel = tonic::transport::Channel::from_static("http://0.0.0.0:50051")
            .connect()
            .await
            .unwrap();
        let mut client = auth_client::AuthClient::new(channel);

        let email = "test-login@test.com";
        let pass = "test";
        let user_name = "test-login";

        let sign_up_req = SignupRequest {
            email: String::from(email),
            user_name: String::from(user_name),
            password: String::from(pass),
        };

        client.sign_up(sign_up_req).await.unwrap();

        let login_req = LoginRequest {
            email: String::from(email),
            password: String::from(pass),
        };

        let response = client.login(login_req).await.unwrap();
        let inner= response.into_inner();

        assert_eq!(inner.user_name, user_name);
        assert_yaml_snapshot!(format!("User logged with name {}", inner.user_name));

        Ok(())
    }
}
