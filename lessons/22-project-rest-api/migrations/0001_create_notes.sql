CREATE TABLE notes (
    id      BIGSERIAL PRIMARY KEY,
    title   TEXT NOT NULL CHECK (title <> ''),
    body    TEXT NOT NULL DEFAULT '',
    done    BOOLEAN NOT NULL DEFAULT FALSE
);
