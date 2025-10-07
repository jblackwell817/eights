use axum::{routing::get, Router};
use eights::model::Result;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route(
        "/",
        get(|| async {
            let mens_results = load_mens_data();
            let womens_results = load_womens_data();
            format!(
                "Men's results: {:?}, \nWomen's results: {:?}",
                mens_results, womens_results
            )
        }),
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn load_mens_data() -> Vec<Result> {
    let mens_data = include_str!("data/results_men.json");
    serde_json::from_str(mens_data).unwrap()
}

fn load_womens_data() -> Vec<Result> {
    let womens_data = include_str!("data/results_women.json");
    serde_json::from_str(womens_data).unwrap()
}
