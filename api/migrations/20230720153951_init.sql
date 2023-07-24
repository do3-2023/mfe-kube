-- Add migration script here
CREATE TABLE IF NOT EXISTS messages(
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    content varchar(2000) NOT NULL,
    created_at timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

