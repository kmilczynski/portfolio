# Portfolio

Personal portfolio website built with Rust, Perseus, and Sycamore, featuring a built-in CMS for blog management.

## Stack

- **Perseus** - Static site generation and server-side rendering
- **Sycamore** - Reactive UI components
- **Tailwind CSS** - Styling
- **Axum** - Web framework for API and admin panel
- **SQLx** - Database access with SQLite
- **Fluent** - Internationalization (i18n)

## Features

- 🌐 Multilingual support (English/Polish)
- 📝 Admin CMS for blog post management
- 🗄️ SQLite database for dynamic content
- 🎨 Dark theme with custom animations
- 🐳 Docker support for easy deployment

## Development Setup

### Prerequisites

- Rust (latest stable)
- Node.js (for Tailwind CSS)
- Perseus CLI
- SQLite

### Installation

```bash
# Install Perseus CLI
cargo install perseus-cli --version 0.4.2

# Install dependencies
npm install

# Set up environment variables
cp .env.example .env
# Edit .env and set ADMIN_PASSWORD

# Create database directory
mkdir data

# Watch CSS changes (in one terminal)
npm run watch-css

# Run development server (in another terminal)
perseus serve
```

### Building for Production

```bash
# Build minified CSS
npm run prod

# Build Perseus app
perseus deploy
```

## Configuration

### Content Files

Edit files in `src/content/` directory:

- `site-en.toml` / `site-pl.toml` - Personal info, skills, experience (localized)
- `projects-en.toml` / `projects-pl.toml` - Project entries (localized)

### Blog Posts

Blog posts are managed through the **Admin CMS** (not markdown files):

1. Navigate to `/admin` (requires authentication)
2. Use the admin panel to create, edit, and delete blog posts
3. Posts are stored in SQLite database (`data/portfolio.db`)

### Environment Variables

- `ADMIN_PASSWORD` - Password for admin panel access
- `DATABASE_URL` - SQLite database path (default: `sqlite:data/portfolio.db`)
- `RUST_LOG` - Log level (default: `info`)

## Project Structure

```
├── src/
│   ├── admin/          # Admin panel and CMS handlers
│   ├── components/     # Reusable UI components
│   ├── content/        # Static content (TOML files)
│   ├── models/         # Data models
│   └── templates/      # Page templates
├── static/
│   ├── css/            # Compiled CSS
│   └── img/            # Images and assets
├── style/
│   └── main.css        # Source CSS (Tailwind)
├── data/               # SQLite database (gitignored)
├── Dockerfile          # Multi-stage Docker build
├── docker-compose.yml  # Docker Compose configuration
└── package.json        # Node.js dependencies (Tailwind)
```

## Admin Panel

Access the admin panel at `/admin` to:

- Create and publish blog posts
- Edit existing posts with markdown support
- Delete posts
- Preview posts before publishing

Authentication required - use the password set in `.env`

## Internationalization

The site supports multiple languages using Fluent. Language can be switched via the navigation menu. Content is localized in:

- `src/content/site-{locale}.toml`
- `src/content/projects-{locale}.toml`
- Translations stored in translation files

## License

ISC
