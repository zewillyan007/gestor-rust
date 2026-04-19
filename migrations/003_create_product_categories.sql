-- Relação N:N entre produtos e categorias
CREATE TABLE IF NOT EXISTS product_categories (
    product_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    PRIMARY KEY (product_id, category_id),
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);
