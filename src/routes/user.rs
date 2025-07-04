use crate::{
    models::{
        user::User,
        user_dto::{AuthResponse, LoginUser, RegisterUser, UserResponse},
    },
    schema::users::dsl::*,
    services::auth,
    state::AppState,
    utils::error::AppError,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use diesel::prelude::*;

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUser>,
) -> Result<impl IntoResponse, AppError> {
    // Hash password
    let hashed_password = auth::hash_password(&payload.password)?;

    // Insert user into DB
    let mut conn = state
        .db_pool
        .get()
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    // Insert without get_result (RETURNING not supported)
    diesel::insert_into(users)
        .values((
            username.eq(&payload.username),
            email.eq(&payload.email),
            password_hash.eq(&hashed_password),
        ))
        .execute(&mut conn)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // Query the inserted user by email or username
    let inserted_user = users
        .filter(email.eq(&payload.email))
        .first::<User>(&mut conn)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let response = UserResponse {
        id: inserted_user.id,
        username: inserted_user.username,
        email: inserted_user.email,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // Fetch user from DB
    let user: User = users
        .filter(username.eq(&payload.username))
        .first(&mut conn)
        .map_err(|_| AppError::Unauthorized("Invalid username or password".into()))?;

    // Verify password
    let is_valid = auth::verify_password(&user.password_hash, &payload.password)?;
    if !is_valid {
        return Err(AppError::Unauthorized(
            "Invalid username or password".into(),
        ));
    }

    // Create JWT token
    let token = auth::create_jwt(user.id, &state.config)?;

    Ok((StatusCode::OK, Json(AuthResponse { token })))
}
