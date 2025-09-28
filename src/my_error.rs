use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    Io(#[from] std::io::Error),
}
