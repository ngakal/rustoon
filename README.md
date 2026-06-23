# 🦀 RustToon — Webtoon Reader

A mobile-first webtoon reader built with **Rust + Dioxus 0.7**.

## Features
- 📱 Mobile-first fluent UI with bottom navigation
- 🔍 Search manga/manhwa via AniList API
- 📖 Trending, Korean Manhwa, Popular sections
- 🔖 Bookmarks & reading history (local storage)
- 🎨 Dark theme with smooth animations
- 🦀 Full Rust stack — WASM frontend, no JS framework

## Build from source

### Web
```bash
cargo install dioxus-cli
cd crates/frontend
dx build --platform web --release
# Output: target/dx/rustoon-frontend/release/web/public/
```

### Android APK
```bash
# Prerequisites: Android SDK + NDK, JDK 17+
export ANDROID_HOME=~/android-sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/27.0.12077973
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

cd crates/frontend
dx build --platform android --release
# Output: target/dx/rustoon-frontend/release/android/*.apk
```

## Deploy to GitHub Pages
```bash
# Optional local verification before pushing
cargo install dioxus-cli
cd crates/frontend
dx build --platform web --release
```

After that:
- Push to `main`
- In GitHub repository settings, open `Pages`
- Set the source to `GitHub Actions`
- The workflow `.github/workflows/deploy-pages.yml` will publish the web build

Notes:
- The Pages site URL will usually be `https://<user>.github.io/<repo>/`
- The app uses a relative stylesheet path so it can load correctly from a GitHub Pages project site

## Architecture
```
rustoon/
├── crates/
│   ├── shared/          # Shared models (AniList API types, app state)
│   │   └── src/models.rs
│   └── frontend/        # Dioxus WASM app
│       └── src/
│           ├── main.rs
│           ├── app.rs        # Root component
│           ├── anilist.rs    # AniList GraphQL client
│           ├── state.rs      # Global state (Signals)
│           ├── icons.rs      # Inline SVG icons
│           ├── components/   # Reusable UI components
│           └── pages/        # Page components
├── .github/workflows/
│   └── build-apk.yml   # GitHub Actions: auto-build APK
└── Cargo.toml           # Workspace root
```

## Tech Stack
- **Frontend**: Dioxus 0.7 (Rust → WASM)
- **API**: AniList GraphQL
- **Storage**: browser localStorage
- **Icons**: Inline SVG (Lucide-style)
