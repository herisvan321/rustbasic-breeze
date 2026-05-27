use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use colored::*;
use regex::Regex;

use std::path::Path;

pub use rustbasic_core::MailService;

/// Menyiapkan scaffolding untuk module email
pub async fn setup_email() {
    println!("\n{}", "📧 Setup Email Module...".cyan().bold());

    // 1. Membuat folder templates/emails jika belum ada
    let email_view_dir = "src/resources/views/emails";
    if !Path::new(email_view_dir).exists() {
        fs::create_dir_all(email_view_dir).ok();
        println!("   {} {}", "✅ Created:".green(), email_view_dir.cyan());
    }

    // 2. Membuat file template welcome.rb.html jika belum ada
    let welcome_template_path = format!("{}/welcome.rb.html", email_view_dir);
    if !Path::new(&welcome_template_path).exists() {
        let welcome_template = r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body { font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #1e293b; margin: 0; padding: 0; background-color: #f8fafc; }
        .container { max-width: 600px; margin: 0 auto; padding: 40px 20px; }
        .card { background: #ffffff; border-radius: 20px; overflow: hidden; box-shadow: 0 10px 30px rgba(0,0,0,0.04); border: 1px solid #e2e8f0; }
        .header { background: linear-gradient(135deg, #4f46e5, #818cf8); padding: 48px 40px; text-align: center; color: white; }
        .header h1 { margin: 0; font-size: 28px; font-weight: 800; letter-spacing: -0.03em; }
        .header p { margin: 8px 0 0 0; font-size: 16px; opacity: 0.9; }
        .content { padding: 40px; }
        .greeting { font-size: 20px; font-weight: 700; color: #0f172a; margin-top: 0; margin-bottom: 16px; }
        p { margin: 16px 0; color: #475569; font-size: 15px; }
        .button-container { text-align: center; margin: 32px 0; }
        .button { display: inline-block; padding: 14px 36px; background: linear-gradient(135deg, #4f46e5, #6366f1); color: #ffffff !important; text-decoration: none; border-radius: 12px; font-weight: 600; box-shadow: 0 4px 14px rgba(79, 70, 229, 0.3); transition: all 0.2s ease; }
        .feature-grid { background: #f8fafc; border-radius: 14px; padding: 24px; margin: 24px 0; border: 1px solid #f1f5f9; }
        .feature-item { display: flex; align-items: start; margin-bottom: 16px; }
        .feature-item:last-child { margin-bottom: 0; }
        .feature-icon { margin-right: 12px; font-size: 18px; line-height: 1.5; }
        .feature-text { font-size: 14px; color: #334155; }
        .divider { height: 1px; background: #e2e8f0; margin: 32px 0; }
        .footer { padding: 0 20px; text-align: center; font-size: 13px; color: #94a3b8; }
        .footer p { color: #94a3b8; font-size: 13px; margin: 8px 0; }
    </style>
</head>
<body>
    <div class="container">
        <div class="card">
            <div class="header">
                <h1>{{ app_name }}</h1>
                <p>Selamat Datang di Ekosistem Kami</p>
            </div>
            <div class="content">
                <h2 class="greeting">Halo, {{ name }}!</h2>
                <p>Terima kasih telah bergabung dengan <strong>{{ app_name }}</strong>. Kami sangat senang menyambut Anda di platform pengembangan web modern bertenaga Rust ini.</p>
                
                <div class="feature-grid">
                    <div class="feature-item">
                        <span class="feature-icon">⚡</span>
                        <span class="feature-text"><strong>Performa Kilat:</strong> Rasakan kecepatan luar biasa dengan backend berbasis Rust dan Tokio.</span>
                    </div>
                    <div class="feature-item">
                        <span class="feature-icon">🛡️</span>
                        <span class="feature-text"><strong>Aman secara Bawaan:</strong> Sistem keamanan modern terintegrasi secara default di seluruh siklus aplikasi.</span>
                    </div>
                    <div class="feature-item">
                        <span class="feature-icon">🎨</span>
                        <span class="feature-text"><strong>Inertia & React:</strong> Integrasi frontend yang mulus memberikan pengalaman visual premium yang dinamis.</span>
                    </div>
                </div>

                <p>Untuk memulai dan menjelajahi semua fitur hebat yang tersedia, silakan masuk ke dashboard Anda sekarang.</p>

                <div class="button-container">
                    <a href="{{ dashboard_url }}" class="button">Masuk ke Dashboard</a>
                </div>

                <p>Jika Anda memiliki pertanyaan atau butuh bantuan, tim kami siap membantu Anda kapan saja. Silakan balas email ini secara langsung.</p>
                
                <div class="divider"></div>
                
                <p style="font-size: 12px; color: #94a3b8; margin-bottom: 0;">
                    Email ini dikirim secara otomatis oleh sistem {{ app_name }}. Tolong jangan bagikan tautan login Anda kepada siapa pun demi keamanan akun Anda.
                </p>
            </div>
        </div>
        <div class="footer">
            <p>&copy; 2026 {{ app_name }}. All rights reserved.</p>
            <p>DKI Jakarta, Indonesia</p>
        </div>
    </div>
</body>
</html>
"##;
        fs::write(&welcome_template_path, welcome_template).ok();
        println!("   {} {}", "✅ Created:".green(), welcome_template_path.cyan());
    }

    // 3. Membuat file template reset.rb.html jika belum ada
    let reset_template_path = format!("{}/reset.rb.html", email_view_dir);
    if !Path::new(&reset_template_path).exists() {
        let reset_template = r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body { font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #1e293b; margin: 0; padding: 0; background-color: #f8fafc; }
        .container { max-width: 600px; margin: 0 auto; padding: 40px 20px; }
        .card { background: #ffffff; border-radius: 20px; overflow: hidden; box-shadow: 0 10px 30px rgba(0,0,0,0.04); border: 1px solid #e2e8f0; }
        .header { background: linear-gradient(135deg, #4f46e5, #818cf8); padding: 48px 40px; text-align: center; color: white; }
        .header h1 { margin: 0; font-size: 28px; font-weight: 800; letter-spacing: -0.03em; }
        .header p { margin: 8px 0 0 0; font-size: 16px; opacity: 0.9; }
        .content { padding: 40px; }
        .greeting { font-size: 20px; font-weight: 700; color: #0f172a; margin-top: 0; margin-bottom: 16px; }
        p { margin: 16px 0; color: #475569; font-size: 15px; }
        .button-container { text-align: center; margin: 32px 0; }
        .button { display: inline-block; padding: 14px 36px; background: linear-gradient(135deg, #4f46e5, #6366f1); color: #ffffff !important; text-decoration: none; border-radius: 12px; font-weight: 600; box-shadow: 0 4px 14px rgba(79, 70, 229, 0.3); transition: all 0.2s ease; }
        .divider { height: 1px; background: #e2e8f0; margin: 32px 0; }
        .footer { padding: 0 20px; text-align: center; font-size: 13px; color: #94a3b8; }
        .footer p { color: #94a3b8; font-size: 13px; margin: 8px 0; }
    </style>
</head>
<body>
    <div class="container">
        <div class="card">
            <div class="header">
                <h1>{{ app_name }}</h1>
                <p>Permintaan Reset Kata Sandi</p>
            </div>
            <div class="content">
                <h2 class="greeting">Halo!</h2>
                <p>Anda menerima email ini karena kami menerima permintaan reset password untuk akun Anda di <strong>{{ app_name }}</strong>.</p>
                
                <div class="button-container">
                    <a href="{{ reset_url }}" class="button">Reset Password Saya</a>
                </div>

                <p style="font-size: 14px; color: #64748b;">Tautan reset ini akan kedaluwarsa dalam 60 menit. Jika Anda tidak meminta reset password, abaikan saja email ini.</p>
                
                <div class="divider"></div>
                
                <p style="font-size: 12px; color: #94a3b8; word-break: break-all;">
                    Jika tombol tidak berfungsi, salin dan tempel URL berikut di browser Anda:<br>
                    <span style="color: #4f46e5;">{{ reset_url }}</span>
                </p>
            </div>
        </div>
        <div class="footer">
            <p>&copy; 2026 {{ app_name }}. All rights reserved.</p>
        </div>
    </div>
</body>
</html>
"##;
        fs::write(&reset_template_path, reset_template).ok();
        println!("   {} {}", "✅ Created:".green(), reset_template_path.cyan());
    }

    // 4. Membuat file script uji coba kirim email di src/bin/send_test_email.rs jika belum ada
    let bin_dir = "src/bin";
    if !Path::new(bin_dir).exists() {
        fs::create_dir_all(bin_dir).ok();
        println!("   {} {}", "✅ Created:".green(), bin_dir.cyan());
    }

    let test_email_path = format!("{}/send_test_email.rs", bin_dir);
    if !Path::new(&test_email_path).exists() {
        let test_email_script = r#"use rustbasic_core::dotenvy::dotenv;
use rustbasic_core::MailService;

#[tokio::main]
async fn main() {
    // Muat file .env jika ada
    dotenv().ok();
    
    println!("📧 Mengirim email uji coba...");
    
    let to = "test@example.com";
    let subject = "Test Email dari RustBasic";
    let body = "<h1>Halo!</h1><p>Ini adalah email uji coba yang dikirim menggunakan modul <code>rustbasic-email</code>.</p>";
    
    match MailService::send_email(to, subject, body).await {
        Ok(_) => println!("✅ Email uji coba berhasil dikirim ke {}!", to),
        Err(e) => eprintln!("❌ Gagal mengirim email uji coba: {}", e),
    }
}
"#;
        fs::write(&test_email_path, test_email_script).ok();
        println!("   {} {}", "✅ Created:".green(), test_email_path.cyan());
    }
}

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
    let search_pattern = "fn migrations() -> Vec<Box<dyn MigrationTrait>> {";
    
    if let Some(_pos) = content.find(search_pattern) {
        let insert_pos = content.find("        ]").unwrap_or(content.len());
        content.insert_str(insert_pos, &format!("            Box::new({}::Migration),\n", mod_name));
    }

    fs::write(mod_path, content).expect("Gagal memperbarui database/migrations/mod.rs");
    println!("{} {}", "📝".blue(), "database/migrations/mod.rs diperbarui.".dimmed());
}

pub async fn make_auth() {
    println!("\n{}", "🔐 Scaffolding Authentication...".magenta().bold());

    // Otomatis jalankan setup email sebagai bagian dari paket breeze
    setup_email().await;



    // 1. Create src/routes/auth.rs
    let auth_route_path = "src/routes/auth.rs";
    let auth_route_template = r#"use rustbasic_core::{Router, get, post, from_fn, AppState};
use crate::app::http::controllers::auth;
use crate::app::http::middleware::auth::guest_middleware;

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
            content = content.replace("use rustbasic_core::{Router, get, AppState};", "use rustbasic_core::{Router, get, post, from_fn, AppState};");
            content = content.replace("use rustbasic_core::{Router, get, post, from_fn, AppState};", "use crate::app::http::controllers::{auth, dashboard_controller};\nuse crate::app::http::middleware::auth::auth_middleware;\nuse rustbasic_core::{Router, get, post, from_fn, AppState};\nuse crate::routes::auth as auth_routes;");

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
        let migration_template = format!(r#"use rustbasic_core::{{Schema, SchemaManager, MigrationTrait, DbErr}};
use rustbasic_core::async_trait;

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {{
    fn name(&self) -> &str {{
        "{}"
    }}

    async fn up<'a>(&self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {{
        Schema::create(manager, "password_resets", |table| {{
            table.no_id();
            table.string("email").not_null().primary_key();
            table.string("token").not_null();
        }}).await
    }}

    async fn down<'a>(&self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {{
        Schema::drop(manager, "password_resets").await
    }}
}}
"#, migration_name);
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
        let middleware_template = r#"use rustbasic_core::{Request, Response, Next, IntoResponse, Redirect};

pub async fn auth_middleware(req: Request, next: Next) -> Response {
    if req.session.get::<i32>("user_id").is_none() {
        req.session.set("error", "Silakan login terlebih dahulu");
        return Redirect::to("/login").into_response();
    }
    next.run(req).await
}

pub async fn guest_middleware(req: Request, next: Next) -> Response {
    if req.session.get::<i32>("user_id").is_some() {
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
use rustbasic_core::chrono::NaiveDateTime;

model! {
    table: "password_resets",
    timestamps: false,
    fillable: [email, token, created_at],
    guarded: [],
    Model {
        pub email: String,
        pub token: String,
        pub created_at: NaiveDateTime,
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
use rustbasic_core::{IntoResponse, Response, Redirect, State};
use rustbasic_core::bcrypt::{hash, verify, DEFAULT_COST};
use rustbasic_core::uuid::Uuid;
use rustbasic_core::serde::Deserialize;
use rustbasic_core::validator::Validate;
use rustbasic_core::MailService;
use rustbasic_core::database::DB;
use rustbasic_core::serde_json::json;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Validate for RegisterRequest {
    fn validate(&self) -> Result<(), std::collections::HashMap<String, String>> {
        let mut errors = std::collections::HashMap::new();
        if self.name.trim().len() < 3 {
            errors.insert("name".to_string(), "Nama minimal 3 karakter".to_string());
        }
        if !self.email.contains('@') {
            errors.insert("email".to_string(), "Format email tidak valid".to_string());
        }
        if self.password.len() < 8 {
            errors.insert("password".to_string(), "Password minimal 8 karakter".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember: Option<bool>,
}

impl Validate for LoginRequest {
    fn validate(&self) -> Result<(), std::collections::HashMap<String, String>> {
        let mut errors = std::collections::HashMap::new();
        if !self.email.contains('@') {
            errors.insert("email".to_string(), "Format email tidak valid".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

impl Validate for ForgotPasswordRequest {
    fn validate(&self) -> Result<(), std::collections::HashMap<String, String>> {
        let mut errors = std::collections::HashMap::new();
        if !self.email.contains('@') {
            errors.insert("email".to_string(), "Format email tidak valid".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

impl Validate for ResetPasswordRequest {
    fn validate(&self) -> Result<(), std::collections::HashMap<String, String>> {
        let mut errors = std::collections::HashMap::new();
        if self.password.len() < 8 {
            errors.insert("password".to_string(), "Password minimal 8 karakter".to_string());
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
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
        let existing = DB::table(&state.db, "users")
            .where_("email", &data.email)
            .first::<User>()
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
        let user = DB::table(&state.db, "users")
            .where_("email", &data.email)
            .first::<User>()
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
        let user = DB::table(&state.db, "users")
            .where_("email", &data.email)
            .first::<User>()
            .await
            .ok()
            .flatten();

        if let Some(u) = user {
            // 2. Generate Token
            let token = Uuid::new_v4().to_string();

            // 3. Simpan Token (Hapus token lama jika ada, lalu insert)
            let _ = DB::table(&state.db, "password_resets")
                .where_("email", &u.email)
                .delete()
                .await;
            
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
        let reset = DB::table(&state.db, "password_resets")
            .where_("token", &data.token)
            .first::<PasswordReset>()
            .await
            .ok()
            .flatten();

        if let Some(r) = reset {
            // 2. Cek Kadaluarsa (60 Menit)
            let now = rustbasic_core::chrono::Utc::now().naive_utc();
            let duration = now.signed_duration_since(r.created_at);
            
            if duration.num_minutes() > 60 {
                // Hapus token yang sudah kadaluarsa
                let _ = DB::table(&state.db, "password_resets")
                    .where_("email", &r.email)
                    .delete()
                    .await;
                    
                req.session.set("error", "Tautan reset password sudah kadaluarsa (melebihi 60 menit).");
                return Redirect::to("/login").into_response();
            }

            // 3. Hash Password Baru
            let hashed = rustbasic_core::bcrypt::hash(data.password, rustbasic_core::bcrypt::DEFAULT_COST).unwrap();

            // 4. Update User
            let _ = DB::table(&state.db, "users")
                .where_("email", &r.email)
                .update(rustbasic_core::serde_json::json!({
                    "password": hashed
                }))
                .await;

            // 5. Hapus Token
            let _ = DB::table(&state.db, "password_resets")
                .where_("email", &r.email)
                .delete()
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

type ToastType = 'success' | 'error' | 'warning' | 'info';

interface SingleToastProps {
  id: string;
  type: ToastType;
  message: string;
  duration?: number;
  onDismiss: (id: string) => void;
}

interface ToastProps {
  flash?: {
    success?: string;
    error?: string;
    warning?: string;
    info?: string;
  } | null;
  duration?: number;
  position?: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
}

const ICONS: Record<ToastType, React.ReactNode> = {
  success: (
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
      <circle cx="10" cy="10" r="10" fill="currentColor" opacity={0.15} />
      <path d="M6 10.5L8.5 13L14 7" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  ),
  error: (
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
      <circle cx="10" cy="10" r="10" fill="currentColor" opacity={0.15} />
      <path d="M7 7L13 13M13 7L7 13" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  ),
  warning: (
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
      <circle cx="10" cy="10" r="10" fill="currentColor" opacity={0.15} />
      <path d="M10 6V11" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <circle cx="10" cy="14" r="1" fill="currentColor" />
    </svg>
  ),
  info: (
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
      <circle cx="10" cy="10" r="10" fill="currentColor" opacity={0.15} />
      <path d="M10 9V14" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <circle cx="10" cy="6.5" r="1" fill="currentColor" />
    </svg>
  ),
};

const STYLES: Record<ToastType, { bg: string; border: string; color: string; progress: string; shadow: string }> = {
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

function SingleToast({ id, type, message, duration = 5000, onDismiss }: SingleToastProps) {
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

interface ToastItem {
  id: string;
  type: ToastType;
  message: string;
}

export default function Toast({ flash, duration = 5000, position = 'top-right' }: ToastProps) {
  const [toasts, setToasts] = useState<ToastItem[]>([]);

  useEffect(() => {
    if (!flash) return;
    const newToasts: ToastItem[] = [];
    if (flash.success) newToasts.push({ id: Date.now() + '_s', type: 'success', message: flash.success });
    if (flash.error) newToasts.push({ id: Date.now() + '_e', type: 'error', message: flash.error });
    if (flash.warning) newToasts.push({ id: Date.now() + '_w', type: 'warning', message: flash.warning });
    if (flash.info) newToasts.push({ id: Date.now() + '_i', type: 'info', message: flash.info });

    if (newToasts.length > 0) {
      setToasts(prev => [...prev, ...newToasts]);
    }
  }, [flash?.success, flash?.error, flash?.warning, flash?.info]);

  const handleDismiss = useCallback((id: string) => {
    setToasts(prev => prev.filter(t => t.id !== id));
  }, []);

  const positionStyle: Record<string, React.CSSProperties> = {
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

type AlertType = 'success' | 'error' | 'warning' | 'info';

interface AlertBannerProps {
  type?: AlertType;
  message?: string | null;
  onDismiss?: () => void;
}

const ALERT_STYLES: Record<AlertType, { bg: string; border: string; color: string; icon: string }> = {
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

export default function AlertBanner({ type = 'info', message, onDismiss }: AlertBannerProps) {
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
import { useTheme } from '../Layouts/AppLayout';

interface FormInputProps {
  label?: string;
  type?: string;
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  error?: string;
  placeholder?: string;
  required?: boolean;
  autoFocus?: boolean;
  disabled?: boolean;
}

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
}: FormInputProps) {
  const { isDark } = useTheme();
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
          color: hasError ? '#fb7185' : (isDark ? '#94a3b8' : '#64748b'),
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
          background: isDark ? 'rgba(2, 6, 23, 0.8)' : 'rgba(255, 255, 255, 0.9)',
          border: `1px solid ${hasError ? 'rgba(244, 63, 94, 0.5)' : (isDark ? 'rgba(30, 41, 59, 1)' : 'rgba(203, 213, 225, 1)')}`,
          borderRadius: '12px',
          padding: '12px 14px',
          fontSize: '14px',
          color: isDark ? '#ffffff' : '#0f172a',
          outline: 'none',
          transition: 'all 0.3s ease',
          opacity: disabled ? 0.5 : 1,
        }}
        onFocus={(e) => {
          e.target.style.borderColor = hasError ? 'rgba(244, 63, 94, 0.7)' : 'rgba(232, 82, 14, 0.5)';
          e.target.style.boxShadow = hasError ? '0 0 0 3px rgba(244, 63, 94, 0.1)' : '0 0 0 3px rgba(232, 82, 14, 0.1)';
        }}
        onBlur={(e) => {
          e.target.style.borderColor = hasError ? 'rgba(244, 63, 94, 0.5)' : (isDark ? 'rgba(30, 41, 59, 1)' : 'rgba(203, 213, 225, 1)');
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
  const { flash } = usePage<any>().props;
  const { data, setData, post, processing, errors } = useForm({
    email: '',
    password: '',
    remember: false,
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    post('/login');
  };

  return (
    <div style={{ padding: '60px 24px', flexGrow: 1, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
      <Toast flash={flash} />

      <div className="glassmorphism" style={{
        width: '100%',
        maxWidth: '420px',
        background: 'rgba(255, 255, 255, 0.02)',
        backdropFilter: 'blur(20px)',
        border: '1px solid rgba(232, 82, 14, 0.15)',
        borderRadius: '24px',
        padding: '32px',
        boxShadow: '0 8px 32px rgba(0,0,0,0.1)',
        position: 'relative',
        overflow: 'hidden',
      }}>
        <div style={{
          position: 'absolute', top: 0, right: 0, width: '128px', height: '128px',
          background: 'rgba(232, 82, 14, 0.05)', borderRadius: '50%', filter: 'blur(40px)', pointerEvents: 'none'
        }} />
        
        <div style={{ textAlign: 'center', marginBottom: '32px' }}>
          <span style={{
            fontSize: '11px', fontWeight: 700, letterSpacing: '0.1em', color: '#e8520e',
            background: 'rgba(232, 82, 14, 0.08)', border: '1px solid rgba(232, 82, 14, 0.15)',
            padding: '4px 12px', borderRadius: '999px', textTransform: 'uppercase'
          }}>
            RustBasic SPA
          </span>
          <h1 style={{ fontSize: '1.75rem', fontWeight: 900, color: 'inherit', marginTop: '16px', marginBottom: '8px' }}>Selamat Datang</h1>
          <p style={{ fontSize: '0.85rem', color: '#888', margin: 0 }}>Silakan masuk ke akun Anda</p>
        </div>

        {flash?.success && <AlertBanner type="success" message={flash.success} />}
        {flash?.error && <AlertBanner type="error" message={flash.error} />}

        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
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

          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', fontSize: '0.85rem' }}>
            <label style={{ display: 'flex', alignItems: 'center', gap: '8px', cursor: 'pointer', color: '#888' }}>
              <input
                type="checkbox"
                checked={data.remember}
                onChange={(e) => setData('remember', e.target.checked)}
                style={{
                  width: '16px', height: '16px', borderRadius: '4px',
                  border: '1px solid rgba(232, 82, 14, 0.2)', background: 'transparent'
                }}
              />
              <span style={{ userSelect: 'none' }}>Ingat Saya</span>
            </label>
            <Link href="/forgot-password" style={{ color: '#e8520e', fontWeight: 600, textDecoration: 'none' }}>
              Lupa Password?
            </Link>
          </div>

          <button
            type="submit"
            disabled={processing}
            style={{
              width: '100%',
              padding: '14px',
              background: 'linear-gradient(135deg, #e8520e, #d4470b)',
              border: 'none',
              borderRadius: '12px',
              fontWeight: 700,
              fontSize: '0.9rem',
              color: '#fff',
              cursor: 'pointer',
              boxShadow: '0 4px 20px rgba(232,82,14,0.3)',
              transition: 'all 0.2s',
              opacity: processing ? 0.6 : 1,
            }}
          >
            {processing ? 'MEMROSES...' : 'MASUK KE DASHBOARD'}
          </button>
        </form>

        <p style={{ textAlign: 'center', fontSize: '0.85rem', color: '#666', marginTop: '32px', marginBottom: 0 }}>
          Belum punya akun?{' '}
          <Link href="/register" style={{ color: '#e8520e', fontWeight: 700, textDecoration: 'none' }}>
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
  const { flash } = usePage<any>().props;
  const { data, setData, post, processing, errors } = useForm({
    name: '',
    email: '',
    password: '',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    post('/register');
  };

  return (
    <div style={{ padding: '60px 24px', flexGrow: 1, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
      <Toast flash={flash} />

      <div className="glassmorphism" style={{
        width: '100%',
        maxWidth: '420px',
        background: 'rgba(255, 255, 255, 0.02)',
        backdropFilter: 'blur(20px)',
        border: '1px solid rgba(232, 82, 14, 0.15)',
        borderRadius: '24px',
        padding: '32px',
        boxShadow: '0 8px 32px rgba(0,0,0,0.1)',
        position: 'relative',
        overflow: 'hidden',
      }}>
        <div style={{
          position: 'absolute', top: 0, right: 0, width: '128px', height: '128px',
          background: 'rgba(232, 82, 14, 0.05)', borderRadius: '50%', filter: 'blur(40px)', pointerEvents: 'none'
        }} />
        
        <div style={{ textAlign: 'center', marginBottom: '32px' }}>
          <span style={{
            fontSize: '11px', fontWeight: 700, letterSpacing: '0.1em', color: '#e8520e',
            background: 'rgba(232, 82, 14, 0.08)', border: '1px solid rgba(232, 82, 14, 0.15)',
            padding: '4px 12px', borderRadius: '999px', textTransform: 'uppercase'
          }}>
            RustBasic SPA
          </span>
          <h1 style={{ fontSize: '1.75rem', fontWeight: 900, color: 'inherit', marginTop: '16px', marginBottom: '8px' }}>Daftar Akun</h1>
          <p style={{ fontSize: '0.85rem', color: '#888', margin: 0 }}>Mulai perjalanan Anda bersama kami</p>
        </div>

        {flash?.error && <AlertBanner type="error" message={flash.error} />}
        {flash?.success && <AlertBanner type="success" message={flash.success} />}

        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
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
            style={{
              width: '100%',
              padding: '14px',
              background: 'linear-gradient(135deg, #e8520e, #d4470b)',
              border: 'none',
              borderRadius: '12px',
              fontWeight: 700,
              fontSize: '0.9rem',
              color: '#fff',
              cursor: 'pointer',
              boxShadow: '0 4px 20px rgba(232,82,14,0.3)',
              transition: 'all 0.2s',
              opacity: processing ? 0.6 : 1,
            }}
          >
            {processing ? 'MENDAFTAR...' : 'BUAT AKUN SEKARANG'}
          </button>
        </form>

        <p style={{ textAlign: 'center', fontSize: '0.85rem', color: '#666', marginTop: '32px', marginBottom: 0 }}>
          Sudah punya akun?{' '}
          <Link href="/login" style={{ color: '#e8520e', fontWeight: 700, textDecoration: 'none' }}>
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
  const { flash } = usePage<any>().props;
  const { data, setData, post, processing, errors } = useForm({
    email: '',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    post('/forgot-password');
  };

  return (
    <div style={{ padding: '60px 24px', flexGrow: 1, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
      <Toast flash={flash} />

      <div className="glassmorphism" style={{
        width: '100%',
        maxWidth: '420px',
        background: 'rgba(255, 255, 255, 0.02)',
        backdropFilter: 'blur(20px)',
        border: '1px solid rgba(232, 82, 14, 0.15)',
        borderRadius: '24px',
        padding: '32px',
        boxShadow: '0 8px 32px rgba(0,0,0,0.1)',
        position: 'relative',
        overflow: 'hidden',
      }}>
        <div style={{
          position: 'absolute', top: 0, right: 0, width: '128px', height: '128px',
          background: 'rgba(232, 82, 14, 0.05)', borderRadius: '50%', filter: 'blur(40px)', pointerEvents: 'none'
        }} />
        
        <div style={{ textAlign: 'center', marginBottom: '32px' }}>
          <span style={{
            fontSize: '11px', fontWeight: 700, letterSpacing: '0.1em', color: '#e8520e',
            background: 'rgba(232, 82, 14, 0.08)', border: '1px solid rgba(232, 82, 14, 0.15)',
            padding: '4px 12px', borderRadius: '999px', textTransform: 'uppercase'
          }}>
            Keamanan Akun
          </span>
          <h1 style={{ fontSize: '1.75rem', fontWeight: 900, color: 'inherit', marginTop: '16px', marginBottom: '8px' }}>Lupa Password</h1>
          <p style={{ fontSize: '0.85rem', color: '#888', margin: 0 }}>Kami akan mengirimkan instruksi ke email Anda</p>
        </div>

        {flash?.success && <AlertBanner type="success" message={flash.success} />}
        {flash?.error && <AlertBanner type="error" message={flash.error} />}

        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
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
            style={{
              width: '100%',
              padding: '14px',
              background: 'linear-gradient(135deg, #e8520e, #d4470b)',
              border: 'none',
              borderRadius: '12px',
              fontWeight: 700,
              fontSize: '0.9rem',
              color: '#fff',
              cursor: 'pointer',
              boxShadow: '0 4px 20px rgba(232,82,14,0.3)',
              transition: 'all 0.2s',
              opacity: processing ? 0.6 : 1,
            }}
          >
            {processing ? 'MENGIRIM...' : 'KIRIM LINK RESET PASSWORD'}
          </button>
        </form>

        <p style={{ textAlign: 'center', fontSize: '0.85rem', color: '#666', marginTop: '32px', marginBottom: 0 }}>
          Ingat password Anda?{' '}
          <Link href="/login" style={{ color: '#e8520e', fontWeight: 700, textDecoration: 'none' }}>
            Login Disini
          </Link>
        </p>
      </div>
    </div>
  );
}
"##;

    let toast_view = "src/resources/js/Components/Toast.tsx";
    if !std::path::Path::new(toast_view).exists() {
        fs::write(toast_view, toast_template).ok();
    }

    let alert_banner_view = "src/resources/js/Components/AlertBanner.tsx";
    if !std::path::Path::new(alert_banner_view).exists() {
        fs::write(alert_banner_view, alert_banner_template).ok();
    }

    let form_input_view = "src/resources/js/Components/FormInput.tsx";
    if !std::path::Path::new(form_input_view).exists() {
        fs::write(form_input_view, form_input_template).ok();
    }

    let login_view = "src/resources/js/Pages/Auth/Login.tsx";
    if !std::path::Path::new(login_view).exists() {
        fs::write(login_view, login_template).ok();
    }
    
    let register_view = "src/resources/js/Pages/Auth/Register.tsx";
    if !std::path::Path::new(register_view).exists() {
        fs::write(register_view, register_template).ok();
    }

    let forgot_view = "src/resources/js/Pages/Auth/ForgotPassword.tsx";
    if !std::path::Path::new(forgot_view).exists() {
        fs::write(forgot_view, forgot_template).ok();
    }

    let reset_view = "src/resources/js/Pages/Auth/ResetPassword.tsx";
    if !std::path::Path::new(reset_view).exists() {
        let reset_template = r##"import React from 'react';
import { useForm, usePage } from '@inertiajs/react';
import Toast from '../../Components/Toast';
import AlertBanner from '../../Components/AlertBanner';
import FormInput from '../../Components/FormInput';

interface ResetPasswordProps {
  token: string;
}

export default function ResetPassword({ token }: ResetPasswordProps) {
  const { flash } = usePage<any>().props;
  const { data, setData, post, processing, errors } = useForm({
    token: token || '',
    password: '',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    post('/reset-password');
  };

  return (
    <div style={{ padding: '60px 24px', flexGrow: 1, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
      <Toast flash={flash} />

      <div className="glassmorphism" style={{
        width: '100%',
        maxWidth: '420px',
        background: 'rgba(255, 255, 255, 0.02)',
        backdropFilter: 'blur(20px)',
        border: '1px solid rgba(232, 82, 14, 0.15)',
        borderRadius: '24px',
        padding: '32px',
        boxShadow: '0 8px 32px rgba(0,0,0,0.1)',
        position: 'relative',
        overflow: 'hidden',
      }}>
        <div style={{
          position: 'absolute', top: 0, right: 0, width: '128px', height: '128px',
          background: 'rgba(232, 82, 14, 0.05)', borderRadius: '50%', filter: 'blur(40px)', pointerEvents: 'none'
        }} />
        
        <div style={{ textAlign: 'center', marginBottom: '32px' }}>
          <span style={{
            fontSize: '11px', fontWeight: 700, letterSpacing: '0.1em', color: '#e8520e',
            background: 'rgba(232, 82, 14, 0.08)', border: '1px solid rgba(232, 82, 14, 0.15)',
            padding: '4px 12px', borderRadius: '999px', textTransform: 'uppercase'
          }}>
            Akses Akun
          </span>
          <h1 style={{ fontSize: '1.75rem', fontWeight: 900, color: 'inherit', marginTop: '16px', marginBottom: '8px' }}>Reset Password</h1>
          <p style={{ fontSize: '0.85rem', color: '#888', margin: 0 }}>Silakan masukkan password baru Anda</p>
        </div>

        {flash?.error && <AlertBanner type="error" message={flash.error} />}

        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
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
            style={{
              width: '100%',
              padding: '14px',
              background: 'linear-gradient(135deg, #e8520e, #d4470b)',
              border: 'none',
              borderRadius: '12px',
              fontWeight: 700,
              fontSize: '0.9rem',
              color: '#fff',
              cursor: 'pointer',
              boxShadow: '0 4px 20px rgba(232, 82, 14, 0.3)',
              transition: 'all 0.2s',
              opacity: processing ? 0.6 : 1,
            }}
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
    let dashboard_view = "src/resources/js/Pages/Dashboard.tsx";
    if !std::path::Path::new(dashboard_view).exists() {
        let dashboard_template = r##"import React from 'react';
import { Link, router, usePage } from '@inertiajs/react';
import Toast from '../Components/Toast';
import { useTheme } from '../Layouts/AppLayout';

interface DashboardProps {
  title?: string;
  userName?: string;
  userEmail?: string;
  totalUsers?: number;
}

export default function Dashboard({ title, userName, userEmail, totalUsers }: DashboardProps) {
  const { flash } = usePage<any>().props;
  const { colors, isDark } = useTheme();

  const handleLogout = (e: React.FormEvent) => {
    e.preventDefault();
    router.post('/logout');
  };

  return (
    <div style={{
      display: 'flex',
      flexDirection: 'column',
      flexGrow: 1,
      width: '100%',
      boxSizing: 'border-box',
    }}>
      <Toast flash={flash} />
      
      <div style={{
        maxWidth: '1280px',
        width: '100%',
        margin: '0 auto',
        padding: '40px 24px 80px',
        display: 'flex',
        flexDirection: 'column',
        gap: '32px',
        position: 'relative',
        zIndex: 10,
      }}>
        {/* Header Section */}
        <div style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '16px',
        }}>
          <div>
            <h1 style={{
              fontSize: '2.25rem',
              fontWeight: 900,
              color: colors.textMain,
              margin: 0,
              letterSpacing: '-0.02em',
            }}>
              {title || 'Dashboard'}
            </h1>
            <p style={{
              fontSize: '0.9rem',
              color: colors.textMuted,
              marginTop: '4px',
              marginBottom: 0,
            }}>
              Selamat datang kembali, kendalikan project Anda secara instan.
            </p>
          </div>

          <div style={{
            display: 'inline-flex',
            alignItems: 'center',
            gap: '8px',
            padding: '8px 16px',
            borderRadius: '999px',
            fontSize: '0.75rem',
            fontWeight: 700,
            background: 'rgba(16,185,129,0.08)',
            color: '#34d399',
            border: '1px solid rgba(16,185,129,0.15)',
          }}>
            <span style={{
              width: '8px',
              height: '8px',
              borderRadius: '50%',
              background: '#34d399',
              boxShadow: '0 0 10px #34d399',
            }} />
            Server Status: Running
          </div>
        </div>

        {/* Stats Grid */}
        <div style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))',
          gap: '20px',
        }}>
          {/* Stat 1 */}
          <div style={{
            padding: '28px',
            borderRadius: '20px',
            border: `1px solid ${colors.cardBorder}`,
            background: colors.cardBg,
            display: 'flex',
            flexDirection: 'column',
            gap: '12px',
          }}>
            <span style={{
              fontSize: '0.7rem',
              fontWeight: 700,
              color: '#e8520e',
              letterSpacing: '0.1em',
              textTransform: 'uppercase',
            }}>
              User Terdaftar
            </span>
            <div style={{ display: 'flex', alignItems: 'baseline', gap: '8px' }}>
              <span style={{ fontSize: '3rem', fontWeight: 900, color: colors.textMain }}>
                {totalUsers || 0}
              </span>
              <span style={{ fontSize: '0.85rem', color: colors.textMuted }}>
                pengguna
              </span>
            </div>
          </div>

          {/* Stat 2 */}
          <div style={{
            padding: '28px',
            borderRadius: '20px',
            border: `1px solid ${colors.cardBorder}`,
            background: colors.cardBg,
            display: 'flex',
            flexDirection: 'column',
            gap: '12px',
          }}>
            <span style={{
              fontSize: '0.7rem',
              fontWeight: 700,
              color: '#ff8c42',
              letterSpacing: '0.1em',
              textTransform: 'uppercase',
            }}>
              Response Time
            </span>
            <div style={{ display: 'flex', alignItems: 'baseline', gap: '4px' }}>
              <span style={{ fontSize: '3rem', fontWeight: 900, color: colors.textMain }}>
                24
              </span>
              <span style={{ fontSize: '1rem', fontWeight: 700, color: colors.textMuted }}>
                ms
              </span>
            </div>
          </div>

          {/* Stat 3 */}
          <div style={{
            padding: '28px',
            borderRadius: '20px',
            border: `1px solid ${colors.activeCardBorder}`,
            background: colors.activeCardBg,
            display: 'flex',
            flexDirection: 'column',
            gap: '12px',
          }}>
            <span style={{
              fontSize: '0.7rem',
              fontWeight: 700,
              color: '#e8520e',
              letterSpacing: '0.1em',
              textTransform: 'uppercase',
            }}>
              Database Status
            </span>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginTop: '8px' }}>
              <span style={{
                width: '10px',
                height: '10px',
                borderRadius: '50%',
                background: '#34d399',
                boxShadow: '0 0 12px #34d399',
              }} />
              <span style={{ fontSize: '1.25rem', fontWeight: 800, color: '#34d399', letterSpacing: '0.05em' }}>
                HEALTHY
              </span>
            </div>
          </div>
        </div>

        {/* User profile & action panel */}
        <div style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))',
          gap: '24px',
        }}>
          {/* User profile card */}
          <div style={{
            padding: '32px',
            borderRadius: '24px',
            border: `1px solid ${colors.cardBorder}`,
            background: colors.cardBg,
            display: 'flex',
            flexDirection: 'column',
            gap: '20px',
          }}>
            <h3 style={{
              fontSize: '1.15rem',
              fontWeight: 800,
              color: colors.textMain,
              margin: 0,
            }}>
              👤 Profil Pengguna
            </h3>
            <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
              <div style={{
                width: '48px',
                height: '48px',
                borderRadius: '50%',
                background: 'linear-gradient(135deg, #e8520e, #ff8c42)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                fontWeight: 800,
                color: '#fff',
                fontSize: '1.2rem',
              }}>
                {userName ? userName[0].toUpperCase() : 'G'}
              </div>
              <div style={{ overflow: 'hidden' }}>
                <h4 style={{ margin: 0, fontSize: '0.95rem', fontWeight: 700, color: colors.textMain, whiteSpace: 'nowrap', textOverflow: 'ellipsis' }}>
                  {userName || 'Administrator'}
                </h4>
                <p style={{ margin: '2px 0 0 0', fontSize: '0.8rem', color: colors.textMuted, whiteSpace: 'nowrap', textOverflow: 'ellipsis' }}>
                  {userEmail || 'admin@rustbasic.dev'}
                </p>
              </div>
            </div>

            <form onSubmit={handleLogout} style={{ marginTop: '8px' }}>
              <button
                type="submit"
                style={{
                  width: '100%',
                  padding: '12px',
                  background: 'rgba(239, 68, 68, 0.08)',
                  border: '1px solid rgba(239, 68, 68, 0.15)',
                  borderRadius: '12px',
                  color: '#ef4444',
                  fontWeight: 700,
                  fontSize: '0.85rem',
                  cursor: 'pointer',
                  transition: 'all 0.2s',
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.background = 'rgba(239, 68, 68, 0.15)';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.background = 'rgba(239, 68, 68, 0.08)';
                }}
              >
                🚪 KELUAR SISTEM
              </button>
            </form>
          </div>

          {/* Server Info Panel */}
          <div style={{
            padding: '32px',
            borderRadius: '24px',
            border: `1px solid ${colors.cardBorder}`,
            background: colors.cardBg,
            display: 'flex',
            flexDirection: 'column',
            gap: '16px',
          }}>
            <h3 style={{
              fontSize: '1.15rem',
              fontWeight: 800,
              color: colors.textMain,
              margin: 0,
            }}>
              ⚙️ Informasi Kernel Server
            </h3>

            <div style={{
              padding: '16px',
              borderRadius: '12px',
              fontFamily: 'monospace',
              fontSize: '0.75rem',
              background: isDark ? 'rgba(0,0,0,0.3)' : 'rgba(0,0,0,0.03)',
              border: `1px solid ${colors.cardBorder}`,
              color: isDark ? '#34d399' : '#059669',
              lineHeight: 1.6,
            }}>
              <div>[OK] Compiled with Rust Engine 0.8.2</div>
              <div>[OK] Database Pool: Connection Established</div>
              <div>[OK] Modern SPA Routing: Powered by Inertia.js</div>
            </div>
          </div>
        </div>
      </div>
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
use rustbasic_core::{Response, State};
use rustbasic_core::database::DB;
use rustbasic_core::serde_json::json;

pub struct DashboardController;

impl DashboardController {
    pub async fn index(State(state): State<AppState>, req: Request) -> Response {
        let user_id = req.session.get::<i32>("user_id").unwrap_or(0);
        let user = DB::table(&state.db, "users").where_("id", user_id).first::<users::Model>().await.ok().flatten();
        let total_users = DB::table(&state.db, "users").count().await.unwrap_or(0);

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

    // 7. Update Welcome.tsx
    let welcome_path = "src/resources/js/Pages/Welcome.tsx";
    if let Ok(content) = fs::read_to_string(welcome_path)
        && !content.contains("auth_installed &&") {
            let target = r#"              <Link
                href="/about"
                style={{
                  padding: '16px 32px',
                  borderRadius: '16px',
                  background: 'linear-gradient(135deg, #e8520e, #d4470b)',
                  fontWeight: 700,
                  fontSize: '0.95rem',
                  color: '#fff',
                  textDecoration: 'none',
                  boxShadow: '0 8px 32px rgba(232,82,14,0.35), inset 0 1px 0 rgba(255,255,255,0.15)',
                  transition: 'all 0.3s ease',
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '8px',
                }}
              >
                Jelajahi SPA →
              </Link>"#;

            let replacement = r#"              <Link
                href="/about"
                style={{
                  padding: '16px 32px',
                  borderRadius: '16px',
                  background: 'linear-gradient(135deg, #e8520e, #d4470b)',
                  fontWeight: 700,
                  fontSize: '0.95rem',
                  color: '#fff',
                  textDecoration: 'none',
                  boxShadow: '0 8px 32px rgba(232,82,14,0.35), inset 0 1px 0 rgba(255,255,255,0.15)',
                  transition: 'all 0.3s ease',
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '8px',
                }}
              >
                Jelajahi SPA →
              </Link>
              {auth_installed && (
                <>
                  <Link
                    href="/login"
                    style={{
                      padding: '16px 32px',
                      borderRadius: '16px',
                      background: isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.04)',
                      fontWeight: 700,
                      fontSize: '0.95rem',
                      color: isDark ? '#ccc' : '#444',
                      textDecoration: 'none',
                      border: `1px solid ${isDark ? 'rgba(255,255,255,0.1)' : 'rgba(0,0,0,0.1)'}`,
                      transition: 'all 0.3s ease',
                      display: 'inline-flex',
                      alignItems: 'center',
                    }}
                  >
                    Masuk
                  </Link>
                  <Link
                    href="/register"
                    style={{
                      padding: '16px 32px',
                      borderRadius: '16px',
                      background: 'linear-gradient(135deg, #4f46e5, #4338ca)',
                      fontWeight: 700,
                      fontSize: '0.95rem',
                      color: '#fff',
                      textDecoration: 'none',
                      boxShadow: '0 8px 32px rgba(79,70,229,0.2)',
                      transition: 'all 0.3s ease',
                      display: 'inline-flex',
                      alignItems: 'center',
                    }}
                  >
                    Daftar
                  </Link>
                </>
              )}"#;

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
        if content.contains("use rustbasic_core::{Router, get, post, from_fn, AppState};") {
            content = content.replace("use rustbasic_core::{Router, get, post, from_fn, AppState};", "use rustbasic_core::{Router, get, AppState};");
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
        if !content.contains("use rustbasic_core::{Router, get, AppState};") {
            content = content.replace("use rustbasic_core::{Router, get};", "use rustbasic_core::{Router, get, AppState};");
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

    // 5. Delete Components, React Pages/Auth & Dashboard.tsx
    let components_dir = "src/resources/js/Components";
    let toast_view = "src/resources/js/Components/Toast.tsx";
    if std::path::Path::new(toast_view).exists() {
        fs::remove_file(toast_view).ok();
    }
    let alert_banner_view = "src/resources/js/Components/AlertBanner.tsx";
    if std::path::Path::new(alert_banner_view).exists() {
        fs::remove_file(alert_banner_view).ok();
    }
    let form_input_view = "src/resources/js/Components/FormInput.tsx";
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

    let dashboard_page = "src/resources/js/Pages/Dashboard.tsx";
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
        if content.contains("pub use password_resets::Entity as PasswordReset;") {
            content = content.replace("#[allow(unused_imports)]\npub use password_resets::Entity as PasswordReset;\n", "");
            content = content.replace("pub use password_resets::Entity as PasswordReset;\n", "");
            content = content.replace("pub use password_resets::Entity as PasswordReset;", "");
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

    // 7.3 Restore Welcome.tsx
    let welcome_path = "src/resources/js/Pages/Welcome.tsx";
    if let Ok(content) = fs::read_to_string(welcome_path)
        && content.contains("auth_installed &&") {
            let target = r#"              <Link
                href="/about"
                style={{
                  padding: '16px 32px',
                  borderRadius: '16px',
                  background: 'linear-gradient(135deg, #e8520e, #d4470b)',
                  fontWeight: 700,
                  fontSize: '0.95rem',
                  color: '#fff',
                  textDecoration: 'none',
                  boxShadow: '0 8px 32px rgba(232,82,14,0.35), inset 0 1px 0 rgba(255,255,255,0.15)',
                  transition: 'all 0.3s ease',
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '8px',
                }}
              >
                Jelajahi SPA →
              </Link>
              {auth_installed && (
                <>
                  <Link
                    href="/login"
                    style={{
                      padding: '16px 32px',
                      borderRadius: '16px',
                      background: isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.04)',
                      fontWeight: 700,
                      fontSize: '0.95rem',
                      color: isDark ? '#ccc' : '#444',
                      textDecoration: 'none',
                      border: `1px solid ${isDark ? 'rgba(255,255,255,0.1)' : 'rgba(0,0,0,0.1)'}`,
                      transition: 'all 0.3s ease',
                      display: 'inline-flex',
                      alignItems: 'center',
                    }}
                  >
                    Masuk
                  </Link>
                  <Link
                    href="/register"
                    style={{
                      padding: '16px 32px',
                      borderRadius: '16px',
                      background: 'linear-gradient(135deg, #4f46e5, #4338ca)',
                      fontWeight: 700,
                      fontSize: '0.95rem',
                      color: '#fff',
                      textDecoration: 'none',
                      boxShadow: '0 8px 32px rgba(79,70,229,0.2)',
                      transition: 'all 0.3s ease',
                      display: 'inline-flex',
                      alignItems: 'center',
                    }}
                  >
                    Daftar
                  </Link>
                </>
              )}"#;

            let replacement = r#"              <Link
                href="/about"
                style={{
                  padding: '16px 32px',
                  borderRadius: '16px',
                  background: 'linear-gradient(135deg, #e8520e, #d4470b)',
                  fontWeight: 700,
                  fontSize: '0.95rem',
                  color: '#fff',
                  textDecoration: 'none',
                  boxShadow: '0 8px 32px rgba(232,82,14,0.35), inset 0 1px 0 rgba(255,255,255,0.15)',
                  transition: 'all 0.3s ease',
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '8px',
                }}
              >
                Jelajahi SPA →
              </Link>"#;

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

    sqlx::any::install_default_drivers();
    if let Ok(pool) = sqlx::AnyPool::connect(&db_url).await {
        let sql = "DELETE FROM migration_history WHERE version LIKE '%_create_password_resets_table'";
        let _ = sqlx::query(sql).execute(&pool).await;
        println!("   {} {}", "✅ Cleaned:".green(), "Database migration records removed.".cyan());
    }


    // Clean up email files
    let test_email_path = "src/bin/send_test_email.rs";
    if std::path::Path::new(test_email_path).exists() {
        fs::remove_file(test_email_path).ok();
        println!("   {} {}", "✅ Deleted:".green(), test_email_path.cyan());
    }
    let email_views_dir = "src/resources/views/emails";
    if std::path::Path::new(email_views_dir).exists() {
        fs::remove_dir_all(email_views_dir).ok();
        println!("   {} {}", "✅ Deleted:".green(), email_views_dir.cyan());
    }

    println!("\n{}", "✨ Authentication removed successfully!".green().bold());
}
