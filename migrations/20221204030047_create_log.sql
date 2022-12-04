-- Add migration script here
CREATE TABLE IF NOT EXISTS log(
    price int,
    category int,
    buy_date date
);