use crate::models::*;
use crate::establish_connection;
use diesel::prelude::*;

pub fn insert_user(data: NewUser) -> ResponseUser {
    use crate::schema::users;

    let connection = &mut establish_connection();

    let result: User = diesel::insert_into(users::table)
        .values(&data)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new user");

    let response = ResponseUser { id: result.id, username: result.username, first_name: result.first_name, last_name: result.last_name };
    response
}