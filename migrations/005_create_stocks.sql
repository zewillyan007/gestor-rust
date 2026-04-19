-- Controle de estoque por produto
CREATE TABLE IF NOT EXISTS stocks (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL UNIQUE,
    quantity INTEGER NOT NULL DEFAULT 0,
    min_quantity INTEGER NOT NULL DEFAULT 0,
    location TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
);
