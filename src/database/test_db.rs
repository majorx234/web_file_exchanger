/*
 * This file is part of the web_file_exchanger distribution (https://github.com/majorx234/web_file_exchanger ).
 * Copyright (c) 2023-2024 Majorx234 <majorx234@googlemail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::database::DataBaseInterface;
use axum::async_trait;

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
