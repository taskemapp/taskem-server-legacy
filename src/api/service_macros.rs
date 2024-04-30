#[macro_export]
macro_rules! extract_user_id_from_metadata {
    ($request:expr) => {{
        let metadata = $request.metadata().clone();

        if !metadata.contains_key(MIDDLEWARE_AUTH_SESSION_KEY) {
            return Err(tonic::Status::unauthenticated(
                "Unauthenticated".to_string(),
            ));
        }

        let metadata_value = metadata.get(MIDDLEWARE_AUTH_SESSION_KEY).unwrap();

        let user_id = match uuid::Uuid::from_str(
            metadata_value
                .to_str()
                .expect("Failed to convert metadata value to str"),
        ) {
            Ok(value) => value,
            Err(_) => {
                return Err(tonic::Status::invalid_argument("Invalid user id"));
            }
        };

        user_id
    }};
}
