CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER REFERENCES users(id),
    machine_id INTEGER REFERENCES machines(id),
    started_at DATETIME NOT NULL,
    ended_at DATETIME,
    minutes_consumed INTEGER DEFAULT 0
);