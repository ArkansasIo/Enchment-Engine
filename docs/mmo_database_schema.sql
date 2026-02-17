-- MMO Database Schema (PostgreSQL example)

-- Accounts & Authentication
CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    username VARCHAR(32) UNIQUE NOT NULL,
    email VARCHAR(128) UNIQUE NOT NULL,
    password_hash VARCHAR(128) NOT NULL
);

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    token VARCHAR(128) NOT NULL,
    expires_at BIGINT NOT NULL
);

-- Characters
CREATE TABLE characters (
    id SERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    name VARCHAR(32) NOT NULL,
    class VARCHAR(32),
    race VARCHAR(32),
    background VARCHAR(32),
    level INTEGER NOT NULL,
    xp BIGINT NOT NULL,
    stats JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Inventory
CREATE TABLE inventory (
    id SERIAL PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id),
    item_id VARCHAR(64) NOT NULL,
    quantity INTEGER NOT NULL,
    equipped BOOLEAN DEFAULT FALSE
);

-- Economy
CREATE TABLE currency (
    id SERIAL PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id),
    amount BIGINT NOT NULL
);

CREATE TABLE auction_listings (
    id SERIAL PRIMARY KEY,
    seller INTEGER REFERENCES characters(id),
    item_id VARCHAR(64) NOT NULL,
    price BIGINT NOT NULL,
    expires_at BIGINT NOT NULL
);

CREATE TABLE trades (
    id SERIAL PRIMARY KEY,
    from_id INTEGER REFERENCES characters(id),
    to_id INTEGER REFERENCES characters(id),
    item_ids TEXT[],
    currency BIGINT
);

-- Social
CREATE TABLE guilds (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL
);

CREATE TABLE guild_members (
    guild_id INTEGER REFERENCES guilds(id),
    character_id INTEGER REFERENCES characters(id),
    PRIMARY KEY (guild_id, character_id)
);

CREATE TABLE friends (
    player_id INTEGER REFERENCES characters(id),
    friend_id INTEGER REFERENCES characters(id),
    PRIMARY KEY (player_id, friend_id)
);

-- World & Events
CREATE TABLE zones (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL
);

CREATE TABLE world_events (
    id SERIAL PRIMARY KEY,
    zone_id INTEGER REFERENCES zones(id),
    description TEXT,
    active BOOLEAN
);
