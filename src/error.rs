#[derive(Debug)]
pub enum Error {
    RpcError,
    WrongAddr,
}
pub type Result<T> = std::result::Result<T, Error>;
