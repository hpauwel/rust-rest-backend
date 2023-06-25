use crate::models::*;
use crate::establish_connection;
use diesel::prelude::*;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn insert_user(data: &mut NewUser) -> ResponseUser {
    use crate::schema::users;

    let connection = &mut establish_connection();

    let hashed_pwd = hash_password(&data.password);

    let updated_data = NewUser {
        username: data.username,
        first_name: data.first_name,
        last_name: data.last_name,
        password: hashed_pwd.as_str()
    };

    let result: User = diesel::insert_into(users::table)
        .values(updated_data)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new user");

    let response = ResponseUser { id: result.id, username: result.username, first_name: result.first_name, last_name: result.last_name };
    response
}

fn hash_password(pwd: &str) -> String{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed = argon2.hash_password(pwd.as_bytes(), &salt).unwrap();
    hashed.to_string()
}