use crate::models::*;
use crate::establish_connection;
use diesel::prelude::*;

pub fn load_user(uid: i32) -> ResponseUser {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let result: User = users
        .filter(id.eq(uid))
        .first::<User>(connection)
        .expect("Error loading user");

    let result_user = ResponseUser { id: result.id, username: result.username, first_name: result.first_name, last_name: result.last_name };
    result_user
}

pub fn load_all() -> Vec<ResponseUser> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let result: Vec<User> = users
        .load::<User>(connection)
        .expect("Error loading users");

    let mut result_users = vec![];
    for i in result {
        result_users.push(ResponseUser {
            id: i.id,
            username: i.username,
            first_name: i.first_name,
            last_name: i.last_name
        });
    }

    result_users
}