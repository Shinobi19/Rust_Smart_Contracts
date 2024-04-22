use std::collections::HashMap;

#[derive(Debug)]
pub struct UserData {
    name: String,
    email: String,
}

#[derive(Debug)]
pub struct Storage {
    data: HashMap<String, UserData>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: HashMap::new(),
        }
    }

    pub fn set_user(&mut self, user_id: &str, user_data: UserData) {
        self.data.insert(user_id.to_string(), user_data);
    }

    pub fn get_user(&self, user_id: &str) -> Option<UserData> {
        self.data.get(user_id)
            .map(|v| v.clone()) // Clone the value to avoid ownership issues
    }
}
