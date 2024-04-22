Simple Storage and Factory in Rust
This project provides a basic implementation of a key-value storage system and a factory for creating storage instances in Rust.

Features
Stores data using a HashMap for efficient retrieval.
Uses structs to organize user data (name and email in this example).
Provides a factory for creating new storage instances.
Usage
Clone the repository.
Run cargo build to compile the code.
Run the program with cargo run.
Example:

Rust
let mut storage = StorageFactory::create_storage();
storage.set_user("user1", UserData { name: "Alice".to_string(), email: "alice@example.com".to_string() });

let user_data = storage.get_user("user1");

if let Some(data) = user_data {
    println!("User data for 'user1': {:?}", data);
} else {
    println!("User 'user1' not found");
}

This example creates a storage instance, adds a user with an ID of "user1" and sets their name and email. It then retrieves the user data and prints it if found.
