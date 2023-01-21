/*
    Appellation: handlers <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::handler::*;

pub(crate) mod handler;

pub(crate) mod specs {
    use async_trait::async_trait;

    ///
    #[async_trait]
    pub trait AsyncHandle: Clone + Send + Sync {
        type Error: std::error::Error + Send + Sync;

        async fn handler(&self) -> Result<&Self, Self::Error>
        where
            Self: Sized;
    }
    ///
    pub trait Handle: Clone {
        type Error: std::error::Error + 'static;

        fn handler(&self) -> Result<&Self, Self::Error>
        where
            Self: Sized;
    }
}
