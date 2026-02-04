-- Enum Cart and Payment
CREATE TYPE order_status_type AS ENUM ('pending', 'confirmed', 'processing', 'shipped', 'delivered', 'cancelled','returned');
CREATE TYPE payment_status_type AS ENUM ('pending', 'paid', 'failed','refunded');

-- TABLE: User Addresses
CREATE TABLE IF NOT EXISTS user_addresses(
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  recipient_name TEXT NOT NULL,
  recipient_phone TEXT NOT NULL,

  address_line TEXT NOT NULL, -- Số nhà,tên đường
  ward TEXT,                  -- Phường/Xã
  district TEXT NOT NULL,     -- Quận/Huyện
  city TEXT NOT NULL,         -- Tỉnh/Thành Phố

  is_default BOOLEAN DEFAULT FALSE,
  
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TABLE: Order
CREATE TABLE IF NOT EXISTS orders (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID REFERENCES users(id) ON DELETE SET NULL, -- Even user is deleted, but need to keep the order to checking
  address_id UUID REFERENCES user_addresses(id) ON DELETE SET NULL,

  order_number SERIAL UNIQUE,

  status order_status_type NOT NULL DEFAULT 'pending',
  payment_status payment_status_type NOT NULL DEFAULT 'pending',

  shipping_address_snapshot JSONB,

  total_amount DECIMAL(12,2) NOT NULL,
  shipping_fee DECIMAL(10,2) NOT NULL DEFAULT 0,

  notes TEXT,

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TABLE: Order Items
CREATE TABLE IF NOT EXISTS order_items(
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  variant_id UUID REFERENCES product_variants(id) ON DELETE SET NULL,

  quantity INTEGER NOT NULL CHECK (quantity > 0),
  price_at_purchase DECIMAL(12,2) NOT NULL, -- Snapshot: Price when buying.

  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TABLE: Cart
CREATE TABLE IF NOT EXISTS cart_items (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  variant_id UUID NOT NULL REFERENCES product_variants(id) ON DELETE CASCADE,

  quantity INTEGER NOT NULL CHECK (quantity > 0),

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  -- Important part: Bound
  -- A user cannot own more than 2 same product (line), like if you add to card, i should be 1 item and increase amount(quantity), not split to 2 items
  -- Only update quantity if user want to add more
  UNIQUE(user_id, variant_id)
);

-- Indexes
CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_addresses_user_id ON user_addresses(user_id);
