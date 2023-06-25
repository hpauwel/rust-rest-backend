pub struct User {
    username: String,
    first_name: String,
    last_name: String,
    password: String,
}

impl User {
    pub fn username(&mut self) -> &mut String {
        &mut self.username
    }

    pub fn first_name(&mut self) -> &mut String {
        &mut self.first_name
    }

    pub fn last_name(&mut self) -> &mut String {
        &mut self.last_name
    }

    pub fn password(&self) -> &str {
        &self.password
    }

}