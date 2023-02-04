use async_trait::async_trait;
use rand;
use rand::distributions::Uniform;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[async_trait]
pub trait QuotesStorage: Sync + Send {
    async fn get_quote(&mut self) -> String;
}

pub struct QuotesStorageImpl {
    quotes: Vec<String>,
    rand_range: SmallRng,
}

impl QuotesStorageImpl {
    pub fn new() -> QuotesStorageImpl {
        let quotes = vec!["Quote 1".to_string(), "Quote 2".to_string()];
        QuotesStorageImpl {
            quotes,
            rand_range: SmallRng::from_entropy(),
        }
    }
}

#[async_trait]
impl QuotesStorage for QuotesStorageImpl {
    async fn get_quote(&mut self) -> String {
        let x = self.rand_range.sample(Uniform::new(0, self.quotes.len()));
        self.quotes[x].to_string()
    }
}
