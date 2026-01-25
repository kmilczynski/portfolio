-- Posts table with both Polish and English content
CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,

    -- Polish content
    title_pl TEXT NOT NULL,
    excerpt_pl TEXT NOT NULL,
    content_pl TEXT NOT NULL,

    -- English content
    title_en TEXT NOT NULL,
    excerpt_en TEXT NOT NULL,
    content_en TEXT NOT NULL,

    -- Shared metadata
    date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    tags TEXT NOT NULL DEFAULT '[]',
    image TEXT,

    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Admin sessions table
CREATE TABLE IF NOT EXISTS admin_sessions (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT NOT NULL
);

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_posts_slug ON posts(slug);
CREATE INDEX IF NOT EXISTS idx_posts_status ON posts(status);
CREATE INDEX IF NOT EXISTS idx_sessions_expires ON admin_sessions(expires_at);
