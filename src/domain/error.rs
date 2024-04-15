#[derive(Clone, Debug)]
pub struct RepositoryError {
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct RedisError {
    pub message: String,
}
