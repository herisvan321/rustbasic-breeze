use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use colored::*;
use regex::Regex;

pub fn update_controller_mod_rs(mod_name: &str) {
    let mod_path = "src/app/http/controllers/mod.rs";
    let mut content = String::new();
    if let Ok(mut file) = fs::File::open(mod_path) {
        file.read_to_string(&mut content).ok();
    }

    let line = format!("pub mod {};", mod_name);
    if content.contains(&line) {
        return;
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(mod_path)
        .expect("Gagal membuka controllers/mod.rs");

    writeln!(file, "{}", line).ok();
    println!("{} {}", "📝".blue(), "controllers/mod.rs diperbarui.".dimmed());
}

pub fn update_migration_mod_rs(mod_name: &str) {
    let mod_path = "database/migrations/mod.rs";
    let mut content = String::new();
    if let Ok(mut file) = fs::File::open(mod_path) {
        file.read_to_string(&mut content).ok();
    }

    // Tambahkan mod declaration
    if !content.contains(&format!("pub mod {};", mod_name)) {
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(&format!("pub mod {};\n", mod_name));
    }

    // Tambahkan ke list migrations
    let search_pattern = "fn migrations() -> Vec<Box<dyn sea_orm_migration::prelude::MigrationTrait>> {";
    let search_pattern_alt = "fn migrations() -> Vec<Box<dyn MigrationTrait>> {";
    
    let mut pos = content.find(search_pattern);
    if pos.is_none() {
        pos = content.find(search_pattern_alt);
    }
    
    if let Some(_pos) = pos {
        let insert_pos = content.find("        ]").unwrap_or(content.len());
        content.insert_str(insert_pos, &format!("            Box::new({}::Migration),\n", mod_name));
    }

    fs::write(mod_path, content).expect("Gagal memperbarui database/migrations/mod.rs");
    println!("{} {}", "📝".blue(), "database/migrations/mod.rs diperbarui.".dimmed());
}

pub async fn make_auth() {
    println!("\n{}", "🔐 Scaffolding Authentication...".magenta().bold());

    // Dynamically add validator and sea-orm-migration to Cargo.toml if not present
    if let Ok(mut content) = fs::read_to_string("Cargo.toml") {
        let mut changed = false;
        
        if !content.contains("validator = ") {
            if let Some(pos) = content.find("[dependencies]") {
                let insert_pos = pos + "[dependencies]".len();
                content.insert_str(insert_pos, "\nvalidator = { version = \"0.20\", features = [\"derive\"] }");
                changed = true;
            }
        }
        
        if !content.contains("sea-orm-migration = ") {
            if let Some(pos) = content.find("[dependencies]") {
                let insert_pos = pos + "[dependencies]".len();
                content.insert_str(insert_pos, "\nsea-orm-migration = { version = \"1.1\", features = [\"runtime-tokio-rustls\", \"sqlx-sqlite\", \"sqlx-mysql\"], default-features = false }");
                changed = true;
            }
        }
        
        if changed {
            fs::write("Cargo.toml", content).ok();
            println!("   {} {}", "📝 Updated:".blue(), "Cargo.toml dependencies".cyan());
        }
    }

    // 1. Create src/routes/auth.rs
    let auth_route_path = "src/routes/auth.rs";
    let auth_route_template = r#"use rustbasic_core::axum::{Router, routing::{get, post}, middleware::from_fn};
use crate::app::http::controllers::auth;
use crate::app::http::middleware::auth::guest_middleware;
use rustbasic_core::server::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(auth::auth_controller::AuthController::login_page))
        .route("/login", post(auth::auth_controller::AuthController::login))
        .route("/register", get(auth::auth_controller::AuthController::register_page))
        .route("/register", post(auth::auth_controller::AuthController::register))
        .route("/forgot-password", get(auth::auth_controller::AuthController::forgot_password_page))
        .route("/forgot-password", post(auth::auth_controller::AuthController::send_reset_link))
        .route("/reset-password", get(auth::auth_controller::AuthController::reset_password_page))
        .route("/reset-password", post(auth::auth_controller::AuthController::update_password))
        .layer(from_fn(guest_middleware))
}
"#;
    if !std::path::Path::new(auth_route_path).exists() {
        fs::write(auth_route_path, auth_route_template).ok();
        println!("   {} {}", "✅ Created:".green(), auth_route_path.cyan());
    } else {
        println!("   {} {}", "⚠️  Exists:".yellow(), auth_route_path.cyan());
    }

    // 2. Update src/routes/mod.rs
    let routes_mod_path = "src/routes/mod.rs";
    if let Ok(mut content) = fs::read_to_string(routes_mod_path)
        && !content.contains("pub mod auth;") {
            content.push_str("pub mod auth;\n");
            fs::write(routes_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), routes_mod_path.cyan());
        }

    // 3. Update src/routes/web.rs
    let web_route_path = "src/routes/web.rs";
    if let Ok(mut content) = fs::read_to_string(web_route_path)
        && !content.contains("use crate::routes::auth as auth_routes;") {
            content = content.replace("use rustbasic_core::axum::{Router, routing::get};", "use rustbasic_core::axum::{Router, routing::{get, post}, middleware::from_fn};");
            content = content.replace("use rustbasic_core::server::AppState;", "use crate::app::http::controllers::{auth, dashboard_controller};\nuse crate::app::http::middleware::auth::auth_middleware;\nuse rustbasic_core::server::AppState;\nuse crate::routes::auth as auth_routes;");

            let merge_logic = r#"let auth_protected_routes = Router::new()
        .route("/dashboard", get(dashboard_controller::DashboardController::index))
        .route("/logout", post(auth::auth_controller::AuthController::logout))
        .layer(from_fn(auth_middleware));

    Router::new()
        .route("/", get(welcome_controller::index))
        .route("/about", get(welcome_controller::about))
        .route("/dev", get(welcome_controller::dev_info))
        .merge(auth_routes::router())
        .merge(auth_protected_routes)"#;

            // Use regex for more robust replacement (includes leading spaces)
            let re = Regex::new(r#"(?s)Router::new\(\s*\n\s*\.route\("/", get\(welcome_controller::index\)\)\s*\n\s*\.route\("/about", get\(welcome_controller::about\)\)\s*\n\s*\.route\("/dev", get\(welcome_controller::dev_info\)\)"#).unwrap();
            if re.is_match(&content) {
                content = re.replace(&content, merge_logic).to_string();
            } else {
                // Fallback for simple replacement
                content = content.replace("Router::new()\n        .route(\"/\", get(welcome_controller::index))\n        .route(\"/about\", get(welcome_controller::about))\n        .route(\"/dev\", get(welcome_controller::dev_info))", merge_logic);
            }
            
            fs::write(web_route_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), web_route_path.cyan());
        }

    // 3.1 Create Password Resets Migration
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let migration_name = format!("m{}_create_password_resets_table", timestamp);
    let migration_path = format!("database/migrations/{}.rs", migration_name);
    
    // Check if any password reset migration already exists
    let mut exists = false;
    if let Ok(entries) = std::fs::read_dir("database/migrations") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str()
                && name.ends_with("_create_password_resets_table.rs") {
                    exists = true;
                    println!("   {} {}", "⚠️  Exists:".yellow(), name.cyan());
                    break;
                }
        }
    }

    if !exists {
        let migration_template = r#"use rustbasic_core::sea_orm_migration::prelude::*;
use rustbasic_core::async_trait;

#[derive(Iden)]
enum PasswordResets {
    Table,
    Email,
    Token,
    CreatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PasswordResets::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PasswordResets::Email).string().not_null().primary_key())
                    .col(ColumnDef::new(PasswordResets::Token).string().not_null())
                    .col(
                        ColumnDef::new(PasswordResets::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PasswordResets::Table).to_owned())
            .await
    }
}
"#.to_string();
        fs::write(&migration_path, migration_template).ok();
        
        update_migration_mod_rs(&migration_name);
        println!("   {} {}", "✅ Created:".green(), format!("Migration {}", migration_name).cyan());
    }

    // 4. Create Controller Folder & mod.rs
    let auth_controller_dir = "src/app/http/controllers/auth";
    fs::create_dir_all(auth_controller_dir).ok();
    let auth_controller_mod = "src/app/http/controllers/auth/mod.rs";
    if !std::path::Path::new(auth_controller_mod).exists() {
        fs::write(auth_controller_mod, "pub mod auth_controller;").ok();
    }
    update_controller_mod_rs("auth");

    // 4.1 Create Auth Middleware
    let auth_middleware_dir = "src/app/http/middleware";
    fs::create_dir_all(auth_middleware_dir).ok();
    let auth_middleware_path = "src/app/http/middleware/auth.rs";
    if !std::path::Path::new(auth_middleware_path).exists() {
        let middleware_template = r#"use rustbasic_core::axum::{
    middleware::Next,
    response::{IntoResponse, Redirect},
    extract::Request,
};
use rustbasic_core::session_manager::RustBasicSessionStore;
use rustbasic_core::axum_session::Session;

pub async fn auth_middleware(req: Request, next: Next) -> impl IntoResponse {
    let session = req.extensions().get::<Session<RustBasicSessionStore>>().unwrap();
    if session.get::<i32>("user_id").is_none() {
        session.set("error", "Silakan login terlebih dahulu");
        return Redirect::to("/login").into_response();
    }
    next.run(req).await
}

pub async fn guest_middleware(req: Request, next: Next) -> impl IntoResponse {
    let session = req.extensions().get::<Session<RustBasicSessionStore>>().unwrap();
    if session.get::<i32>("user_id").is_some() {
        return Redirect::to("/dashboard").into_response();
    }
    next.run(req).await
}
"#;
        fs::write(auth_middleware_path, middleware_template).ok();
        
        // Update src/app/http/middleware/mod.rs
        let middleware_mod_path = "src/app/http/middleware/mod.rs";
        if let Ok(mut content) = fs::read_to_string(middleware_mod_path)
            && !content.contains("pub mod auth;") {
                content.push_str("pub mod auth;\n");
                fs::write(middleware_mod_path, content).ok();
            }
        println!("   {} {}", "✅ Created:".green(), auth_middleware_path.cyan());
    }

    // 4.1 Create Password Resets Model
    let model_path = "src/app/models/password_resets.rs";
    if !std::path::Path::new(model_path).exists() {
        let model_template = r#"use rustbasic_core::model;
use rustbasic_core::sea_orm::entity::prelude::*;

model! {
    table: "password_resets",
    timestamps: false,
    fillable: [email, token, created_at],
    guarded: [],
    Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub email: String,
        pub token: String,
        pub created_at: DateTime,
    }
}
"#;
        fs::write(model_path, model_template).ok();
        
        // Update src/app/models/mod.rs
        let models_mod_path = "src/app/models/mod.rs";
        if let Ok(mut content) = fs::read_to_string(models_mod_path)
            && !content.contains("pub mod password_resets;") {
                content.push_str("pub mod password_resets;\n#[allow(unused_imports)]\npub use password_resets::Entity as PasswordReset;\n");
                fs::write(models_mod_path, content).ok();
            }
        println!("   {} {}", "✅ Created:".green(), "Model password_resets".cyan());
    }

    // 4.2 Create Auth Controller
    let auth_controller_path = "src/app/http/controllers/auth/auth_controller.rs";
    if !std::path::Path::new(auth_controller_path).exists() {
        let controller_template = r#"/* ---------------------------------------------------------
 * 📑 LABEL: AUTH CONTROLLER (auth/auth_controller.rs)
 * Menangani pendaftaran, login, dan logout user.
 * --------------------------------------------------------- */

use crate::app::inertia::inertia;
use crate::app::models::{User, PasswordReset};
use rustbasic_core::requests::Request;
use rustbasic_core::server::AppState;
use rustbasic_core::axum::{response::{IntoResponse, Response, Redirect}, extract::State};
use rustbasic_core::bcrypt::{hash, verify, DEFAULT_COST};
use rustbasic_core::uuid::Uuid;
use rustbasic_core::serde::Deserialize;
use rustbasic_core::validator::Validate;
use rustbasic_core::mail::MailService;
use rustbasic_core::sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use rustbasic_core::serde_json::json;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,
    
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    pub password: String,
    pub remember: Option<bool>,
}

#[derive(Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}

pub struct AuthController;

impl AuthController {
    /// Menampilkan halaman login
    pub async fn login_page(req: Request) -> Response {
        inertia(&req, "Auth/Login", json!({ "title": "Login" }))
    }

    /// Menampilkan halaman register
    pub async fn register_page(req: Request) -> Response {
        inertia(&req, "Auth/Register", json!({ "title": "Daftar Akun" }))
    }

    /// Proses Pendaftaran
    pub async fn register(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        // 1. Validasi Input
        let data = match req.validate::<RegisterRequest>() {
            Ok(d) => d,
            Err(_) => return Redirect::to("/register").into_response(),
        };

        // 2. Cek apakah email sudah terdaftar
        let existing = User::find()
            .filter(crate::app::models::users::Column::Email.eq(&data.email))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if existing.is_some() {
            req.session.set("error", "Email sudah terdaftar");
            return Redirect::to("/register").into_response();
        }

        // 3. Hash Password
        let hashed = hash(data.password, DEFAULT_COST).unwrap();

        // 4. Simpan ke Database
        let create_result = User::create(&state.db, rustbasic_core::serde_json::json!({
            "name": data.name,
            "email": data.email,
            "password": hashed,
        })).await;

        if let Err(e) = create_result {
            rustbasic_core::tracing::error!("Gagal menyimpan user: {}", e);
            req.session.set("error", "Gagal mendaftar, coba lagi.");
            return Redirect::to("/register").into_response();
        }

        req.session.set("success", "Pendaftaran berhasil! Silakan login.");
        Redirect::to("/login").into_response()
    }

    /// Proses Login
    pub async fn login(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        // 1. Validasi Input
        let data = match req.validate::<LoginRequest>() {
            Ok(d) => d,
            Err(_) => return Redirect::to("/login").into_response(),
        };

        // 2. Ambil User dari DB
        let user = User::find()
            .filter(crate::app::models::users::Column::Email.eq(&data.email))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if let Some(u) = user {
            // 3. Verifikasi Password
            if verify(data.password, &u.password).unwrap_or(false) {
                // 4. Set Session
                req.session.set("user_id", u.id);
                req.session.set("success", "Selamat datang kembali!");
                return Redirect::to("/dashboard").into_response();
            }
        }

        req.session.set("error", "Email atau password salah");
        Redirect::to("/login").into_response()
    }

    /// Menampilkan halaman lupa password
    pub async fn forgot_password_page(req: Request) -> Response {
        inertia(&req, "Auth/ForgotPassword", json!({ "title": "Lupa Password" }))
    }

    /// Kirim link reset password
    pub async fn send_reset_link(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        let data = match req.validate::<ForgotPasswordRequest>() {
            Ok(d) => d,
            Err(_) => return Redirect::to("/forgot-password").into_response(),
        };

        // 1. Cek apakah user ada
        let user = User::find()
            .filter(crate::app::models::users::Column::Email.eq(&data.email))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if let Some(u) = user {
            // 2. Generate Token
            let token = Uuid::new_v4().to_string();

            // 3. Simpan Token (Hapus token lama jika ada, lalu insert)
            let _ = PasswordReset::delete_by_id(&u.email).exec(&state.db).await;
            
            let _ = PasswordReset::create(&state.db, rustbasic_core::serde_json::json!({
                "email": u.email.clone(),
                "token": token.clone(),
                "created_at": rustbasic_core::chrono::Utc::now().naive_utc(),
            })).await;

            // 4. Kirim Email (Gunakan Config::load().mail_*)
            let config = rustbasic_core::Config::load();
            let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "RustBasic".to_string());
            let reset_url = format!("{}/reset-password?token={}", config.app_url, token);

            let subject = format!("Reset Password - {}", app_name);
            let body = rustbasic_core::view::render_to_string("emails/reset.rb.html", rustbasic_core::minijinja::context! {
                app_name => app_name,
                reset_url => reset_url,
            });

            if let Err(e) = MailService::send_email(&u.email, &subject, &body).await {
                rustbasic_core::tracing::error!("Gagal mengirim email reset: {}", e);
            }

            rustbasic_core::tracing::info!("Reset link for {}: {}", u.email, reset_url);
        }

        req.session.set("success", "Jika email terdaftar, link reset password akan dikirim.");
        Redirect::to("/login").into_response()
    }

    /// Menampilkan halaman reset password
    pub async fn reset_password_page(req: Request) -> Response {
        let token = req.input_as_str("token").unwrap_or_default();
        inertia(&req, "Auth/ResetPassword", json!({ "title": "Reset Password", "token": token }))
    }

    /// Proses update password baru
    pub async fn update_password(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        let data = match req.validate::<ResetPasswordRequest>() {
            Ok(d) => d,
            Err(_) => return Redirect::to("/login").into_response(),
        };

        // 1. Cari Token
        let reset = PasswordReset::find()
            .filter(crate::app::models::password_resets::Column::Token.eq(&data.token))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if let Some(r) = reset {
            // 2. Cek Kadaluarsa (60 Menit)
            let now = rustbasic_core::chrono::Utc::now().naive_utc();
            let duration = now.signed_duration_since(r.created_at);
            
            if duration.num_minutes() > 60 {
                // Hapus token yang sudah kadaluarsa
                let _ = PasswordReset::delete_by_id(r.email.clone())
                    .exec(&state.db)
                    .await;
                    
                req.session.set("error", "Tautan reset password sudah kadaluarsa (melebihi 60 menit).");
                return Redirect::to("/login").into_response();
            }

            // 3. Hash Password Baru
            let hashed = rustbasic_core::bcrypt::hash(data.password, rustbasic_core::bcrypt::DEFAULT_COST).unwrap();

            // 4. Update User
            let _ = User::update_many()
                .col_expr(crate::app::models::users::Column::Password, rustbasic_core::sea_orm::sea_query::Expr::value(hashed))
                .filter(crate::app::models::users::Column::Email.eq(&r.email))
                .exec(&state.db)
                .await;

            // 5. Hapus Token
            let _ = PasswordReset::delete_by_id(r.email)
                .exec(&state.db)
                .await;

            req.session.set("success", "Password berhasil diubah. Silakan login.");
            return Redirect::to("/login").into_response();
        }

        req.session.set("error", "Token tidak valid atau sudah kadaluarsa.");
        Redirect::to("/login").into_response()
    }

    /// Proses Logout
    pub async fn logout(req: Request) -> impl IntoResponse {
        req.session.remove("user_id");
        req.session.set("success", "Anda telah keluar.");
        Redirect::to("/").into_response()
    }
}
"#;
        fs::write(auth_controller_path, controller_template).ok();
        println!("   {} {}", "✅ Created:".green(), auth_controller_path.cyan());
    }

    // 5. Views (React Components in resources/js/Pages)
    let auth_page_dir = "src/resources/js/Pages/Auth";
    fs::create_dir_all(auth_page_dir).ok();

    // 5.0 Create Components dir & shared components
    let components_dir = "src/resources/js/Components";
    fs::create_dir_all(components_dir).ok();

    let toast_template = r##"import React, { useState, useEffect, useCallback } from 'react';

const ICONS = {
  success: (
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
      <circle cx="10" cy="10" r="10" fill="currentColor" opacity="0.15" />
      <path d="M6 10.5L8.5 13L14 7" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  ),
  error: (
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
      <circle cx="10" cy="10" r="10" fill="currentColor" opacity="0.15" />
      <path d="M7 7L13 13M13 7L7 13" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  ),
  warning: (
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
      <circle cx="10" cy="10" r="10" fill="currentColor" opacity="0.15" />
      <path d="M10 6V11" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <circle cx="10" cy="14" r="1" fill="currentColor" />
    </svg>
  ),
  info: (
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
      <circle cx="10" cy="10" r="10" fill="currentColor" opacity="0.15" />
      <path d="M10 9V14" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <circle cx="10" cy="6.5" r="1" fill="currentColor" />
    </svg>
  ),
};

const STYLES = {
  success: {
    bg: 'rgba(16, 185, 129, 0.08)',
    border: 'rgba(16, 185, 129, 0.25)',
    color: '#34d399',
    progress: '#10b981',
    shadow: '0 8px 32px rgba(16, 185, 129, 0.15)',
  },
  error: {
    bg: 'rgba(244, 63, 94, 0.08)',
    border: 'rgba(244, 63, 94, 0.25)',
    color: '#fb7185',
    progress: '#f43f5e',
    shadow: '0 8px 32px rgba(244, 63, 94, 0.15)',
  },
  warning: {
    bg: 'rgba(245, 158, 11, 0.08)',
    border: 'rgba(245, 158, 11, 0.25)',
    color: '#fbbf24',
    progress: '#f59e0b',
    shadow: '0 8px 32px rgba(245, 158, 11, 0.15)',
  },
  info: {
    bg: 'rgba(99, 102, 241, 0.08)',
    border: 'rgba(99, 102, 241, 0.25)',
    color: '#818cf8',
    progress: '#6366f1',
    shadow: '0 8px 32px rgba(99, 102, 241, 0.15)',
  },
};

function SingleToast({ id, type, message, duration = 5000, onDismiss }) {
  const [isVisible, setIsVisible] = useState(false);
  const [isLeaving, setIsLeaving] = useState(false);
  const [progress, setProgress] = useState(100);

  const style = STYLES[type] || STYLES.info;

  const dismiss = useCallback(() => {
    setIsLeaving(true);
    setTimeout(() => onDismiss(id), 350);
  }, [id, onDismiss]);

  useEffect(() => {
    const enterTimer = setTimeout(() => setIsVisible(true), 10);
    const dismissTimer = setTimeout(() => dismiss(), duration);
    const startTime = Date.now();
    const progressInterval = setInterval(() => {
      const elapsed = Date.now() - startTime;
      const remaining = Math.max(0, 100 - (elapsed / duration) * 100);
      setProgress(remaining);
      if (remaining <= 0) clearInterval(progressInterval);
    }, 30);

    return () => {
      clearTimeout(enterTimer);
      clearTimeout(dismissTimer);
      clearInterval(progressInterval);
    };
  }, [duration, dismiss]);

  return (
    <div
      style={{
        background: style.bg,
        border: `1px solid ${style.border}`,
        borderRadius: '16px',
        padding: '14px 18px',
        marginBottom: '10px',
        display: 'flex',
        alignItems: 'flex-start',
        gap: '12px',
        color: style.color,
        backdropFilter: 'blur(20px)',
        boxShadow: style.shadow,
        transform: isVisible && !isLeaving
          ? 'translateX(0) scale(1)'
          : isLeaving
            ? 'translateX(120%) scale(0.9)'
            : 'translateX(120%) scale(0.9)',
        opacity: isVisible && !isLeaving ? 1 : 0,
        transition: 'all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1)',
        position: 'relative',
        overflow: 'hidden',
        minWidth: '320px',
        maxWidth: '420px',
        cursor: 'pointer',
      }}
      onClick={dismiss}
      role="alert"
    >
      <div style={{ flexShrink: 0, marginTop: '1px' }}>{ICONS[type] || ICONS.info}</div>
      <div style={{ flex: 1, minWidth: 0 }}>
        <div style={{
          fontSize: '11px',
          fontWeight: 700,
          textTransform: 'uppercase',
          letterSpacing: '0.08em',
          opacity: 0.7,
          marginBottom: '2px',
        }}>
          {type === 'success' && 'Berhasil'}
          {type === 'error' && 'Kesalahan'}
          {type === 'warning' && 'Peringatan'}
          {type === 'info' && 'Informasi'}
        </div>
        <div style={{
          fontSize: '13px',
          fontWeight: 500,
          color: '#e2e8f0',
          lineHeight: 1.5,
          wordBreak: 'break-word',
        }}>{message}</div>
      </div>
      <button
        onClick={(e) => { e.stopPropagation(); dismiss(); }}
        style={{
          background: 'none',
          border: 'none',
          color: style.color,
          cursor: 'pointer',
          padding: '2px',
          opacity: 0.5,
          transition: 'opacity 0.2s',
          flexShrink: 0,
          marginTop: '1px',
        }}
      >
        ✕
      </button>
      <div style={{
        position: 'absolute',
        bottom: 0,
        left: 0,
        right: 0,
        height: '3px',
        background: `${style.progress}15`,
        borderRadius: '0 0 16px 16px',
        overflow: 'hidden',
      }}>
        <div style={{
          width: `${progress}%`,
          height: '100%',
          background: `linear-gradient(90deg, ${style.progress}, ${style.progress}aa)`,
          transition: 'width 0.1s linear',
          borderRadius: '0 0 16px 16px',
        }} />
      </div>
    </div>
  );
}

export default function Toast({ flash, duration = 5000, position = 'top-right' }) {
  const [toasts, setToasts] = useState([]);

  useEffect(() => {
    if (!flash) return;
    const newToasts = [];
    if (flash.success) newToasts.push({ id: Date.now() + '_s', type: 'success', message: flash.success });
    if (flash.error) newToasts.push({ id: Date.now() + '_e', type: 'error', message: flash.error });
    if (flash.warning) newToasts.push({ id: Date.now() + '_w', type: 'warning', message: flash.warning });
    if (flash.info) newToasts.push({ id: Date.now() + '_i', type: 'info', message: flash.info });

    if (newToasts.length > 0) {
      setToasts(prev => [...prev, ...newToasts]);
    }
  }, [flash?.success, flash?.error, flash?.warning, flash?.info]);

  const handleDismiss = useCallback((id) => {
    setToasts(prev => prev.filter(t => t.id !== id));
  }, []);

  const positionStyle = {
    'top-right': { top: '24px', right: '24px' },
    'top-left': { top: '24px', left: '24px' },
    'bottom-right': { bottom: '24px', right: '24px' },
    'bottom-left': { bottom: '24px', left: '24px' },
  };

  if (toasts.length === 0) return null;

  return (
    <div style={{ position: 'fixed', zIndex: 99999, pointerEvents: 'none', ...positionStyle[position] }}>
      <div style={{ pointerEvents: 'auto' }}>
        {toasts.map(toast => (
          <SingleToast
            key={toast.id}
            id={toast.id}
            type={toast.type}
            message={toast.message}
            duration={duration}
            onDismiss={handleDismiss}
          />
        ))}
      </div>
    </div>
  );
}
"##;

    let alert_banner_template = r##"import React from 'react';

const ALERT_STYLES = {
  success: {
    bg: 'rgba(16, 185, 129, 0.08)',
    border: 'rgba(16, 185, 129, 0.2)',
    color: '#34d399',
    icon: '✅',
  },
  error: {
    bg: 'rgba(244, 63, 94, 0.08)',
    border: 'rgba(244, 63, 94, 0.2)',
    color: '#fb7185',
    icon: '❌',
  },
  warning: {
    bg: 'rgba(245, 158, 11, 0.08)',
    border: 'rgba(245, 158, 11, 0.2)',
    color: '#fbbf24',
    icon: '⚠️',
  },
  info: {
    bg: 'rgba(99, 102, 241, 0.08)',
    border: 'rgba(99, 102, 241, 0.2)',
    color: '#818cf8',
    icon: 'ℹ️',
  },
};

export default function AlertBanner({ type = 'info', message, onDismiss }) {
  if (!message) return null;
  const style = ALERT_STYLES[type] || ALERT_STYLES.info;

  return (
    <div
      role="alert"
      style={{
        background: style.bg,
        border: `1px solid ${style.border}`,
        borderRadius: '14px',
        padding: '14px 18px',
        marginBottom: '20px',
        display: 'flex',
        alignItems: 'center',
        gap: '10px',
        animation: 'alertSlideIn 0.4s cubic-bezier(0.34, 1.56, 0.64, 1)',
      }}
    >
      <span style={{ fontSize: '16px', flexShrink: 0 }}>{style.icon}</span>
      <span style={{ flex: 1, fontSize: '13px', fontWeight: 600, color: style.color, lineHeight: 1.5 }}>{message}</span>
      {onDismiss && (
        <button
          onClick={onDismiss}
          style={{
            background: 'none',
            border: 'none',
            color: style.color,
            cursor: 'pointer',
            padding: '2px 4px',
            opacity: 0.6,
            transition: 'opacity 0.2s',
            fontSize: '14px',
          }}
        >
          ✕
        </button>
      )}
      <style>{`
        @keyframes alertSlideIn {
          from { opacity: 0; transform: translateY(-8px) scale(0.97); }
          to { opacity: 1; transform: translateY(0) scale(1); }
        }
      `}</style>
    </div>
  );
}
"##;

    let form_input_template = r##"import React from 'react';

export default function FormInput({
  label,
  type = 'text',
  value,
  onChange,
  error,
  placeholder,
  required = false,
  autoFocus = false,
  disabled = false,
}) {
  const hasError = !!error;

  return (
    <div>
      {label && (
        <label style={{
          display: 'block',
          fontSize: '11px',
          fontWeight: 700,
          textTransform: 'uppercase',
          letterSpacing: '0.08em',
          marginBottom: '8px',
          color: hasError ? '#fb7185' : '#94a3b8',
          transition: 'color 0.3s ease',
        }}>{label}</label>
      )}
      <input
        type={type}
        value={value}
        onChange={onChange}
        placeholder={placeholder}
        required={required}
        autoFocus={autoFocus}
        disabled={disabled}
        style={{
          width: '100%',
          boxSizing: 'border-box',
          background: 'rgba(2, 6, 23, 0.8)',
          border: `1px solid ${hasError ? 'rgba(244, 63, 94, 0.5)' : 'rgba(30, 41, 59, 1)'}`,
          borderRadius: '12px',
          padding: '12px 14px',
          fontSize: '14px',
          color: '#ffffff',
          outline: 'none',
          transition: 'all 0.3s ease',
          opacity: disabled ? 0.5 : 1,
        }}
        onFocus={(e) => {
          e.target.style.borderColor = hasError ? 'rgba(244, 63, 94, 0.7)' : 'rgba(99, 102, 241, 0.5)';
          e.target.style.boxShadow = hasError ? '0 0 0 3px rgba(244, 63, 94, 0.1)' : '0 0 0 3px rgba(99, 102, 241, 0.1)';
        }}
        onBlur={(e) => {
          e.target.style.borderColor = hasError ? 'rgba(244, 63, 94, 0.5)' : 'rgba(30, 41, 59, 1)';
          e.target.style.boxShadow = 'none';
        }}
      />
      {hasError && (
        <div style={{
          display: 'flex',
          alignItems: 'center',
          gap: '5px',
          marginTop: '6px',
          animation: 'errorShake 0.4s cubic-bezier(0.36, 0.07, 0.19, 0.97)',
        }}>
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" style={{ flexShrink: 0 }}>
            <circle cx="6" cy="6" r="6" fill="rgba(244, 63, 94, 0.15)" />
            <path d="M6 3.5V6.5" stroke="#fb7185" strokeWidth="1.2" strokeLinecap="round" />
            <circle cx="6" cy="8.2" r="0.6" fill="#fb7185" />
          </svg>
          <span style={{ fontSize: '12px', fontWeight: 600, color: '#fb7185', lineHeight: 1.3 }}>{error}</span>
        </div>
      )}
      <style>{`
        @keyframes errorShake {
          0%, 100% { transform: translateX(0); }
          20% { transform: translateX(-4px); }
          40% { transform: translateX(4px); }
          60% { transform: translateX(-2px); }
          80% { transform: translateX(2px); }
        }
      `}</style>
    </div>
  );
}
"##;

    let login_template = r##"import React from 'react';
import { Link, useForm, usePage } from '@inertiajs/react';
import Toast from '../../Components/Toast';
import AlertBanner from '../../Components/AlertBanner';
import FormInput from '../../Components/FormInput';

export default function Login() {
  const { flash } = usePage().props;
  const { data, setData, post, processing, errors } = useForm({
    email: '',
    password: '',
    remember: false,
  });

  const handleSubmit = (e) => {
    e.preventDefault();
    post('/login');
  };

  return (
    <div className="min-h-screen bg-gradient-to-tr from-slate-950 via-slate-900 to-indigo-950 flex items-center justify-center p-6 text-slate-100 font-sans">
      <Toast flash={flash} />

      <div className="w-full max-w-md bg-slate-900/60 backdrop-blur-md border border-slate-800/80 rounded-3xl p-8 shadow-2xl relative overflow-hidden glassmorphism">
        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/10 rounded-full blur-3xl pointer-events-none" />
        <div className="absolute bottom-0 left-0 w-32 h-32 bg-purple-500/10 rounded-full blur-3xl pointer-events-none" />
        
        <div className="text-center mb-8">
          <span className="text-xs font-bold tracking-widest text-indigo-400 bg-indigo-500/10 px-3 py-1 rounded-full border border-indigo-500/20 uppercase">
            RustBasic SPA
          </span>
          <h1 className="text-3xl font-extrabold text-white mt-4 tracking-tight">Selamat Datang</h1>
          <p className="text-slate-400 text-sm mt-2">Silakan masuk ke akun Anda</p>
        </div>

        {flash?.success && <AlertBanner type="success" message={flash.success} />}
        {flash?.error && <AlertBanner type="error" message={flash.error} />}

        <form onSubmit={handleSubmit} className="space-y-5">
          <FormInput
            label="Email Address"
            type="email"
            value={data.email}
            onChange={(e) => setData('email', e.target.value)}
            error={errors.email}
            placeholder="nama@email.com"
            required
          />

          <FormInput
            label="Password"
            type="password"
            value={data.password}
            onChange={(e) => setData('password', e.target.value)}
            error={errors.password}
            placeholder="••••••••"
            required
          />

          <div className="flex items-center justify-between text-sm">
            <label className="flex items-center space-x-2 text-slate-400 cursor-pointer">
              <input
                type="checkbox"
                checked={data.remember}
                onChange={(e) => setData('remember', e.target.checked)}
                className="w-4 h-4 rounded border-slate-800 bg-slate-950 text-indigo-600 focus:ring-indigo-500 focus:ring-opacity-25"
              />
              <span className="select-none">Ingat Saya</span>
            </label>
            <Link href="/forgot-password" className="text-indigo-400 hover:text-indigo-300 font-semibold transition-colors duration-200" style={{ textDecoration: 'none' }}>
              Lupa Password?
            </Link>
          </div>

          <button
            type="submit"
            disabled={processing}
            className="w-full py-3.5 px-4 bg-gradient-to-r from-indigo-600 to-indigo-700 hover:from-indigo-500 hover:to-indigo-600 text-white rounded-xl font-bold tracking-wide shadow-[0_0_20px_rgba(99,102,241,0.25)] hover:shadow-[0_0_25px_rgba(99,102,241,0.4)] disabled:opacity-50 transition-all duration-300 transform active:scale-[0.98]"
          >
            {processing ? 'MEMROSES...' : 'MASUK KE DASHBOARD'}
          </button>
        </form>

        <p className="text-center text-sm text-slate-500 mt-8">
          Belum punya akun?{' '}
          <Link href="/register" className="text-indigo-400 hover:underline font-bold transition-colors duration-200" style={{ textDecoration: 'none' }}>
            Daftar Sekarang
          </Link>
        </p>
      </div>
    </div>
  );
}
"##;

    let register_template = r##"import React from 'react';
import { Link, useForm, usePage } from '@inertiajs/react';
import Toast from '../../Components/Toast';
import AlertBanner from '../../Components/AlertBanner';
import FormInput from '../../Components/FormInput';

export default function Register() {
  const { flash } = usePage().props;
  const { data, setData, post, processing, errors } = useForm({
    name: '',
    email: '',
    password: '',
  });

  const handleSubmit = (e) => {
    e.preventDefault();
    post('/register');
  };

  return (
    <div className="min-h-screen bg-gradient-to-tr from-slate-950 via-slate-900 to-indigo-950 flex items-center justify-center p-6 text-slate-100 font-sans">
      <Toast flash={flash} />

      <div className="w-full max-w-md bg-slate-900/60 backdrop-blur-md border border-slate-800/80 rounded-3xl p-8 shadow-2xl relative overflow-hidden glassmorphism">
        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/10 rounded-full blur-3xl pointer-events-none" />
        <div className="absolute bottom-0 left-0 w-32 h-32 bg-purple-500/10 rounded-full blur-3xl pointer-events-none" />
        
        <div className="text-center mb-8">
          <span className="text-xs font-bold tracking-widest text-indigo-400 bg-indigo-500/10 px-3 py-1 rounded-full border border-indigo-500/20 uppercase">
            RustBasic SPA
          </span>
          <h1 className="text-3xl font-extrabold text-white mt-4 tracking-tight">Daftar Akun</h1>
          <p className="text-slate-400 text-sm mt-2">Mulai perjalanan Anda bersama kami</p>
        </div>

        {flash?.error && <AlertBanner type="error" message={flash.error} />}
        {flash?.success && <AlertBanner type="success" message={flash.success} />}

        <form onSubmit={handleSubmit} className="space-y-5">
          <FormInput
            label="Nama Lengkap"
            type="text"
            value={data.name}
            onChange={(e) => setData('name', e.target.value)}
            error={errors.name}
            placeholder="Nama Lengkap Anda"
            required
          />

          <FormInput
            label="Email Address"
            type="email"
            value={data.email}
            onChange={(e) => setData('email', e.target.value)}
            error={errors.email}
            placeholder="nama@email.com"
            required
          />

          <FormInput
            label="Password"
            type="password"
            value={data.password}
            onChange={(e) => setData('password', e.target.value)}
            error={errors.password}
            placeholder="Min. 8 karakter"
            required
          />

          <button
            type="submit"
            disabled={processing}
            className="w-full py-3.5 px-4 bg-gradient-to-r from-indigo-600 to-indigo-700 hover:from-indigo-500 hover:to-indigo-600 text-white rounded-xl font-bold tracking-wide shadow-[0_0_20px_rgba(99,102,241,0.25)] hover:shadow-[0_0_25px_rgba(99,102,241,0.4)] disabled:opacity-50 transition-all duration-300 transform active:scale-[0.98]"
          >
            {processing ? 'MENDAFTAR...' : 'BUAT AKUN SEKARANG'}
          </button>
        </form>

        <p className="text-center text-sm text-slate-500 mt-8">
          Sudah punya akun?{' '}
          <Link href="/login" className="text-indigo-400 hover:underline font-bold transition-colors duration-200" style={{ textDecoration: 'none' }}>
            Login Disini
          </Link>
        </p>
      </div>
    </div>
  );
}
"##;

    let forgot_template = r##"import React from 'react';
import { Link, useForm, usePage } from '@inertiajs/react';
import Toast from '../../Components/Toast';
import AlertBanner from '../../Components/AlertBanner';
import FormInput from '../../Components/FormInput';

export default function ForgotPassword() {
  const { flash } = usePage().props;
  const { data, setData, post, processing, errors } = useForm({
    email: '',
  });

  const handleSubmit = (e) => {
    e.preventDefault();
    post('/forgot-password');
  };

  return (
    <div className="min-h-screen bg-gradient-to-tr from-slate-950 via-slate-900 to-indigo-950 flex items-center justify-center p-6 text-slate-100 font-sans">
      <Toast flash={flash} />

      <div className="w-full max-w-md bg-slate-900/60 backdrop-blur-md border border-slate-800/80 rounded-3xl p-8 shadow-2xl relative overflow-hidden glassmorphism">
        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/10 rounded-full blur-3xl pointer-events-none" />
        <div className="absolute bottom-0 left-0 w-32 h-32 bg-purple-500/10 rounded-full blur-3xl pointer-events-none" />
        
        <div className="text-center mb-8">
          <span className="text-xs font-bold tracking-widest text-indigo-400 bg-indigo-500/10 px-3 py-1 rounded-full border border-indigo-500/20 uppercase">
            Keamanan Akun
          </span>
          <h1 className="text-3xl font-extrabold text-white mt-4 tracking-tight">Lupa Password</h1>
          <p className="text-slate-400 text-sm mt-2">Kami akan mengirimkan instruksi ke email Anda</p>
        </div>

        {flash?.success && <AlertBanner type="success" message={flash.success} />}
        {flash?.error && <AlertBanner type="error" message={flash.error} />}

        <form onSubmit={handleSubmit} className="space-y-5">
          <FormInput
            label="Email Address"
            type="email"
            value={data.email}
            onChange={(e) => setData('email', e.target.value)}
            error={errors.email}
            placeholder="nama@email.com"
            required
            autoFocus
          />

          <button
            type="submit"
            disabled={processing}
            className="w-full py-3.5 px-4 bg-gradient-to-r from-indigo-600 to-indigo-700 hover:from-indigo-500 hover:to-indigo-600 text-white rounded-xl font-bold tracking-wide shadow-[0_0_20px_rgba(99,102,241,0.25)] hover:shadow-[0_0_25px_rgba(99,102,241,0.4)] disabled:opacity-50 transition-all duration-300 transform active:scale-[0.98]"
          >
            {processing ? 'MENGIRIM...' : 'KIRIM LINK RESET PASSWORD'}
          </button>
        </form>

        <p className="text-center text-sm text-slate-500 mt-8">
          Ingat password Anda?{' '}
          <Link href="/login" className="text-indigo-400 hover:underline font-bold transition-colors duration-200" style={{ textDecoration: 'none' }}>
            Login Disini
          </Link>
        </p>
      </div>
    </div>
  );
}
"##;

    let toast_view = "src/resources/js/Components/Toast.jsx";
    if !std::path::Path::new(toast_view).exists() {
        fs::write(toast_view, toast_template).ok();
    }

    let alert_banner_view = "src/resources/js/Components/AlertBanner.jsx";
    if !std::path::Path::new(alert_banner_view).exists() {
        fs::write(alert_banner_view, alert_banner_template).ok();
    }

    let form_input_view = "src/resources/js/Components/FormInput.jsx";
    if !std::path::Path::new(form_input_view).exists() {
        fs::write(form_input_view, form_input_template).ok();
    }

    let login_view = "src/resources/js/Pages/Auth/Login.jsx";
    if !std::path::Path::new(login_view).exists() {
        fs::write(login_view, login_template).ok();
    }
    
    let register_view = "src/resources/js/Pages/Auth/Register.jsx";
    if !std::path::Path::new(register_view).exists() {
        fs::write(register_view, register_template).ok();
    }

    let forgot_view = "src/resources/js/Pages/Auth/ForgotPassword.jsx";
    if !std::path::Path::new(forgot_view).exists() {
        fs::write(forgot_view, forgot_template).ok();
    }

    let reset_view = "src/resources/js/Pages/Auth/ResetPassword.jsx";
    if !std::path::Path::new(reset_view).exists() {
        let reset_template = r##"import React from 'react';
import { useForm, usePage } from '@inertiajs/react';
import Toast from '../../Components/Toast';
import AlertBanner from '../../Components/AlertBanner';
import FormInput from '../../Components/FormInput';

export default function ResetPassword({ token }) {
  const { flash } = usePage().props;
  const { data, setData, post, processing, errors } = useForm({
    token: token || '',
    password: '',
  });

  const handleSubmit = (e) => {
    e.preventDefault();
    post('/reset-password');
  };

  return (
    <div className="min-h-screen bg-gradient-to-tr from-slate-950 via-slate-900 to-indigo-950 flex items-center justify-center p-6 text-slate-100 font-sans">
      <Toast flash={flash} />

      <div className="w-full max-w-md bg-slate-900/60 backdrop-blur-md border border-slate-800/80 rounded-3xl p-8 shadow-2xl relative overflow-hidden glassmorphism">
        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/10 rounded-full blur-3xl pointer-events-none" />
        <div className="absolute bottom-0 left-0 w-32 h-32 bg-purple-500/10 rounded-full blur-3xl pointer-events-none" />
        
        <div className="text-center mb-8">
          <span className="text-xs font-bold tracking-widest text-indigo-400 bg-indigo-500/10 px-3 py-1 rounded-full border border-indigo-500/20 uppercase">
            Akses Akun
          </span>
          <h1 className="text-3xl font-extrabold text-white mt-4 tracking-tight">Reset Password</h1>
          <p className="text-slate-400 text-sm mt-2">Silakan masukkan password baru Anda</p>
        </div>

        {flash?.error && <AlertBanner type="error" message={flash.error} />}

        <form onSubmit={handleSubmit} className="space-y-5">
          <input type="hidden" value={data.token} />

          <FormInput
            label="Password Baru"
            type="password"
            value={data.password}
            onChange={(e) => setData('password', e.target.value)}
            error={errors.password}
            placeholder="Minimal 8 karakter"
            required
            autoFocus
          />

          <button
            type="submit"
            disabled={processing}
            className="w-full py-3.5 px-4 bg-gradient-to-r from-indigo-600 to-indigo-700 hover:from-indigo-500 hover:to-indigo-600 text-white rounded-xl font-bold tracking-wide shadow-[0_0_20px_rgba(99,102,241,0.25)] hover:shadow-[0_0_25px_rgba(99,102,241,0.4)] disabled:opacity-50 transition-all duration-300 transform active:scale-[0.98]"
          >
            {processing ? 'MENYIMPAN...' : 'SIMPAN PASSWORD BARU'}
          </button>
        </form>
      </div>
    </div>
  );
}
"##;
        fs::write(reset_view, reset_template).ok();
    }

    // 5.1 Create Email Reset Template (Minijinja string processing is still perfect here)
    let email_reset_view = "src/resources/views/emails/reset.rb.html";
    if !std::path::Path::new(email_reset_view).exists() {
        fs::create_dir_all("src/resources/views/emails").ok();
        let email_reset_template = r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <style>
        body { font-family: 'Inter', -apple-system, sans-serif; line-height: 1.6; color: #1a1a1a; margin: 0; padding: 0; }
        .container { max-width: 600px; margin: 0 auto; padding: 40px 20px; }
        .card { background: #ffffff; border-radius: 16px; overflow: hidden; box-shadow: 0 4px 24px rgba(0,0,0,0.06); border: 1px solid #f0f0f0; }
        .header { background: linear-gradient(135deg, #6366f1, #a855f7); padding: 40px; text-align: center; color: white; }
        .content { padding: 40px; }
        .button { display: inline-block; padding: 14px 32px; background: #6366f1; color: #ffffff !important; text-decoration: none; border-radius: 8px; font-weight: 600; margin: 24px 0; }
        .footer { padding: 24px; text-align: center; font-size: 13px; color: #6b7280; }
        h1 { margin: 0; font-size: 24px; font-weight: 800; letter-spacing: -0.025em; }
        p { margin: 16px 0; color: #4b5563; }
        .divider { height: 1px; background: #f3f4f6; margin: 24px 0; }
    </style>
</head>
<body>
    <div class="container">
        <div class="card">
            <div class="header">
                <h1>{{ app_name }}</h1>
            </div>
            <div class="content">
                <h2 style="margin: 0; color: #111827; font-size: 20px;">Halo!</h2>
                <p>Anda menerima email ini karena kami menerima permintaan reset password untuk akun Anda di <strong>{{ app_name }}</strong>.</p>
                
                <div style="text-align: center;">
                    <a href="{{ reset_url }}" class="button">Reset Password Saya</a>
                </div>

                <p style="font-size: 14px; color: #9ca3af;">Link ini akan kadaluarsa dalam 60 menit. Jika Anda tidak merasa meminta reset password, abaikan saja email ini.</p>
                
                <div class="divider"></div>
                
                <p style="font-size: 12px; color: #9ca3af;">
                    Jika Anda kesulitan menekan tombol, salin dan tempel URL berikut ke browser Anda:<br>
                    <span style="word-break: break-all; color: #6366f1;">{{ reset_url }}</span>
                </p>
            </div>
        </div>
        <div class="footer">
            &copy; 2026 {{ app_name }}. All rights reserved.
        </div>
    </div>
</body>
</html>
"##;
        fs::write(email_reset_view, email_reset_template).ok();
    }

    // 5.2 Create Dashboard Page in React
    let dashboard_view = "src/resources/js/Pages/Dashboard.jsx";
    if !std::path::Path::new(dashboard_view).exists() {
        let dashboard_template = r##"import React from 'react';
import { Link, router, usePage } from '@inertiajs/react';
import Toast from '../Components/Toast';

export default function Dashboard({ title, userName, userEmail, totalUsers }) {
  const { flash } = usePage().props;

  const handleLogout = (e) => {
    e.preventDefault();
    router.post('/logout');
  };

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col md:flex-row font-sans">
      {/* Sidebar */}
      <aside className="w-full md:w-80 bg-slate-900 border-b md:border-b-0 md:border-r border-slate-800/80 p-6 flex flex-col justify-between relative overflow-hidden">
        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/5 rounded-full blur-3xl pointer-events-none" />
        
        <div>
          {/* Logo */}
          <div className="flex items-center space-x-3 mb-10">
            <div className="w-10 h-10 bg-indigo-600 rounded-xl flex items-center justify-center font-extrabold text-white text-lg shadow-lg shadow-indigo-600/30">
              R
            </div>
            <span className="text-xl font-extrabold text-white tracking-tight">RustBasic</span>
          </div>

          {/* User Profile Info Card */}
          <div className="bg-slate-950/60 border border-slate-800/50 rounded-2xl p-4 mb-8">
            <div className="flex items-center space-x-3">
              <div className="w-12 h-12 bg-gradient-to-tr from-indigo-500 to-purple-500 rounded-full flex items-center justify-center font-extrabold text-white text-lg">
                {userName ? userName[0].toUpperCase() : 'G'}
              </div>
              <div className="overflow-hidden">
                <h4 className="text-sm font-bold text-white truncate">{userName || 'Administrator'}</h4>
                <p className="text-xs text-slate-500 truncate">{userEmail || 'admin@rustbasic.dev'}</p>
              </div>
            </div>
          </div>

          {/* Navigation links */}
          <nav className="space-y-2">
            <Link
              href="/dashboard"
              className="flex items-center space-x-3 w-full px-4 py-3 bg-indigo-600 text-white rounded-xl font-bold text-sm shadow-lg shadow-indigo-600/10 transition-all duration-300"
              style={{ textDecoration: 'none' }}
            >
              <span>📊</span>
              <span>Dashboard Overview</span>
            </Link>
            <Link
              href="/"
              className="flex items-center space-x-3 w-full px-4 py-3 text-slate-400 hover:text-white rounded-xl font-semibold text-sm hover:bg-slate-800/30 transition-all duration-300"
              style={{ textDecoration: 'none' }}
            >
              <span>🏠</span>
              <span>Main Website</span>
            </Link>
          </nav>
        </div>

        {/* Logout Form / Button */}
        <div className="mt-8 md:mt-0">
          <form onSubmit={handleLogout}>
            <button
              type="submit"
              className="w-full py-3 px-4 bg-rose-500/10 hover:bg-rose-500/20 border border-rose-500/20 text-rose-400 rounded-xl font-bold text-sm transition-all duration-300 flex items-center justify-center space-x-2"
            >
              <span>🚪</span>
              <span>KELUAR SISTEM</span>
            </button>
          </form>
        </div>
      </aside>

      {/* Main Workspace */}
      <main className="flex-1 p-6 md:p-12 overflow-y-auto">
        <div className="max-w-6xl mx-auto">
          {/* Header */}
          <header className="flex flex-col md:flex-row md:items-center md:justify-between mb-10 gap-4">
            <div>
              <h1 className="text-3xl font-extrabold text-white tracking-tight">{title || 'Overview'}</h1>
              <p className="text-slate-400 text-sm mt-1">Selamat datang kembali, kendalikan project Anda secara instan.</p>
            </div>
            <div>
              <span className="inline-flex items-center px-4 py-2 bg-slate-900 border border-slate-800 rounded-xl text-xs font-bold text-slate-300 shadow-sm">
                <span className="w-2.5 h-2.5 bg-emerald-500 rounded-full mr-2 animate-ping" />
                Server Status: <span className="text-emerald-400 ml-1">Running</span>
              </span>
            </div>
          </header>

          {/* Toast Notifications */}
          <Toast flash={flash} />

          {/* Stats Grid */}
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mb-10">
            {/* Stat 1 */}
            <div className="bg-slate-900/60 border border-slate-800/80 rounded-3xl p-6 relative overflow-hidden glassmorphism">
              <span className="text-xs font-bold text-slate-500 uppercase tracking-widest block mb-4">
                User Terdaftar
              </span>
              <div className="flex items-baseline space-x-2">
                <span className="text-5xl font-black text-white tracking-tight">{totalUsers || 0}</span>
                <span className="text-emerald-400 text-sm font-bold">↑ 12%</span>
              </div>
            </div>

            {/* Stat 2 */}
            <div className="bg-slate-900/60 border border-slate-800/80 rounded-3xl p-6 relative overflow-hidden glassmorphism">
              <span className="text-xs font-bold text-slate-500 uppercase tracking-widest block mb-4">
                Response Time
              </span>
              <div className="flex items-baseline space-x-1">
                <span className="text-5xl font-black text-indigo-400 tracking-tight">24</span>
                <span className="text-slate-400 text-lg font-bold">ms</span>
              </div>
            </div>

            {/* Stat 3 */}
            <div className="bg-slate-900/60 border border-slate-800/80 rounded-3xl p-6 relative overflow-hidden glassmorphism">
              <span className="text-xs font-bold text-slate-500 uppercase tracking-widest block mb-4">
                Database Status
              </span>
              <div className="flex items-center space-x-3 mt-2">
                <div className="w-3 h-3 bg-emerald-500 rounded-full shadow-[0_0_12px_#10b981]" />
                <span className="text-xl font-extrabold text-emerald-400 tracking-wide uppercase">HEALTHY</span>
              </div>
            </div>
          </div>

          {/* Main Info Panel */}
          <div className="bg-slate-900/40 border border-slate-800/60 rounded-3xl p-8 glassmorphism">
            <div className="flex items-center justify-between mb-6">
              <div>
                <h3 className="text-lg font-bold text-white">Informasi Kernel Server</h3>
                <p className="text-xs text-slate-400 mt-0.5">Detail lingkungan runtime eksekusi Axum Anda.</p>
              </div>
              <span className="text-[10px] font-bold text-indigo-400 bg-indigo-500/10 border border-indigo-500/20 px-3 py-1 rounded-full uppercase tracking-wider">
                v2026.1
              </span>
            </div>

            <div className="bg-slate-950 border border-slate-800/50 rounded-2xl p-6 font-mono text-xs text-emerald-400 leading-relaxed shadow-inner">
              <div className="text-slate-600 mb-2">// RustBasic SPA Kernel Logs</div>
              <div>[OK] Compiled with Axum 0.8.2</div>
              <div>[OK] Database Pool: Sea-ORM Connection Established</div>
              <div>[OK] Modern SPA Routing: Powered by Inertia.js Bridge</div>
              <div>[OK] Single-Binary Mode: Compile-time embedding enabled</div>
              <div>[OK] Workers: 8 logical threads spawned on CPU cores</div>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
"##;
        fs::write(dashboard_view, dashboard_template).ok();
    }
    
    // 6. Create Dashboard Controller in Rust
    let dashboard_controller_path = "src/app/http/controllers/dashboard_controller.rs";
    if !std::path::Path::new(dashboard_controller_path).exists() {
        let dashboard_template = r#"use crate::app::inertia::inertia;
use crate::app::models::users;
use rustbasic_core::requests::Request;
use rustbasic_core::server::AppState;
use rustbasic_core::axum::{response::Response, extract::State};
use rustbasic_core::sea_orm::{EntityTrait, PaginatorTrait};
use rustbasic_core::serde_json::json;

pub struct DashboardController;

impl DashboardController {
    pub async fn index(State(state): State<AppState>, req: Request) -> Response {
        let user_id = req.session.get::<i32>("user_id").unwrap_or(0);
        let user = users::Entity::find_by_id(user_id).one(&state.db).await.ok().flatten();
        let total_users = users::Entity::find().count(&state.db).await.unwrap_or(0);

        inertia(&req, "Dashboard", json!({
            "title": "Dashboard",
            "userName": user.as_ref().map(|u| u.name.clone()).unwrap_or("Guest".to_string()),
            "userEmail": user.as_ref().map(|u| u.email.clone()).unwrap_or_default(),
            "totalUsers": total_users,
        }))
    }
}
"#;
        fs::write(dashboard_controller_path, dashboard_template).ok();
        println!("   {} {}", "✅ Created:".green(), dashboard_controller_path.cyan());
    }
    update_controller_mod_rs("dashboard_controller");

    println!("   {} Folder src/resources/js/Pages/Auth dan Dashboard siap.", "✅ Views:".green());

    // 7. Update Welcome.jsx
    let welcome_path = "src/resources/js/Pages/Welcome.jsx";
    if let Ok(content) = fs::read_to_string(welcome_path)
        && content.contains("Backend Online") && !content.contains("auth_installed ?") {
            let target = r#"          <div className="flex items-center gap-4">
            <span className="inline-flex items-center gap-1.5 px-3 h-8 rounded-full text-xs font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
              <span className="w-2 h-2 rounded-full bg-emerald-400" style={{ boxShadow: "0 0 10px #34d399" }} />
              Backend Online
            </span>
          </div>"#;

            let replacement = r#"          <div className="flex items-center gap-4">
            <span className="inline-flex items-center gap-1.5 px-3 h-8 rounded-full text-xs font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 mr-2">
              <span className="w-2 h-2 rounded-full bg-emerald-400" style={{ boxShadow: "0 0 10px #34d399" }} />
              Backend Online
            </span>
            {auth_installed ? (
              <Link 
                href="/dashboard" 
                className="px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-sm font-bold text-white transition-all duration-300"
                style={{ textDecoration: 'none' }}
              >
                Dashboard
              </Link>
            ) : (
              <div className="flex gap-2">
                <Link 
                  href="/login" 
                  className="px-4 py-2 rounded-lg border border-white/10 text-sm font-bold hover:bg-white/5 transition-all duration-300 text-gray-300 hover:text-white"
                  style={{ textDecoration: 'none' }}
                >
                  Masuk
                </Link>
                <Link 
                  href="/register" 
                  className="px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-sm font-bold text-white transition-all duration-300"
                  style={{ textDecoration: 'none' }}
                >
                  Daftar
                </Link>
              </div>
            )}
          </div>"#;

            let updated = content.replace(target, replacement);
            fs::write(welcome_path, updated).ok();
            println!("   {} {}", "📝 Updated:".blue(), welcome_path.cyan());
        }

    println!("\n{}", "✨ Authentication scaffolded successfully!".green().bold());
    println!("{}", "Jalankan 'cargo rustbasic route:list' untuk melihat rute baru.".dimmed());
}

pub async fn remove_auth() {
    println!("\n{}", "🗑️  Removing Authentication Scaffold...".red().bold());

    // 1. Delete src/routes/auth.rs
    let auth_route_path = "src/routes/auth.rs";
    if std::path::Path::new(auth_route_path).exists() {
        fs::remove_file(auth_route_path).ok();
        println!("   {} {}", "✅ Deleted:".green(), auth_route_path.cyan());
    }

    // 2. Update src/routes/mod.rs
    let routes_mod_path = "src/routes/mod.rs";
    if let Ok(mut content) = fs::read_to_string(routes_mod_path)
        && content.contains("pub mod auth;") {
            content = content.replace("pub mod auth;\n", "");
            fs::write(routes_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), routes_mod_path.cyan());
        }

    // 3. Update src/routes/web.rs
    let web_route_path = "src/routes/web.rs";
    if let Ok(mut content) = fs::read_to_string(web_route_path) {
        let mut changed = false;
        
        // Remove imports
        if content.contains("use rustbasic_core::axum::{Router, routing::{get, post}, middleware::from_fn};") {
            content = content.replace("use rustbasic_core::axum::{Router, routing::{get, post}, middleware::from_fn};", "use rustbasic_core::axum::{Router, routing::get};");
            changed = true;
        }
        
        let imports_to_remove = [
            "use crate::app::http::controllers::{auth, dashboard_controller};\n",
            "use crate::app::http::middleware::auth::auth_middleware;\n",
            "use rustbasic_core::server::AppState;\n",
            "use crate::routes::auth as auth_routes;\n",
            "use crate::app::http::controllers::{auth, dashboard_controller};",
            "use crate::app::http::middleware::auth::auth_middleware;",
            "use crate::routes::auth as auth_routes;",
        ];
        
        for imp in imports_to_remove {
            if content.contains(imp) {
                content = content.replace(imp, "");
                changed = true;
            }
        }
        
        // Re-add server::AppState if it was removed
        if !content.contains("use rustbasic_core::server::AppState;") {
            content = content.replace("use rustbasic_core::axum::{Router, routing::get};", "use rustbasic_core::axum::{Router, routing::get};\nuse rustbasic_core::server::AppState;");
        }

        // Remove auth_protected_routes logic and restore basic Router
        if content.contains("let auth_protected_routes = Router::new()") {
            let re = Regex::new(r##"(?s)\s*let auth_protected_routes = Router::new\(\).*?\.layer\(from_fn\(auth_middleware\)\);\s*"##).unwrap();
            content = re.replace(&content, "\n").to_string();
            
            content = content.replace(".merge(auth_routes::router())", "");
            content = content.replace(".merge(auth_protected_routes)", "");
            
            // Restore clean Router::new()
            let clean_router = r#"    Router::new()
        .route("/", get(welcome_controller::index))
        .route("/about", get(welcome_controller::about))
        .route("/dev", get(welcome_controller::dev_info))"#;
            
            let router_re = Regex::new(r##"(?s)Router::new\(\).*?\.route\(\s*\"/dev\"\s*,\s*get\(welcome_controller::dev_info\)\s*\)"##).unwrap();
            content = router_re.replace(&content, clean_router).to_string();
            
            // Final cleanup of multiple newlines
            let multi_newline_re = Regex::new(r#"\n{3,}"#).unwrap();
            content = multi_newline_re.replace_all(&content, "\n\n").to_string();
            
            changed = true;
        }

        if changed {
            fs::write(web_route_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), web_route_path.cyan());
        }
    }

    // 4. Delete Controllers
    let auth_controller_dir = "src/app/http/controllers/auth";
    if std::path::Path::new(auth_controller_dir).exists() {
        fs::remove_dir_all(auth_controller_dir).ok();
        println!("   {} {}", "✅ Deleted:".green(), auth_controller_dir.cyan());
    }

    // 4.1 Delete Password Resets Migration & Model
    if let Ok(entries) = std::fs::read_dir("database/migrations") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str()
                && name.ends_with("_create_password_resets_table.rs") {
                    let path = entry.path();
                    fs::remove_file(&path).ok();
                    println!("   {} {}", "✅ Deleted:".green(), path.display().to_string().cyan());
                }
        }
    }
    
    let model_path = "src/app/models/password_resets.rs";
    if std::path::Path::new(model_path).exists() {
        fs::remove_file(model_path).ok();
        println!("   {} {}", "✅ Deleted:".green(), model_path.cyan());
    }

    // 5. Delete Components, React Pages/Auth & Dashboard.jsx
    let components_dir = "src/resources/js/Components";
    let toast_view = "src/resources/js/Components/Toast.jsx";
    if std::path::Path::new(toast_view).exists() {
        fs::remove_file(toast_view).ok();
    }
    let alert_banner_view = "src/resources/js/Components/AlertBanner.jsx";
    if std::path::Path::new(alert_banner_view).exists() {
        fs::remove_file(alert_banner_view).ok();
    }
    let form_input_view = "src/resources/js/Components/FormInput.jsx";
    if std::path::Path::new(form_input_view).exists() {
        fs::remove_file(form_input_view).ok();
    }
    if std::path::Path::new(components_dir).exists()
        && let Ok(entries) = std::fs::read_dir(components_dir)
            && entries.count() == 0 {
                fs::remove_dir(components_dir).ok();
            }

    let auth_page_dir = "src/resources/js/Pages/Auth";
    if std::path::Path::new(auth_page_dir).exists() {
        fs::remove_dir_all(auth_page_dir).ok();
        println!("   {} {}", "✅ Deleted:".green(), auth_page_dir.cyan());
    }

    let dashboard_page = "src/resources/js/Pages/Dashboard.jsx";
    if std::path::Path::new(dashboard_page).exists() {
        fs::remove_file(dashboard_page).ok();
        println!("   {} {}", "✅ Deleted:".green(), dashboard_page.cyan());
    }

    // 5.1 Delete Auth Middleware
    let auth_middleware_path = "src/app/http/middleware/auth.rs";
    if std::path::Path::new(auth_middleware_path).exists() {
        fs::remove_file(auth_middleware_path).ok();
        println!("   {} {}", "✅ Deleted:".green(), auth_middleware_path.cyan());
    }

    let middleware_mod_path = "src/app/http/middleware/mod.rs";
    if let Ok(mut content) = fs::read_to_string(middleware_mod_path)
        && content.contains("pub mod auth;") {
            content = content.replace("pub mod auth;\n", "");
            fs::write(middleware_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), middleware_mod_path.cyan());
        }

    // 6. Delete Dashboard Controller
    let dashboard_path = "src/app/http/controllers/dashboard_controller.rs";
    if std::path::Path::new(dashboard_path).exists() {
        fs::remove_file(dashboard_path).ok();
        println!("   {} {}", "✅ Deleted:".green(), dashboard_path.cyan());
    }

    // 7. Update src/app/http/controllers/mod.rs
    let controllers_mod_path = "src/app/http/controllers/mod.rs";
    if let Ok(mut content) = fs::read_to_string(controllers_mod_path) {
        let mut changed = false;
        if content.contains("pub mod auth;") {
            content = content.replace("pub mod auth;\n", "");
            changed = true;
        }
        if content.contains("pub mod dashboard_controller;") {
            content = content.replace("pub mod dashboard_controller;\n", "");
            changed = true;
        }
        if changed {
            fs::write(controllers_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), controllers_mod_path.cyan());
        }
    }

    // 7.1 Update src/app/models/mod.rs
    let models_mod_path = "src/app/models/mod.rs";
    if let Ok(mut content) = fs::read_to_string(models_mod_path) {
        let mut changed = false;
        if content.contains("pub mod password_resets;") {
            content = content.replace("pub mod password_resets;\n", "");
            content = content.replace("pub mod password_resets;", "");
            changed = true;
        }
        if changed {
            fs::write(models_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), models_mod_path.cyan());
        }
    }

    // 7.2 Update database/migrations/mod.rs
    let migration_mod_path = "database/migrations/mod.rs";
    if let Ok(content) = fs::read_to_string(migration_mod_path) {
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut changed = false;
        
        // Remove the mod line
        lines.retain(|line| {
            if line.contains("_create_password_resets_table;") || (line.contains("Box::new(") && line.contains("_create_password_resets_table::Migration")) {
                changed = true;
                false
            } else {
                true
            }
        });

        if changed {
            fs::write(migration_mod_path, lines.join("\n")).ok();
            println!("   {} {}", "📝 Updated:".blue(), migration_mod_path.cyan());
        }
    }

    // 7.3 Restore Welcome.jsx
    let welcome_path = "src/resources/js/Pages/Welcome.jsx";
    if let Ok(content) = fs::read_to_string(welcome_path)
        && content.contains("auth_installed ?") {
            let target = r#"          <div className="flex items-center gap-4">
            <span className="inline-flex items-center gap-1.5 px-3 h-8 rounded-full text-xs font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 mr-2">
              <span className="w-2 h-2 rounded-full bg-emerald-400" style={{ boxShadow: "0 0 10px #34d399" }} />
              Backend Online
            </span>
            {auth_installed ? (
              <Link 
                href="/dashboard" 
                className="px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-sm font-bold text-white transition-all duration-300"
                style={{ textDecoration: 'none' }}
              >
                Dashboard
              </Link>
            ) : (
              <div className="flex gap-2">
                <Link 
                  href="/login" 
                  className="px-4 py-2 rounded-lg border border-white/10 text-sm font-bold hover:bg-white/5 transition-all duration-300 text-gray-300 hover:text-white"
                  style={{ textDecoration: 'none' }}
                >
                  Masuk
                </Link>
                <Link 
                  href="/register" 
                  className="px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-sm font-bold text-white transition-all duration-300"
                  style={{ textDecoration: 'none' }}
                >
                  Daftar
                </Link>
              </div>
            )}
          </div>"#;

            let replacement = r#"          <div className="flex items-center gap-4">
            <span className="inline-flex items-center gap-1.5 px-3 h-8 rounded-full text-xs font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
              <span className="w-2 h-2 rounded-full bg-emerald-400" style={{ boxShadow: "0 0 10px #34d399" }} />
              Backend Online
            </span>
          </div>"#;

            let updated = content.replace(target, replacement);
            fs::write(welcome_path, updated).ok();
            println!("   {} {}", "📝 Restored:".blue(), welcome_path.cyan());
        }

    // 7.4 Delete Migration Record from Database
    println!("   {} {}", "⏳".blue(), "Cleaning up migration records from database...".dimmed());
    let db_connection = std::env::var("DB_CONNECTION").unwrap_or_else(|_| "sqlite".to_string());
    let db_host = std::env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let db_port = std::env::var("DB_PORT").unwrap_or_else(|_| "3306".to_string());
    let db_database = std::env::var("DB_DATABASE").unwrap_or_else(|_| "rustbasic".to_string());
    let db_username = std::env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string());
    let db_password = std::env::var("DB_PASSWORD").unwrap_or_default();

    let db_url = if db_connection == "mysql" {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            db_username, db_password, db_host, db_port, db_database
        )
    } else {
        format!("sqlite:database/{}.sqlite?mode=rwc", db_database)
    };

    if let Ok(db) = sea_orm::Database::connect(db_url).await {
        use sea_orm::ConnectionTrait;
        let table_name = if db_connection == "mysql" { "sea_orm_migrations" } else { "seaql_migrations" };
        let sql = format!("DELETE FROM {} WHERE version LIKE '%_create_password_resets_table'", table_name);
        let db_backend = if db_connection == "mysql" { sea_orm::DbBackend::MySql } else { sea_orm::DbBackend::Sqlite };
        let _ = db.execute(sea_orm::Statement::from_string(db_backend, sql)).await;
        println!("   {} {}", "✅ Cleaned:".green(), "Database migration records removed.".cyan());
    }

    // Clean up validator and sea-orm-migration from Cargo.toml
    if let Ok(mut content) = fs::read_to_string("Cargo.toml") {
        let mut changed = false;
        
        let validator_lines = [
            "validator = { version = \"0.20\", features = [\"derive\"] }\n",
            "validator = { version = \"0.20\", features = [\"derive\"] }",
        ];
        for line in &validator_lines {
            if content.contains(line) {
                content = content.replace(line, "");
                changed = true;
            }
        }
        
        let migration_lines = [
            "sea-orm-migration = { version = \"1.1\", features = [\"runtime-tokio-rustls\", \"sqlx-sqlite\", \"sqlx-mysql\"], default-features = false }\n",
            "sea-orm-migration = { version = \"1.1\", features = [\"runtime-tokio-rustls\", \"sqlx-sqlite\", \"sqlx-mysql\"], default-features = false }",
        ];
        for line in &migration_lines {
            if content.contains(line) {
                content = content.replace(line, "");
                changed = true;
            }
        }
        
        if changed {
            fs::write("Cargo.toml", content).ok();
            println!("   {} {}", "📝 Updated:".blue(), "Cargo.toml dependencies cleaned".cyan());
        }
    }

    println!("\n{}", "✨ Authentication removed successfully!".green().bold());
}
