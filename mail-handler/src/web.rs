use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::utils::{release_batch};

struct User {
    user: String,
    email: String
}

fn admin_emails() -> Vec<String> {
    dotenv::var("ADMIN_EMAILS").unwrap().split(",").map(|s| s.to_string()).collect()
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request.headers().get_one("X-User");
        let email = request.headers().get_one("X-Email");
        let error = Outcome::Forward(Status::Unauthorized);

        match (user, email) {
            (Some(user), Some(email)) => if admin_emails().contains(&email.to_string()) {
                Outcome::Success(User { user: user.to_string(), email: email.to_string() })
            } else { error },
            _ => error
        }
    }
}

#[get("/release_batch")]
fn release_batch_route(_user: User) {
    println!("Release batch");
    release_batch();
}

#[get("/release_batch", rank = 2)]
fn release_batch_route_2() -> &'static str {
    "You are not authorized to perform this action."
}

pub async fn rocket() {
    println!("Rocket starting");
    let _start = rocket::build().mount("/mail-handler/", routes![release_batch_route, release_batch_route_2]).launch().await;
}