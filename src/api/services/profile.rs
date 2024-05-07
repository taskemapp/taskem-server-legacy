use crate::domain::constants::MIDDLEWARE_AUTH_SESSION_KEY;
use crate::domain::repositories::file::FileRepository;
use crate::domain::repositories::user::UserRepository;
use crate::extract_user_id_from_metadata;
use crate::profile::profile_server::Profile;
use crate::profile::AddOrUpdateAvatarRequest;
use crate::profile::GetProfileResponse;
use autometrics::autometrics;
use derive_new::new;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

#[derive(new)]
pub struct ProfileServiceImpl {
    pub(self) file_repository: Arc<dyn FileRepository>,
    pub(self) user_repository: Arc<dyn UserRepository>,
    pub(self) file_service_url: String,
}

#[async_trait]
#[autometrics]
impl Profile for ProfileServiceImpl {
    async fn add_or_update_avatar(
        &self,
        request: Request<AddOrUpdateAvatarRequest>,
    ) -> Result<Response<()>, Status> {
        let file_repository = self.file_repository.clone();
        let user_repository = self.user_repository.clone();

        let user_id = extract_user_id_from_metadata!(&request);

        let add_or_update_request = request.into_inner();

        if add_or_update_request.avatar_image.is_empty() {
            return Err(Status::invalid_argument("Empty avatar image"));
        }

        if add_or_update_request.avatar_image.len() > 4 * 1024 * 1024 {
            return Err(Status::invalid_argument(
                "Invalid picture size, must be less than 4MB",
            ));
        }

        let user = user_repository
            .get(&user_id)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        user_repository
            .set_profile_picture(
                &user_id,
                &format!(
                    "{}/{}/{}",
                    self.file_service_url, &user.user_name, "avatar.jpg"
                ),
            )
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        file_repository
            .upload(
                "users",
                format!("{}/{}", user.user_name, "avatar.jpg").as_str(),
                add_or_update_request.avatar_image.as_slice(),
            )
            .await
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        Ok(Response::new(()))
    }

    async fn get_profile(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetProfileResponse>, Status> {
        todo!()
    }
}
