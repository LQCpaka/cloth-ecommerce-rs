-- Enum Cart and Payment
CREATE TYPE order_status_type AS ENUM ('pending', 'confirmed', 'processing', 'shipped', 'delivered', 'cancelled','returned');
CREATE TYPE payment_status_type AS ENUM ('pending', 'paid', 'failed','returned');

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
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE SET NULL, -- Even user is deleted, but need to keep the order to checking
  address_id UUID NOT NULL REFERENCES user_addresses(id) ON DELETE SET NULL,

  order_number SERIAL UNIQUE,

  status order_status_type NOT NULL DEFAULT 'pending',
  payment_status payment_status_type NOT NULL DEFAULT 'pending',

  total_amount DECIMAL(12,2) NOT NULL,
  shipping_fee DECIMAL(10,2) NOT NULL,

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

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

-- Indexes
CREATE INDEX indx_orders_user_id ON orders(user_id);
CREATE INDEX indx_oder_items_order_id ON order_items(order_id);
CREATE INDEX indx_addressses_user_id ON user_addresses(user_id);
