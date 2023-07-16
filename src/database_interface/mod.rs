#[derive(Clone)]
pub struct DataBaseInterface {
    users: Vec<(String, String)>,
}

impl DataBaseInterface {
    pub fn new() -> Self {
        DataBaseInterface { users: Vec::new() }
    }

    pub fn add(&mut self, user_name: String, password: String) {
        self.users.push((user_name, password));
    }

    pub fn compare_password(&self, user_name: &str, password: &str) -> bool {
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

impl Default for DataBaseInterface {
    fn default() -> Self {
        DataBaseInterface { users: Vec::new() }
    }
}
