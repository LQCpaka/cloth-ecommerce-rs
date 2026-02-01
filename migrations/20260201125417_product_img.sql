CREATE TABLE IF NOT EXISTS product_images(
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,

  variant_id UUID REFERENCES product_variants(id) ON DELETE CASCADE,

  image_url TEXT NOT NULL,
  is_main BOOLEAN DEFAULT FALSE, 
  sort_order INTEGER DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_images_product_id ON product_images(product_id);
CREATE INDEX idx_images_variant_id ON product_images(variant_id);
