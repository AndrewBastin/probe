CREATE TABLE Report (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    ip_address TEXT,
    data TEXT NOT NULL
);

INSERT INTO Report (title, ip_address, data)
SELECT "Untitled" as title, ip_address, data FROM Event;

DROP TABLE Event;
