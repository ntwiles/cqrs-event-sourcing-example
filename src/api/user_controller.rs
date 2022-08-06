use axum::{extract::Path, http::StatusCode, Extension, Json};
use bson::oid;

use std::sync::Arc;

use crate::{application::query::user::UserStore, domain::user::User};

pub async fn read(
    Path(user_id): Path<String>,
    Extension(user_store): Extension<Arc<UserStore>>,
) -> Result<Json<User>, StatusCode> {
    let user_id = oid::ObjectId::parse_str(user_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    user_store
        .get(user_id)
        .await
        .map(|c| Json(c))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
