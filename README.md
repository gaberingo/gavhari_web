```bash
gavhari_web/
│
├── frontend/                # Aplikasi Yew
│   ├── src/
│   │   ├── components/      # Komponen UI Yew
│   │   ├── pages/           # Halaman-halaman utama
│   │   ├── router.rs        # Routing aplikasi
│   │   ├── main.rs          # Entry point aplikasi Yew
│   │   └── lib.rs
│   ├── static/              # File statis (favicon, gambar, manifest, dll.)
│   ├── tailwind.config.js   # Konfigurasi Tailwind + DaisyUI
│   ├── index.html
│   ├── Trunk.toml           # Konfigurasi Trunk (bundler untuk Yew)
│   └── Cargo.toml
│
├── backend/                 # Backend Actix + SeaORM
│   ├── src/
│   │   ├── main.rs          # Entry point server
│   │   ├── api/             # Handler HTTP / route group
│   │   ├── db/              # Setup database & SeaORM
│   │   │   ├── models/      # Model SeaORM
│   │   │   ├── entities/    # Hasil generate sea-orm-cli
│   │   │   └── mod.rs
│   │   ├── config.rs        # Konfigurasi (env, dsb)
│   │   └── state.rs         # AppState / Data Shared (DB pool dsb)
│   ├── migrations/          # Migrasi database (SeaORM CLI)
│   └── Cargo.toml
│
├── shared/                  # Crate bersama (jika ada data/struct dipakai di frontend & backend)
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
│
├── Cargo.toml               # Workspace root
└── Cargo.lock
```
