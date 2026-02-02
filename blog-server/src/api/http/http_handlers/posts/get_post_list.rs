use crate::api::http::errors::ApiError;
use crate::api::http::http_handlers::posts::PostResponse;
use crate::application::blog::get_post_list_handler;
use crate::application::contracts::PostRepository;
use actix_web::{HttpResponse, get, web};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[get("/posts")]
#[tracing::instrument(name = "Get post list", skip(post_repo))]
pub(crate) async fn get_post_list(
    pagination: web::Query<Pagination>,
    post_repo: web::Data<Arc<dyn PostRepository>>,
) -> Result<HttpResponse, ApiError> {
    let (posts, total) =
        get_post_list_handler(pagination.limit, pagination.offset, post_repo.get_ref())
            .await
            .map_err(ApiError::from)?;
    let response = Response {
        posts: posts.iter().map(PostResponse::from).collect(),
        total,
        limit: pagination.limit,
        offset: pagination.offset,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Debug, Deserialize)]
struct Pagination {
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default = "default_offset")]
    offset: usize,
}

fn default_limit() -> usize {
    10
}

fn default_offset() -> usize {
    0
}

#[derive(Serialize)]
struct Response<'a> {
    posts: Vec<PostResponse<'a>>,
    total: usize,
    limit: usize,
    offset: usize,
}
