# 🔐 Panduan Autentikasi Breeze

## 📝 Kata Pengantar
Selamat datang di panduan resmi **Breeze Authentication Scaffolding**. Dokumentasi ini dirancang khusus untuk memandu pengembang mengaktifkan dan mengelola modul autentikasi terintegrasi secara instan pada proyek RustBasic Anda. Breeze secara otomatis menyusun controller backend, middleware pelindung, skema migrasi reset token database, serta halaman visual premium React (SPA) di sisi frontend.

---

## 🛠️ Script Contoh

### A. Penambahan Breeze ke Bagian dependencies (`Cargo.toml`)
```toml
[dependencies]
rustbasic-core = "0.1"
rustbasic-breeze = "0.0"
```

### B. Hashing Password Pengguna dengan Bcrypt
```rust
use rustbasic_core::bcrypt::{hash, DEFAULT_COST};

// Hashing password secara aman di controller registrasi
let password_hash = hash("user_password_rahasia", DEFAULT_COST).unwrap();
```

### C. Proteksi Rute Melalui Auth Middleware (`src/routes/web.rs`)
```rust
use rustbasic_core::{Router, get, from_fn, AppState};
use crate::app::http::controllers::dashboard_controller;
use crate::app::http::middleware::auth::auth_middleware;

pub fn router() -> Router<AppState> {
    Router::new()
        // Hanya user yang telah login yang boleh mengakses dashboard
        .route("/dashboard", get(dashboard_controller::index))
        .layer(from_fn(auth_middleware))
}
```

---

## 🔄 Perbandingan Pemakaian (Autentikasi Manual vs Scaffolding Breeze)

Berikut adalah perbandingan pemakaian pengembangan sistem keamanan autentikasi:

| Aspek Implementasi | Membangun Sistem Manual | Menggunakan Scaffolding Breeze |
| :--- | :--- | :--- |
| **Waktu Pengerjaan** | Memakan waktu berhari-hari untuk logika & UI. | Instan (tergenerasi otomatis saat startup). |
| **Keamanan Cookie Sesi** | Harus mengkonfigurasi enkripsi secara manual. | Sudah otomatis terintegrasi dengan session secure. |
| **Alur Lupa Password** | Harus mendesain database token & kirim email sendiri. | Terintegrasi langsung dengan mail transport Lettre. |
| **UI Halaman** | Harus mendesain layout login/register sendiri. | Disediakan halaman visual premium split-screen. |

---

## 📊 Tabel Ringkasan Berkas Tergenerasi

Berikut adalah daftar berkas yang disalin otomatis ke struktur folder lokal proyek Anda oleh Breeze:

| Berkas Terbuat | Lokasi Berkas di Proyek | Deskripsi Fungsi |
| :--- | :--- | :--- |
| **AuthController** | `src/app/http/controllers/auth/auth_controller.rs` | Memproses aksi login, pendaftaran, logout, & reset password. |
| **AuthMiddleware** | `src/app/http/middleware/auth.rs` | Membelokkan akses tamu tak dikenal kembali ke halaman `/login`. |
| **Rute Autentikasi** | `src/routes/auth.rs` | Menyediakan URL jalur login, register, lupa sandi, & ubah password. |
| **Halaman React** | `src/resources/js/Pages/Auth/` | Halaman visual React SPA (Login, Register, ResetPassword, dll). |
| **Template Email** | `src/resources/views/emails/` | Template email HTML premium untuk selamat datang & link reset sandi. |

---

## 🏁 Penutup
Dengan mengaktifkan modul autentikasi Breeze, Anda memiliki sistem otentikasi siap pakai berkinerja tinggi, aman terhadap ancaman celah keamanan bypass, serta menyajikan pengalaman visual premium yang sangat memuaskan bagi pengguna akhir aplikasi Anda.
