-- Your SQL goes here
CREATE TABLE IF NOT EXISTS devices
(
    id    int GENERATED ALWAYS AS IDENTITY,
    name  varchar(128) not null,
    topic varchar(128) not null,
    PRIMARY KEY (id)
);