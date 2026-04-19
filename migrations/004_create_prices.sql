-- Histórico de preços por produto
CREATE TABLE IF NOT EXISTS prices (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    cost_price REAL NOT NULL,
    sale_price REAL NOT NULL,
    effective_date TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
);
