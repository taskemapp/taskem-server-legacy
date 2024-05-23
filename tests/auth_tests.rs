mod common;

#[cfg(test)]
mod auth_tests {
    mod auth {
        tonic::include_proto!("auth");
    }

    use auth::*;

    use crate::common;

    #[tokio::test]
    async fn sign_up() {
        common::setup();
        let channel = tonic::transport::Channel::from_static("http://0.0.0.0:50051")
            .connect()
            .await
            .unwrap();
        let mut client = auth_client::AuthClient::new(channel);

        let sign_up_req = SignupRequest::default();

        let response = client.sign_up(sign_up_req).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn login() {
        let channel = tonic::transport::Channel::from_static("http://0.0.0.0:50051")
            .connect()
            .await
            .unwrap();
        let mut client = auth_client::AuthClient::new(channel);

        let login_req = LoginRequest::default();

        let response = client.login(login_req).await;

        assert!(response.is_ok());
    }
}
