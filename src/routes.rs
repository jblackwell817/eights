use std::sync::LazyLock;

use crate::model::{AnnualResult, Crew, CrewResult, Position, Year};
use axum::{extract::Path, routing::get, Json, Router};

static MENS_RESULTS: LazyLock<String> =
    LazyLock::new(|| include_str!("data/results_men.json").to_string());
static WOMENS_RESULTS: LazyLock<String> =
    LazyLock::new(|| include_str!("data/results_women.json").to_string());

async fn get_mens_results() -> Json<Vec<AnnualResult>> {
    let results: Vec<AnnualResult> = serde_json::from_str(&MENS_RESULTS).unwrap();
    Json(results)
}

async fn get_womens_results() -> Json<Vec<AnnualResult>> {
    let results: Vec<AnnualResult> = serde_json::from_str(&WOMENS_RESULTS).unwrap();
    Json(results)
}

async fn get_all_results_for_mens_crew(Path(crew): Path<Crew>) -> Json<CrewResult> {
    let all_results: Vec<AnnualResult> = serde_json::from_str(&MENS_RESULTS).unwrap();
    get_all_results_for_crew(crew, all_results).await
}

async fn get_all_results_for_womens_crew(Path(crew): Path<Crew>) -> Json<CrewResult> {
    let all_results: Vec<AnnualResult> = serde_json::from_str(&WOMENS_RESULTS).unwrap();
    get_all_results_for_crew(crew, all_results).await
}

async fn get_all_results_for_crew(crew: Crew, all_results: Vec<AnnualResult>) -> Json<CrewResult> {
    let filtered_results: Vec<(Year, Position)> = all_results
        .into_iter()
        .filter(|result| result.standings().contains(&crew))
        .map(|result| {
            (
                result.year(),
                result.standings().iter().position(|c| c == &crew).unwrap() as u8 + 1,
            )
        })
        .collect();
    Json(CrewResult::new(crew, filtered_results))
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/results/mens", get(get_mens_results))
        .route("/results/womens", get(get_womens_results))
        .route(
            "/results/mens/{college}/{boat}",
            get(get_all_results_for_mens_crew),
        )
        .route(
            "/results/womens/{college}/{boat}",
            get(get_all_results_for_womens_crew),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    use axum::body::Body;
    use axum::http::{Request, Response, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn setup_and_get_response(uri: &str) -> Response<Body> {
        let app = app();
        app.oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_hello_world() {
        let response = setup_and_get_response("/").await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, World!");
    }

    #[tokio::test]
    async fn test_get_all_mens_results() {
        let response = setup_and_get_response("/results/mens").await;
        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = std::str::from_utf8(&body_bytes).unwrap();
        let results: Vec<AnnualResult> = serde_json::from_str(body_str).unwrap();
        assert_eq!(results.len(), 120, "Should contain 120 years of results");
    }

    #[tokio::test]
    async fn test_get_all_womens_results() {
        let response = setup_and_get_response("/results/womens").await;
        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = std::str::from_utf8(&body_bytes).unwrap();
        let results: Vec<AnnualResult> = serde_json::from_str(body_str).unwrap();
        assert_eq!(results.len(), 47, "Should contain 47 years of results");
    }

    #[tokio::test]
    async fn test_get_all_results_for_mens_crew() {
        let response = setup_and_get_response("/results/mens/Trinity/1").await;
        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = std::str::from_utf8(&body_bytes).unwrap();
        let crew_result: CrewResult = serde_json::from_str(body_str).unwrap();
        assert_eq!(
            crew_result.crew().college(),
            College::Trinity,
            "College should be Trinity"
        );
        assert_eq!(crew_result.crew().boat(), 1, "Boat should be 1");
        assert_eq!(
            crew_result.results().len(),
            120,
            "Should contain 120 years of results"
        );
    }

    #[tokio::test]
    async fn test_get_all_results_for_womens_crew() {
        let response = setup_and_get_response("/results/womens/Hertford/2").await;
        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = std::str::from_utf8(&body_bytes).unwrap();
        let crew_result: CrewResult = serde_json::from_str(body_str).unwrap();
        assert_eq!(
            crew_result.crew().college(),
            College::Hertford,
            "College should be Hertford"
        );
        assert_eq!(crew_result.crew().boat(), 2, "Boat should be 2");
        assert_eq!(
            crew_result.results().len(),
            38,
            "Should contain 38 years of results"
        );
    }
}
