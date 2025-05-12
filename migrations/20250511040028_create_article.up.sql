-- Add up migration script here
CREATE TABLE IF NOT EXISTS articles (
    aid CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    title VARCHAR(255) NOT NULL,
    summary TEXT,
    content VARCHAR(255) NOT NULL,
    secret VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create the 'tags' table
CREATE TABLE IF NOT EXISTS tags (
    tid INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) UNIQUE NOT NULL
);

-- Create the 'article_tags' relation table
CREATE TABLE IF NOT EXISTS article_tags (
    aid CHAR(36),
    tid INT,
    PRIMARY KEY (aid, tid),
    FOREIGN KEY (aid) REFERENCES articles(aid) ON DELETE CASCADE,
    FOREIGN KEY (tid) REFERENCES tags(tid) ON DELETE CASCADE
);

-- Create the 'categories' table
CREATE TABLE IF NOT EXISTS categories (
    cid INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) UNIQUE NOT NULL
);

-- Create the 'article_categories' relation table
CREATE TABLE IF NOT EXISTS article_categories (
    aid CHAR(36) REFERENCES articles(aid) ON DELETE CASCADE,
    cid INT REFERENCES categories(cid) ON DELETE CASCADE,
    PRIMARY KEY (aid, cid)
);