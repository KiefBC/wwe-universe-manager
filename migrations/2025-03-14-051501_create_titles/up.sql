-- Your SQL goes here
CREATE TABLE titles (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    current_holder_id ID INTEGER NULL,
    FOREIGN KEY (current_holder_id) REFERENCES wrestlers (id)
);
