-- Initial database schema for telemetry-kit server

-- Token tiers enum
CREATE TYPE token_tier AS ENUM ('free', 'pro', 'business', 'enterprise');

-- API tokens table
CREATE TABLE api_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL,
    app_id UUID NOT NULL,
    token VARCHAR(255) UNIQUE NOT NULL,
    secret VARCHAR(255) NOT NULL,
    tier token_tier NOT NULL DEFAULT 'free',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ,
    CONSTRAINT unique_org_app UNIQUE (org_id, app_id)
);

CREATE INDEX idx_tokens_token ON api_tokens(token) WHERE is_active = true;
CREATE INDEX idx_tokens_org_app ON api_tokens(org_id, app_id);

-- Events table
CREATE TABLE events (
    id BIGSERIAL PRIMARY KEY,
    event_id UUID UNIQUE NOT NULL,
    org_id UUID NOT NULL,
    app_id UUID NOT NULL,
    schema_version VARCHAR(20) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,

    -- Service info
    service_name VARCHAR(255) NOT NULL,
    service_version VARCHAR(100) NOT NULL,
    service_language VARCHAR(50) NOT NULL,
    service_language_version VARCHAR(50),

    -- User info (anonymous)
    user_id VARCHAR(255) NOT NULL,
    session_id VARCHAR(255),

    -- Environment
    os VARCHAR(50),
    os_version VARCHAR(100),
    arch VARCHAR(50),
    ci BOOLEAN,
    shell VARCHAR(50),

    -- Event
    event_type VARCHAR(100) NOT NULL,
    event_category VARCHAR(50),
    event_data JSONB NOT NULL,

    -- Metadata
    sdk_version VARCHAR(50) NOT NULL,
    transmission_timestamp TIMESTAMPTZ NOT NULL,
    batch_size INTEGER NOT NULL,
    retry_count INTEGER NOT NULL DEFAULT 0,

    -- Tracking
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for common queries
CREATE INDEX idx_events_org_app ON events(org_id, app_id);
CREATE INDEX idx_events_timestamp ON events(timestamp DESC);
CREATE INDEX idx_events_user_id ON events(user_id);
CREATE INDEX idx_events_event_type ON events(event_type);
CREATE INDEX idx_events_service_name ON events(service_name, timestamp DESC);
CREATE INDEX idx_events_event_id ON events(event_id);

-- Composite index for analytics queries
CREATE INDEX idx_events_analytics ON events(org_id, app_id, timestamp DESC, event_type);

-- JSONB index for event data queries
CREATE INDEX idx_events_data_gin ON events USING GIN (event_data);

-- Insert a test token for development
INSERT INTO api_tokens (org_id, app_id, token, secret, tier)
VALUES (
    '550e8400-e29b-41d4-a716-446655440000',
    '7c9e6679-7425-40de-944b-e07fc1f90ae7',
    'tk_550e8400e29b41d4a716446655440000_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6',
    '9f4b3c2a1e8d7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a1b0',
    'free'
);
