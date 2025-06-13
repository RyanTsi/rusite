-- Add up migration script here
CREATE TABLE IF NOT EXISTS comments (
    cid CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    aid CHAR(36) NOT NULL,
    uid CHAR(36) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (aid) REFERENCES articles(aid) ON DELETE CASCADE,
    FOREIGN KEY (uid) REFERENCES users(uid) ON DELETE CASCADE
);