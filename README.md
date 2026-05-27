# 🔐 RustBasic Breeze

## 📝 Kata Pengantar

Selamat datang di **RustBasic Breeze**. Paket ini menyediakan solusi *authentication scaffolding* instan yang aman dan premium untuk aplikasi berbasis **RustBasic Framework**. Breeze secara otomatis menyusun dan mengintegrasikan seluruh komponen backend (rute auth, middleware session, model database, migrasi tabel token reset) dan komponen visual frontend React (Login, Register, Dashboard, ResetPassword) ke dalam struktur proyek lokal Anda dalam hitungan detik.

---

## 🛠️ Script Contoh

### A. Penambahan Dependensi (`Cargo.toml`)
```toml
[dependencies]
rustbasic-core = "0.1"
rustbasic-breeze = "0.0"
```

### B. Inisiasi Scaffolding Autentikasi dalam Kode Rust (`src/main.rs` / CLI Utility)
```rust
use rustbasic_breeze::make_auth;

#[tokio::main]
async fn main() {
    // Menjalankan perintah pembuatan file template login & register secara otomatis
    make_auth().await;
}
```

### C. Proteksi Rute dengan Middleware Autentikasi (`src/routes/web.rs`)
```rust
use rustbasic_core::{Router, get, from_fn, AppState};
use crate::app::http::controllers::dashboard_controller;
use crate::app::http::middleware::auth::auth_middleware;

pub fn router() -> Router<AppState> {
    Router::new()
        // Mengamankan halaman dashboard dari akses tamu yang belum login
        .route("/dashboard", get(dashboard_controller::index))
        .layer(from_fn(auth_middleware))
}
```

---

## 🔄 Perbandingan Pemakaian (Autentikasi Manual vs Scaffolding Breeze)

Berikut adalah perbandingan pemakaian dan kecepatan implementasi sistem keamanan akun pada RustBasic:

| Parameter Evaluasi | Membangun Sistem Secara Manual | Menggunakan Scaffolding Breeze |
| :--- | :--- | :--- |
| **Waktu Pengerjaan** | Membutuhkan waktu berhari-hari untuk merancang model, controller, & UI. | Instan, file langsung disalin otomatis ke struktur folder proyek. |
| **Konsistensi Struktur** | Berisiko tidak konsisten dan menyulitkan kolaborasi tim. | Sangat rapi, mengikuti standar konvensi direktori RustBasic. |
| **Fitur Reset Password** | Harus membuat UUID token, tabel verifikasi, & kirim email sendiri. | Sudah siap pakai beserta template email HTML & sistem Lettre SMTP. |
| **UI Halaman Frontend** | Harus mendesain formulir masuk & daftar sendiri dari awal. | Disediakan halaman login split-screen modern & responsif. |

---

## 📊 Tabel Ringkasan Komponen yang Dihasilkan

Berikut adalah daftar berkas utama yang disalin otomatis ke folder proyek Anda setelah menjalankan modul Breeze:

| Nama Berkas / Komponen | Lokasi Penyimpanan Berkas | Deskripsi Fungsi & Kegunaan |
| :--- | :--- | :--- |
| **AuthController** | `src/app/http/controllers/auth/auth_controller.rs` | Mengolah input pendaftaran, verifikasi login, logout, & reset password. |
| **AuthMiddleware** | `src/app/http/middleware/auth.rs` | Membelokkan akses tamu tak dikenal kembali ke halaman `/login`. |
| **Rute Autentikasi** | `src/routes/auth.rs` | Rute URL web penanganan form masuk, daftar, & lupa sandi. |
| **Halaman React (SPA)** | `src/resources/js/Pages/Auth/` | Kumpulan file UI React (.jsx) halaman login, register, & reset sandi. |
| **Template Email HTML** | `src/resources/views/emails/` | Template email premium untuk tautan reset kata sandi & ucapan selamat datang. |

---

## 🏁 Penutup

Dengan mengaktifkan modul autentikasi **RustBasic Breeze**, Anda mendapatkan fondasi keamanan sistem akun yang teruji secara industri, terlindung dari eksploitasi celah CSRF, serta siap melayani pengguna dengan tampilan visual yang sangat premium. Untuk panduan lengkap dalam Bahasa Indonesia, silakan merujuk ke berkas panduan internal di: [Panduan Autentikasi Breeze](docs/auth_cli.md).
