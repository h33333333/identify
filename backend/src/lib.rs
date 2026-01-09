pub mod logging;

pub type Result<T> = eyre::Result<T, eyre::Report>;
