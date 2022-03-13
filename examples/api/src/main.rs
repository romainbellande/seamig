extern crate seamig;
use api::Entity;
use seamig::Entity;

#[derive(Entity)]
struct User {
    #[column(type = varchar)]
    pub foo: String,
}

fn main() {
    User::create_table();
}
