# WWE Universe Manager

<div align="center">
  <img src="Example.png" alt="WWE Universe Manager Interface" width="800" />

  <p><em>The Ultimate Wrestling Management Experience</em></p>
</div>

A comprehensive WWE Universe management application built entirely in Rust, featuring a custom WWE-themed interface that brings the excitement of professional wrestling to your desktop.

## ✨ Features

- **WWE-Themed Interface**: Custom red/yellow/black color scheme with championship-style design
- **Complete Roster Management**: Create and manage wrestlers with detailed stats
- **Show Creation**: Design and organize your wrestling shows
- **Championship System**: Create and manage championship titles
- **Desktop App**: Built with Tauri 2.0 for native performance
- **Responsive Design**: Adapts to any window size
- **Rust Throughout**: Frontend (Leptos) and backend both written in Rust

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (for Tailwind CSS)
- [Diesel CLI](https://diesel.rs/guides/getting-started) (`cargo install diesel_cli --no-default-features --features sqlite`)
- SQLite3

### Installation & Setup

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd wwe-universe-manager
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Setup database**
   ```bash
   echo "DATABASE_URL=database.db" > .env
   diesel setup
   diesel migration run
   ```

4. **Build CSS and run the app**
   ```bash
   # Terminal 1: Start CSS watcher (rebuilds styles automatically)
   npm run build-css

   # Terminal 2: Start the application
   npm run dev
   ```

### Alternative Quick Start
```bash
# One-time setup
npm install
echo "DATABASE_URL=database.db" > .env
diesel setup && diesel migration run

# Build and run
npm run build-css-prod && npm run dev
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test --workspace

# Run specific package tests
cargo test -p wwe-universe-manager      # Backend tests
cargo test -p wwe-universe-manager-ui   # Frontend tests

# Run specific test suites
cargo test -p wwe-universe-manager --test users        # User management tests
cargo test -p wwe-universe-manager --test wrestlers    # Wrestler management tests
cargo test -p wwe-universe-manager --test titles       # Championship tests
cargo test -p wwe-universe-manager --test shows        # Show management tests
cargo test -p wwe-universe-manager --test integration  # Cross-system tests
```

## 🏗️ Project Architecture

### Technology Stack
- **Frontend**: Leptos (Rust → WebAssembly)
- **Backend**: Tauri 2.0 with Rust
- **Database**: SQLite with Diesel ORM
- **Styling**: Tailwind CSS v4 + DaisyUI
- **Build**: Cargo workspace + npm scripts

### Project Structure
```
wwe-universe-manager/
├── src/                    # Frontend (Leptos)
│   ├── app.rs             # Main UI components
│   └── main.rs            # WASM entry point
├── src-tauri/             # Backend (Tauri)
│   ├── src/
│   │   ├── db.rs          # Database operations
│   │   ├── models/        # Data models
│   │   └── auth.rs        # Authentication
│   └── tests/             # Comprehensive test suite
├── style/                 # CSS and styling
├── migrations/            # Database migrations
└── public/               # Static assets
```

## 📈 Development Status

✅ **Completed**
- Tauri 2.0 migration
- Leptos frontend implementation
- WWE-themed responsive UI
- Database architecture with migrations
- Comprehensive testing framework
- Build system optimization

🔄 **In Progress**
- Advanced wrestler statistics
- Show scheduling system
- Championship tracking features

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test --workspace`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request
