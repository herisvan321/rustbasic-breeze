# RustBasic Breeze

**Authentication scaffolding for the RustBasic Framework.**

RustBasic Breeze provides a simple and fast way to set up a complete authentication system in your RustBasic applications. It automatically scaffolds all the necessary routes, controllers, models, and views (React/Inertia.js components) required for user authentication.

## Features

- **Login & Registration**: Ready-to-use authentication views and backend logic.
- **Password Reset**: Complete forgot password and reset password flow, including email template scaffolding and token management.
- **Session-based Authentication**: Secure session management using `rustbasic-core`.
- **Database Models**: Automatically generates SeaORM models and migrations for users and password resets.
- **Modern UI**: Scaffolds beautiful, responsive React components using Inertia.js and Tailwind CSS (or standard CSS).

## Installation

Run the following command in your project directory:

```bash
cargo add rustbasic-breeze
```

## Usage

You can use the scaffolding function in your CLI or setup scripts.

```rust
use rustbasic_breeze::make_auth;

#[tokio::main]
async fn main() {
    // Scaffold authentication files, routes, and views
    make_auth().await;
}
```

Running `make_auth()` will automatically generate:
- Authentication routes (`src/routes/auth.rs`)
- Authentication middleware (`src/app/http/middleware/auth.rs`)
- `AuthController` for handling login, registration, and password resets
- `password_resets` SeaORM model and database migration
- React components for the frontend (`src/resources/js/Pages/Auth/*.jsx`)

After scaffolding, make sure to run your database migrations so the `password_resets` table is created.

## License

This project is open-sourced software licensed under the [MIT license](https://opensource.org/licenses/MIT).
