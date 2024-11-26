use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
use crate::consts::{LISTMONK_ADDR_VAR, LISTMONK_LIST_ID_VAR, LISTMONK_TOKEN_VAR};
use crate::utils::{release_batch};

fn admin_emails() -> Vec<String> {
    dotenv::var("ADMIN_EMAILS").unwrap().split(",").map(|s| s.to_string()).collect()
}

struct AdminUser {
    user: String,
    email: String
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request.headers().get_one("X-User");
        let email = request.headers().get_one("X-Email");
        let error = Outcome::Forward(Status::Unauthorized);

        match (user, email) {
            (Some(user), Some(email)) => if admin_emails().contains(&email.to_string()) {
                Outcome::Success(AdminUser { user: user.to_string(), email: email.to_string() })
            } else { error },
            _ => error
        }
    }
}

trait ListmonkRequestBuilder {
    fn listmonk_auth(self) -> reqwest::RequestBuilder;
}
impl ListmonkRequestBuilder for reqwest::RequestBuilder {
    fn listmonk_auth(self) -> reqwest::RequestBuilder {
        self.header("Authorization", format!("token {}", dotenv::var(LISTMONK_TOKEN_VAR).unwrap()))
    }
}

struct User {
    user: String,
    email: String
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request.headers().get_one("X-User");
        let email = request.headers().get_one("X-Email");

        match (user, email) {
            (Some(user), Some(email)) => Outcome::Success(User { user: user.to_string(), email: email.to_string() }),
            _ => Outcome::Forward(Status::Unauthorized)
        }
    }
}

#[get("/release_batch")]
fn release_batch_route(_user: AdminUser) {
    println!("Release batch");
    release_batch();
}

#[get("/release_batch", rank = 2)]
fn release_batch_route_2() -> &'static str {
    "You are not authorized to perform this action."
}

#[derive(Serialize)]
struct ListmonkSubscriber {
    email: String,
    name: String,
    status: String,
    lists: Vec<u32>
}

#[post("/subscribe")]
async fn subscribe_route(user: User) {
    println!("Subscribing {}", user.email);
    let client = reqwest::Client::new();
    let s = ListmonkSubscriber {
        email: user.email.to_owned(),
        name: user.email.to_owned(),
        status: "enabled".to_owned(),
        lists: vec! [dotenv::var(LISTMONK_LIST_ID_VAR).unwrap().parse::<u32>().unwrap()]
    };
    let _res = client.post(dotenv::var(LISTMONK_ADDR_VAR).unwrap() + "/api/subscribers")
        .listmonk_auth()
        .json(&s)
        .send()
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct SubscribersQueryResponse {
    data: SubscribersQueryData
}
#[derive(Deserialize)]
struct SubscribersQueryData {
    results: Vec<SubscribersQueryResult>
}
#[derive(Deserialize)]
struct SubscribersQueryResult {
    id: u32
}

#[get("/is_subscribed")]
async fn is_subscribed_route(user: User) -> String {
    let client = reqwest::Client::new();
    let query = format!("subscribers.email = '{}'", user.email);
    let res = client.get(dotenv::var(LISTMONK_ADDR_VAR).unwrap() + "/api/subscribers")
        .listmonk_auth()
        .query(&[("query", query)])
        .send()
        .await.unwrap()
        .json::<SubscribersQueryResponse>()
        .await.unwrap();
    (res.data.results.len() > 0).to_string()
}

pub async fn rocket() {
    println!("Rocket starting");
    let _start = rocket::build().mount("/mail-handler/", routes![release_batch_route, release_batch_route_2, subscribe_route, is_subscribed_route]).launch().await;
}