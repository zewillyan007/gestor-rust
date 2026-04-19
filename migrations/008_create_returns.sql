-- Controle de devoluções
CREATE TABLE IF NOT EXISTS returns (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warranty_id TEXT,
    reason TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'requested' CHECK (status IN ('requested', 'approved', 'rejected', 'completed')),
    refund_amount REAL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (warranty_id) REFERENCES warranties(id) ON DELETE SET NULL
);
