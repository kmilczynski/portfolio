# Portfolio

Personal portfolio website built with Rust, Perseus, and Sycamore.

## Stack

- **Perseus** - Static site generation
- **Sycamore** - Reactive UI components
- **Tailwind CSS** - Styling

## Setup

```bash
# Install Perseus CLI
cargo install perseus-cli

# Run development server
perseus serve

# Build for production
perseus deploy
```

## Configuration

Edit files in `content/` directory:

- `site.toml` - Personal info, skills, experience
- `projects.toml` - Project entries

## Structure

```
├── content/
│   ├── site.toml
│   ├── projects.toml
│   └── posts/
├── src/
│   ├── components/
│   ├── models/
│   └── templates/
└── static/
    ├── css/
    └── images/
```
