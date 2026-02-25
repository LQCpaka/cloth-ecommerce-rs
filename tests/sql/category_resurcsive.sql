-- ==========================================
-- 1. TẠO DANH MỤC GỐC (Root - Không có cha)
-- ==========================================
INSERT INTO categories (name, slug, parent_id)
VALUES ('Thời trang Nam', 'thoi-trang-nam', NULL);

INSERT INTO categories (name, slug, parent_id)
VALUES ('Thời trang Nữ', 'thoi-trang-nu', NULL);

-- ==========================================
-- 2. TẠO DANH MỤC CẤP 1 (Con của Root)
-- ==========================================
-- Tạo "Áo Nam" nằm trong "Thời trang Nam"
INSERT INTO categories (name, slug, parent_id)
VALUES (
    'Áo Nam',
    'ao-nam',
    (SELECT id FROM categories WHERE slug = 'thoi-trang-nam')
);

-- Tạo "Quần Nam" nằm trong "Thời trang Nam"
INSERT INTO categories (name, slug, parent_id)
VALUES (
    'Quần Nam',
    'quan-nam',
    (SELECT id FROM categories WHERE slug = 'thoi-trang-nam')
);

-- ==========================================
-- 3. TẠO DANH MỤC CẤP 2 (Cháu của Root)
-- ==========================================
-- Tạo "Áo thun" nằm trong "Áo Nam"
INSERT INTO categories (name, slug, parent_id)
VALUES (
    'Áo thun nam',
    'ao-thun-nam',
    (SELECT id FROM categories WHERE slug = 'ao-nam')
);

-- Tạo "Áo sơ mi" nằm trong "Áo Nam"
INSERT INTO categories (name, slug, parent_id)
VALUES (
    'Áo sơ mi nam',
    'ao-so-mi-nam',
    (SELECT id FROM categories WHERE slug = 'ao-nam')
);
