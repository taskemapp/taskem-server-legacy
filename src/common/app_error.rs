use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    CreateGrpcServer,
    CreateAxumServer,
}
