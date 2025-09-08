CREATE TABLE IF NOT EXISTS redeemed_codes(
    uid INT,
    code TEXT,
    PRIMARY KEY (uid, code)
);