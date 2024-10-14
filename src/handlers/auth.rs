use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use validator::Validate;
use crate::models::user::{UserLoginDto, UserRegisterDto, UserResponse};
use crate::utils::response::StandardResponse;
use bcrypt::{hash, DEFAULT_COST};


pub async fn register(pool: web::Data<PgPool>, body: web::Json<UserRegisterDto>) -> impl Responder {
    if let Err(errors) = body.validate() {
        let error_message = errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                format!("Field '{}' - {}", field, errors[0].message.clone().unwrap_or_default())
            })
            .collect::<Vec<String>>()
            .join(", ");
        
        return HttpResponse::BadRequest().json(StandardResponse::<String>::new_error(
            400,
            format!("Invalid registration data: {}", error_message),
        ));
    }

    let existing_user = sqlx::query!(
        r#"
        SELECT id FROM users WHERE email = $1
        "#,
        body.email
    )
    .fetch_optional(pool.get_ref())
    .await;

    match existing_user {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(StandardResponse::<()>::new_error(
                409,
                "Email already exists".to_string(),
            ));
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(StandardResponse::<String>::new_error(
                500,
                "Database query failed".to_string(),
            ));
        }
        _ => {}
    }

    let hashed_password = match hash(&body.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(StandardResponse::<String>::new_error(500, "Password hashing failed".to_string()));
        }
    };

    // Insert the user into the database
    let result = sqlx::query!(
        r#"
        INSERT INTO users (email, password, created_at, updated_at)
        VALUES ($1, $2, NOW(), NOW())
        RETURNING id, email, created_at, updated_at
        "#,
        body.email,
        hashed_password
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => {
            let user_response = UserResponse {
                id: user.id,
                email: user.email,
                created_at: user.created_at,
                updated_at: user.updated_at,
            };
            HttpResponse::Ok().json(StandardResponse::new(
                "User registered successfully".to_string(),
                user_response,
            ))
        },
        Err(_) => HttpResponse::InternalServerError().json(StandardResponse::<String>::new_error(
            500,
            "Failed to register user".to_string(),
        )),
    }
}

pub async fn login(body: web::Json<UserLoginDto>) -> impl Responder {
    let email = &body.email;

    let response = StandardResponse::success("Login successfully".to_string(), Some(email.clone()));

    HttpResponse::Ok().json(response)
}