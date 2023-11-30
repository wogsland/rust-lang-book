fn main() {
    let mut mr_user = User {
            active: false,
            username: String::from("Dark Helmet"),
            email: String::from("dh@spaceball.planet"),
            sign_in_count: 0,
    };

    mr_user.email = String::from("roland@druidia.planet");

    let ms_user = User {
        email: String::from("vespa@druidia.planet"),
        ..mr_user
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual; //WTF?
