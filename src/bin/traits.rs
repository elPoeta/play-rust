trait User {
    fn new(email: String, password: String) -> Self;
    fn email_confirmed(&self) -> bool;
}

struct Profile {
    name: String,
    email: String,
    password: String,
    confirmed: bool,
}

impl User for Profile {
    fn new(email: String, password: String) -> Profile {
        return Profile {
            email,
            password,
            confirmed: false,
            name: String::from(""),
        };
    }

    fn email_confirmed(&self) -> bool {
        return self.confirmed;
    }
}

impl ToString for Profile {
    fn to_string(&self) -> String {
        format!(
            "Name: {}, Email: {}, Password: {}, confirmed: {}",
            self.name,
            self.email,
            self.password.replace(&self.password, "********"),
            self.confirmed
        )
    }
}

fn main() {
    let mut profile = Profile::new("elpoeta@gmail.com".to_string(), "123456".to_string());

    profile.name = String::from("Leonardo");
    println!("{}", profile.to_string());
}
