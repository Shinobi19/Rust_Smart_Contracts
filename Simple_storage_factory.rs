use crate::Storage;
use crate::UserData;

pub struct StorageFactory {}

impl StorageFactory {
    pub fn create_storage() -> Storage {
        Storage::new()
    }

    pub fn create_user(&mut self, storage: &mut Storage, user_id: &str, name: &str, email: &str) {
        let user_data = UserData {
            name: name.to_string(),
            email: email.to_string(),
        };
        storage.set_user(user_id, user_data);
    }
}
