USE api;

CREATE TABLE person(
    id serial NOT NULL PRIMARY KEY,
    last_name text NOT NULL,
    phone_number text NOT NULL,
    location text NOT NULL
);

INSERT INTO person(last_name, phone_number, location)
    VALUES ('John', '0702030405', 'Marseille'),
('Doe', '0603040506', 'Montpellier');

