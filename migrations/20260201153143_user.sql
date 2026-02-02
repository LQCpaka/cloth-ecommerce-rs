CREATE TYPE user_role_type AS ENUM ('user', 'seller', 'moderator', 'admin');
CREATE TYPE user_status_type AS ENUM ('unverified', 'active', 'banned');
CREATE TYPE auth_provider_type AS ENUM ('local', 'google');

-- TABLE: User
CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

  -- user data
  email TEXT NOT NULL UNIQUE,
  password_hash TEXT,

  --user profile
  name TEXT NOT NULL,
  avatar_url TEXT, -- can be null because if people not using it, then they will use default url image that FE decided if this field is null or empty
  description TEXT,

  role user_role_type NOT NULL DEFAULT 'user',
  status user_status_type NOT NULL DEFAULT 'unverified',

  provider auth_provider_type NOT NULL DEFAULT 'local',
  provider_id TEXT, -- Goolge ID

  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- TABLE: User Session
CREATE TABLE IF NOT EXISTS user_sessions (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  refresh_token TEXT NOT NULL,
  user_agent TEXT, -- Example: Chrome or Brave On Windows
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_provider_id ON users(provider_id); -- Quick search with Google ID
