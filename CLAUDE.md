# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a WWE Universe Manager application built with:
- **Backend**: Rust using Tauri 2.0 framework for desktop app functionality
- **Frontend**: Leptos (Rust-based web framework) compiled to WebAssembly
- **Database**: SQLite with Diesel ORM for data persistence
- **Styling**: Tailwind CSS v4 with DaisyUI components and custom WWE theme
- **UI Theme**: Custom WWE-themed interface with red/yellow/black color scheme

The project is structured as a Rust workspace with two main packages:
- `wwe-universe-manager` (backend, located in `src-tauri/`)
- `wwe-universe-manager-ui` (frontend, located in root directory)

## Development Commands

### Initial Setup
```bash
# Install dependencies
npm install

# Setup environment and database
echo "DATABASE_URL=database.db" > .env
diesel setup
diesel migration run

# Build CSS (required before running app)
npm run build-css-prod
```

### Running the Application

#### Development Mode (Recommended)
Run these commands in separate terminals for the best development experience:

```bash
# Terminal 1: Start CSS watcher (rebuilds styles automatically)
npm run build-css

# Terminal 2: Start Tauri development server
npm run dev
# OR alternatively: cargo tauri dev
```

The CSS watcher will automatically rebuild styles when you modify Tailwind classes in your Rust components.

#### Alternative Development Options
```bash
# Option 1: Build CSS once, then run app
npm run build-css-prod && npm run dev

# Option 2: Use Tauri directly (rebuild CSS manually when needed)
cargo tauri dev

# Option 3: Frontend-only development (without Tauri backend)
trunk serve
```

#### Production Build
```bash
# Build optimized CSS
npm run build-css-prod

# Build Tauri application
npm run build
# OR: cargo tauri build
```

### Testing
```bash
# Run all tests
cargo test --workspace

# Run specific package tests
cargo test -p wwe-universe-manager-ui      # Frontend tests
cargo test -p wwe-universe-manager         # Backend tests

# Run specific test suites
cargo test -p wwe-universe-manager --test users        # User tests only
cargo test -p wwe-universe-manager --test wrestlers    # Wrestler tests only
cargo test -p wwe-universe-manager --test titles       # Title tests only
cargo test -p wwe-universe-manager --test shows        # Show tests only
cargo test -p wwe-universe-manager --test integration  # Integration tests only
```

### Database Operations
```bash
# Create new migration
diesel migration generate <migration_name>

# Run migrations
diesel migration run

# Revert last migration
diesel migration revert
```

## Architecture

### Backend Structure (`src-tauri/src/`)
- `lib.rs` - Main application entry point and Tauri command registration
- `db.rs` - Database connection pooling (r2d2) and Tauri commands
- `models/` - Data models for User, Wrestler, Title, Show entities
- `schema.rs` - Diesel-generated database schema (auto-generated)
- `auth.rs` - Authentication utilities (Argon2 password hashing)

### Frontend Structure (`src/`)
- `main.rs` - WASM entry point
- `app.rs` - Main Leptos components with WWE-themed UI and Tauri bindings

### Styling Structure
- `style/tailwind.css` - Tailwind CSS entry point with DaisyUI plugin
- `style/output.css` - Generated CSS file (auto-generated, do not edit)
- `tailwind.config.js` - Tailwind configuration with custom WWE theme
- `index.html` - Main HTML template with WWE theme applied

### Database Models
The application manages four main entities:
- **Users** - Application users with authentication
- **Wrestlers** - WWE wrestler profiles
- **Titles** - Championship belts/titles
- **Shows** - WWE shows/events

### Key Patterns
- Database operations use connection pooling (r2d2) for concurrent access
- Tauri commands provide bridge between frontend and backend
- Frontend uses Leptos signals for reactive state management
- All database operations are wrapped in internal functions for testability
- Tests use `serial_test` to prevent concurrent database access issues
- CSS is built using Tailwind CLI with automatic watching in development
- Custom WWE theme provides consistent branding across all components

### Tauri Commands
Currently exposed commands:
- `get_shows` - Fetch all shows
- `create_show` - Create new show
- `create_user` - Create new user
- `create_wrestler` - Create new wrestler
- `create_belt` - Create new title/belt

## Testing Notes
- Backend tests require SQLite database setup with migrations applied
- Use `serial_test` attribute for database-dependent tests to prevent race conditions
- Test framework includes comprehensive test helpers in `test_helpers.rs`
- Tests are organized by entity type: `users.rs`, `wrestlers.rs`, `titles.rs`, `shows.rs`
- Integration tests cover cross-entity scenarios in `integration.rs`
- All tests include proper cleanup to prevent test pollution
- Database timestamps are nullable (Option<NaiveDateTime>) due to SQLite ALTER TABLE limitations
- Foreign key constraints may not be enforced by default in SQLite
- Run tests with: `cargo test --workspace`