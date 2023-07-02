use crate::models::*;
use crate::establish_connection;
use diesel::prelude::*;

pub fn load_all() -> Vec<ResponsePost> {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let result: Vec<Post> = posts
        .load::<Post>(connection)
        .expect("Error loading posts");

    let mut result_posts = vec![];
    for i in result {
        result_posts.push(ResponsePost {
            title: i.title,
            body: i.body
        });
    }

    result_posts
}