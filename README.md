# WWE Universe Manager

<div align="center">
  <img src="Example.gif" alt="WWE Universe Manager Interface" width="800" />

  <p><em>The Ultimate Wrestling Management Experience</em></p>
</div>

A comprehensive WWE Universe management application built entirely in Rust, featuring a custom WWE-themed interface that brings the excitement of professional wrestling to your desktop.

## ✨ Features

### 🏆 Core Management System
- **Multi-Window Desktop App**: Native windows for wrestlers, titles, and shows with state preservation
- **Global Wrestler Pool**: Wrestlers available across all shows with detailed profiles and power ratings
- **Show Roster Management**: Assign wrestlers to specific shows with many-to-many relationships
- **Match Booking System**: Create matches with participants, winners, and title implications
- **Championship Management**: Global title pool with holder tracking and prestige tiers
- **Enhanced Wrestler Profiles**: Power ratings (strength, speed, agility, stamina, charisma, technique)

### 🎨 User Experience
- **WWE-Themed Interface**: Custom slate/modern dark theme with WWE branding
- **Modular Component Architecture**: 31+ specialized UI components
- **Responsive Multi-Window Design**: CEO dashboard, Booker interface, detailed management windows
- **Real-time State Management**: Leptos reactive signals with proper async handling

### ⚡ Technical Excellence
- **Native Performance**: Tauri 2.0 with sub-50ms database operations
- **Memory Safety**: Zero memory leaks with Rust ownership system
- **Type Safety**: Compile-time query checking with Diesel ORM
- **Connection Pooling**: r2d2 for efficient database resource management
- **Comprehensive Testing**: 25+ tests with full CI/CD coverage

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
   # Fast setup: Only 5 consolidated migrations with proper separation of concerns for optimal developer experience
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

## 📚 Documentation

### 📖 Complete Documentation

For comprehensive documentation, including API references, component guides, and architecture details:

**[🔗 View Full Documentation](docs/introduction.mdx)** (Mintlify-powered)

To run the documentation locally:

```bash
cd docs
mintlify dev
# Visit http://localhost:3000
```

### 📋 Quick Reference

- **[Getting Started Guide](docs/quickstart.mdx)** - 5-minute setup
- **[Development Guide](docs/development.mdx)** - Contributing and development workflow
- **[API Reference](docs/api-reference/introduction.mdx)** - Complete Tauri command documentation
- **[Architecture Overview](docs/architecture/overview.mdx)** - System design and patterns

## 🏗️ Project Architecture

### Technology Stack

- **Frontend**: Leptos (Rust → WebAssembly)
- **Backend**: Tauri 2.0 with Rust
- **Database**: SQLite with Diesel ORM
- **Styling**: Tailwind CSS v4 + DaisyUI
- **Build**: Cargo workspace + npm scripts
- **Documentation**: Mintlify

### Project Structure

```
wwe-universe-manager/
├── src/                    # Frontend (Leptos)
│   ├── components/         # Modular UI components
│   │   ├── create_show.rs  # Show creation form
│   │   ├── show_selector.rs # Show selection interface
│   │   └── mod.rs          # Component exports
│   ├── types.rs           # Shared types and API functions
│   ├── app.rs             # Main application component
│   └── main.rs            # WASM entry point
├── src-tauri/             # Backend (Tauri)
│   ├── src/
│   │   ├── db.rs          # Database operations & Tauri commands
│   │   ├── models/        # Data models (Show, Wrestler, Title, User)
│   │   ├── auth.rs        # Authentication utilities
│   │   ├── schema.rs      # Database schema (auto-generated)
│   │   └── lib.rs         # Main entry point
│   ├── tests/             # Comprehensive test suite
│   └── migrations/        # Database migrations
├── docs/                  # Mintlify documentation
├── style/                 # Tailwind CSS configuration
└── public/               # Static assets
```

### Component Architecture

The frontend features a sophisticated modular component system:

- **`src/app.rs`**: Main App component with URL-based routing and window management
- **`src/components/`**: Specialized domain modules (31+ components)
  - **`wrestler/`**: Championship tracking, power ratings management
  - **`title/`**: Champion info, history, holder changes
  - **`show/`**: Match creation, roster management, participant assignment
  - **`booker_dashboard.rs`**: Match booking interface with show-filtered wrestlers
  - **`ceo_dashboard.rs`**: High-level promotion management (removed in favor of simplified shows)
- **`src/services/`**: API service layer for wrestler operations
- **`src/utils/`**: Error handling, URL parsing, and custom hooks
- **`src/constants/`**: UI constants and theming configuration

### Database Schema

Current entities managed by the application:

| Entity        | Description               | Key Features                       |
| ------------- | ------------------------- | ---------------------------------- |
| **Shows**     | Wrestling programs/brands | Name, description, timestamps      |
| **Wrestlers** | Performer profiles        | Name, gender, win/loss records     |
| **Titles**    | Championship belts        | Name, current holder tracking      |
| **Users**     | Application users         | Authentication with Argon2 hashing |

## 🛠️ Development Workflow

### Prerequisites for Development

- **Rust** (latest stable) - [Install via rustup](https://rustup.rs/)
- **Node.js** 18+ - [Install from nodejs.org](https://nodejs.org/)
- **Diesel CLI** - `cargo install diesel_cli --no-default-features --features sqlite`
- **Mintlify CLI** (for docs) - `npm install -g mintlify`

### Setting Up Your Development Environment

1. **Clone and setup**

   ```bash
   git clone <your-repo-url>
   cd wwe-universe-manager
   npm install
   echo "DATABASE_URL=database.db" > .env
   diesel setup && diesel migration run
   ```

2. **Development servers** (run in separate terminals)

   ```bash
   # Terminal 1: CSS watcher (auto-rebuilds on changes)
   npm run build-css

   # Terminal 2: Tauri development server (hot reload)
   npm run dev

   # Terminal 3 (optional): Documentation server
   cd docs && mintlify dev
   ```

### Making Changes

#### Frontend Development (Leptos/WASM)

- **Components**: Add new components in `src/components/`
- **Types**: Update shared types in `src/types.rs`
- **Styling**: Use Tailwind classes with WWE theme colors
- **State**: Use Leptos signals for reactive state management

#### Backend Development (Tauri/Rust)

- **Database**: Create migrations with `diesel migration generate name`
- **Models**: Add new models in `src-tauri/src/models/`
- **API**: Add Tauri commands in `src-tauri/src/db.rs`
- **Tests**: Add tests in `src-tauri/tests/`

#### Adding a New Feature (Example: Wrestler Management)

1. **Database Migration**

   ```bash
   diesel migration generate create_wrestlers
   # Edit the generated SQL files
   diesel migration run
   ```

2. **Create Models**

   ```rust
   // src-tauri/src/models/wrestler.rs
   #[derive(Serialize, Deserialize)]
   pub struct Wrestler {
       pub id: i32,
       pub name: String,
       // ...
   }
   ```

3. **Add Tauri Commands**

   ```rust
   // src-tauri/src/db.rs
   #[tauri::command]
   pub fn create_wrestler(state: State<DbState>, data: WrestlerData) -> Result<Wrestler, String> {
       // Implementation
   }
   ```

4. **Create Frontend Component**

   ```rust
   // src/components/wrestler_form.rs
   #[component]
   pub fn WrestlerForm() -> impl IntoView {
       // Component implementation
   }
   ```

5. **Add Tests**
   ```rust
   // src-tauri/tests/wrestlers.rs
   #[test]
   fn test_create_wrestler() {
       // Test implementation
   }
   ```

### Code Style Guidelines

#### Rust Code

- Use `rustfmt` for formatting: `cargo fmt`
- Follow Rust naming conventions (snake_case, PascalCase)
- Add documentation comments for public APIs
- Use `Result<T, E>` for error handling
- Prefer `&str` over `String` for function parameters

#### Frontend (Leptos)

- Keep components small and focused
- Use descriptive prop names
- Handle loading and error states
- Follow the existing WWE theming patterns

#### Database

- Use meaningful migration names
- Include both `up.sql` and `down.sql`
- Add indexes for frequently queried columns
- Use foreign key constraints where appropriate

### Testing Guidelines

```bash
# Run all tests before committing
cargo test --workspace

# Run specific test categories
cargo test -p wwe-universe-manager --test shows
cargo test -p wwe-universe-manager --test integration

# Check code formatting and linting
cargo fmt --check
cargo clippy -- -D warnings
```

### API Reference

The application exposes these Tauri commands:

| Command           | Purpose             | Parameters                                    | Returns     |
| ----------------- | ------------------- | --------------------------------------------- | ----------- |
| `get_shows`       | Fetch all shows     | `state: DbState`                              | `Vec<Show>` |
| `create_show`     | Create new show     | `state: DbState, show_data: ShowData`         | `Show`      |
| `create_wrestler` | Create wrestler     | `state: DbState, wrestler_data: WrestlerData` | `Wrestler`  |
| `create_belt`     | Create championship | `state: DbState, title_data: TitleData`       | `Title`     |
| `create_user`     | Create user         | `state: DbState, user_data: UserData`         | `User`      |

**Example API Usage (Frontend)**:

```rust
// src/types.rs - API wrapper functions
pub async fn create_show(show_data: ShowData) -> Result<Show, String> {
    let args = serde_json::json!({ "showData": show_data });
    let args_value = serde_wasm_bindgen::to_value(&args)?;
    let result_js = invoke("create_show", args_value).await;
    serde_wasm_bindgen::from_value(result_js)
}
```

### Build System & Scripts

The project uses a combination of Cargo and npm scripts:

```bash
# CSS Development
npm run build-css          # Watch mode (auto-rebuild)
npm run build-css-prod     # Production build (minified)

# Application Development
npm run dev                # Start Tauri dev server
cargo tauri dev            # Alternative Tauri command

# Frontend Only (without Tauri)
trunk serve               # Leptos frontend only

# Production Builds
npm run build             # Full production build
cargo tauri build         # Tauri production build

# Testing & Quality
cargo test --workspace    # Run all tests
cargo fmt                 # Format code
cargo clippy              # Lint code

# Documentation
cd docs && mintlify dev   # Start documentation server
```

### Database Management

```bash
# Create new migration
diesel migration generate migration_name

# Apply migrations
diesel migration run

# Revert last migration
diesel migration revert

# Reset database (caution: destroys data)
diesel database reset
```

## 📈 Development Status

✅ **Completed**

- Tauri 2.0 migration with Rust backend
- Leptos frontend with WebAssembly compilation
- Modular component architecture
- WWE-themed responsive UI with Tailwind CSS
- SQLite database with Diesel ORM
- Comprehensive testing framework (25+ tests)
- Database migrations and schema management
- Mintlify documentation system
- Show management with create/read operations
- Form validation and error handling
- Component-based architecture with separation of concerns

🔄 **In Progress**

- Advanced wrestler statistics and management
- Championship title tracking and history
- Show scheduling and calendar system
- User authentication and session management

📋 **Planned**

### 📖 Storyline & Universe Management
- **Storyline Management System**: Create and track multi-show storylines with branching narratives
- **Rivalry Tracking**: Automatic rivalry detection and intensity scoring based on match history
- **Weekly Time Advancement**: Progress through wrestling calendar with automatic event scheduling
- **Random Events**: Surprise injuries, backstage incidents, and unexpected developments

### 🎯 Advanced Management Features  
- **Drafting Mode**: Interactive draft system where shows take turns selecting wrestlers
- **Wrestler Demands**: Contract negotiations, pay raise requests, and booking complaints
- **Multi-Brand Universe Support**: Manage multiple promotions with cross-promotional events
- **Advanced Analytics**: Performance metrics, popularity tracking, and financial reports

### 🔧 Quality of Life
- **Import/Export Functionality**: Backup and share universe saves
- **Custom Match Types**: Ladder matches, cage matches, royal rumbles
- **Automated Booking AI**: Smart match suggestions based on storylines and rivalries

## 🤝 Contributing

We welcome contributions! Please follow these steps:

### Quick Contribution Steps

1. **Fork** the repository on GitHub
2. **Clone** your fork: `git clone <your-fork-url>`
3. **Create** a feature branch: `git checkout -b feature/amazing-feature`
4. **Setup** development environment (see above)
5. **Make** your changes following our guidelines
6. **Test** thoroughly: `cargo test --workspace`
7. **Format** code: `cargo fmt`
8. **Lint** code: `cargo clippy`
9. **Commit** with clear messages: `git commit -m 'Add amazing feature'`
10. **Push** to your fork: `git push origin feature/amazing-feature`
11. **Create** a Pull Request with detailed description

### What We Look For

- ✅ Clear, descriptive commit messages
- ✅ Comprehensive tests for new features
- ✅ Updated documentation (if applicable)
- ✅ Follows existing code style and patterns
- ✅ No breaking changes (or clearly documented)

### Need Help?

- 📖 Check the [Development Guide](docs/development.mdx)
- 🔍 Browse existing [issues and discussions](https://github.com/your-username/wwe-universe-manager/issues)
- 💬 Open an issue for questions or suggestions

## 📞 Support & Community

- **🐛 Bug Reports**: [GitHub Issues](https://github.com/your-username/wwe-universe-manager/issues)
- **💡 Feature Requests**: [GitHub Discussions](https://github.com/your-username/wwe-universe-manager/discussions)
- **📚 Documentation**: [Mintlify Docs](docs/introduction.mdx)
- **🔧 Development**: [Contributing Guide](#-contributing)
