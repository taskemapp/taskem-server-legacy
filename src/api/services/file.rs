use crate::domain::repositories::file::FileRepository;
use crate::domain::repositories::user::UserRepository;
use axum::extract::Path;
use axum::response::IntoResponse;
use derive_new::new;
use hyper::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct UserFileParams {
    pub user_name: String,
    pub file_name: String,
}

#[derive(new, Clone)]
pub struct FileServiceData {
    pub file_repository: Arc<dyn FileRepository>,
    pub user_repository: Arc<dyn UserRepository>,
}

pub async fn user_file_handler(
    axum::extract::Extension(service_data): axum::extract::Extension<FileServiceData>,
    Path(UserFileParams {
        user_name,
        file_name,
    }): Path<UserFileParams>,
) -> impl IntoResponse {
    // TODO: проврить по юзер айдишнику есть ли доступ к файлу у пользователя для тимы отдельный ендпоинт
    // проверить в какой он команде или это бакет с его юзер неймом

    // let user_repository = service_data.user_repository.clone();
    let file_repository = service_data.file_repository.clone();

    // user_repository
    //     .get_by_name(&user_name)
    //     .map_err(|e| {
    //         tracing::error!("{:?}", e);
    //         StatusCode::NOT_FOUND
    //     })
    //     .unwrap();

    file_repository
        .download("users", format!("{}/{}", user_name, file_name).as_str())
        .await
        .map_err(|e| {
            tracing::error!("{:?}", e);
            (StatusCode::NOT_FOUND, e.to_string())
        })
        .unwrap()
}

// async fn download_file(bucket: &str, key: &str) -> Result<(), Box<dyn Error>> {
//     // let mut output = Vec::new();
//     // let resp = client.get_object()
//     //     .bucket(bucket)
//     //     .key(key)
//     //     .send()
//     //     .await?;
//     //
//     // let mut stream = resp.body.into_async_read();
//     // stream.read_to_end(&mut output).await?;

//     // Ok(Bytes::from(output))
//     Ok(())
// }
