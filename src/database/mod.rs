use axum::async_trait;

#[async_trait]
pub trait DataBaseInterface {
    fn add(&mut self, user_name: String, password: String);
    fn compare_password(&self, user_name: &str, password: &str) -> bool;
}

#[derive(Clone)]
pub struct TestDb {
    users: Vec<(String, String)>,
}

impl TestDb {
    pub fn new() -> Self {
        TestDb { users: Vec::new() }
    }
}

impl Default for TestDb {
    fn default() -> Self {
        TestDb { users: Vec::new() }
    }
}

#[async_trait]
impl DataBaseInterface for TestDb {
    fn add(&mut self, user_name: String, password: String) {
        self.users.push((user_name, password));
    }

    fn compare_password(&self, user_name: &str, password: &str) -> bool {
        for (user, psw) in self.users.iter() {
            if user == user_name {
                if psw == password {
                    return true;
                }
            }
        }
        false
    }
}
