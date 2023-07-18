use axum::async_trait;
pub mod test_db;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait DataBaseInterface {
    fn add(&mut self, user_name: String, password: String);
    fn compare_password(&self, user_name: &str, password: &str) -> bool;
}
