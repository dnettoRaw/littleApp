
        
CREATE TABLE access
(
  id    INTEGER NOT NULL UNIQUE,
  level INTEGER NULL    ,
  name  TEXT    NULL    ,
  PRIMARY KEY (id AUTOINCREMENT)
);

CREATE TABLE adress
(
  id           INTEGER NOT NULL UNIQUE,
  streatNumber INTEGER NULL    ,
  streat       TEXT    NULL    ,
  cep          INTEGER NULL    ,
  country      TEXT    NULL    ,
  PRIMARY KEY (id AUTOINCREMENT)
);

CREATE TABLE category
(
  id   INTEGER NOT NULL UNIQUE,
  name TEXT    NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT)
);

CREATE TABLE configs
(
  version      INTEGER NOT NULL UNIQUE,
  flags_offset BLOB    NULL    ,
  PRIMARY KEY (version AUTOINCREMENT)
);

CREATE TABLE tax
(
  id    INTEGER NOT NULL UNIQUE,
  name  TEXT    NULL    ,
  value REAL    NULL    ,
  PRIMARY KEY (id AUTOINCREMENT)
);

CREATE TABLE providers
(
  id         INTEGER NOT NULL UNIQUE,
  provider   INTEGER NOT NULL,
  next       INTEGER NULL    ,
  date       INTEGER NULL    ,
  buy_prices INTEGER NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (next) REFERENCES providers (id),
  FOREIGN KEY (provider) REFERENCES provider (id),
  FOREIGN KEY (buy_prices) REFERENCES price_log (id)
);

CREATE TABLE product
(
  id        INTEGER NOT NULL UNIQUE,
  ean       INTEGER NULL    ,
  name      TEXT    NOT NULL,
  price     INTEGER NULL    ,
  created   INTEGER NULL    ,
  lot       INTEGER NULL    ,
  providers INTEGER NULL    ,
  category  INTEGER NULL    ,
  flags     INTEGER NULL    ,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (ean) REFERENCES ean (id),
  FOREIGN KEY (price) REFERENCES price (id),
  FOREIGN KEY (lot) REFERENCES lot (id),
  FOREIGN KEY (created) REFERENCES created (id),
  FOREIGN KEY (providers) REFERENCES providers (id),
  FOREIGN KEY (category) REFERENCES category (id)
);

CREATE TABLE price_log
(
  id      INTEGER NOT NULL UNIQUE,
  type    INTEGER NULL     DEFAULT 0,
  price   REAL    NULL    ,
  date    INTEGER NULL    ,
  next    INTEGER NULL    ,
  product INTEGER NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (next) REFERENCES price_log (id),
  FOREIGN KEY (product) REFERENCES product (id)
);

CREATE TABLE price
(
  id        INTEGER NOT NULL UNIQUE,
  tax       INTEGER NOT NULL,
  tax_price REAL    NULL    ,
  sell      REAL    NULL    ,
  price_log INTEGER NULL    ,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (tax) REFERENCES tax (id),
  FOREIGN KEY (price_log) REFERENCES price_log (id)
);

CREATE TABLE ean
(
  id      INTEGER NOT NULL UNIQUE,
  type    INTEGER NULL    ,
  data    BLOB    NULL     UNIQUE,
  nextEan INTEGER NULL    ,
  article INTEGER NOT NULL,
  price   INTEGER NULL    ,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (nextEan) REFERENCES ean (id),
  FOREIGN KEY (article) REFERENCES product (id),
  FOREIGN KEY (price) REFERENCES price (id)
);

CREATE TABLE user_sector
(
  id    INTEGER NOT NULL UNIQUE,
  acess INTEGER NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (acess) REFERENCES access (id)
);

CREATE TABLE user
(
  id        INTEGER NOT NULL UNIQUE,
  Name      TEXT    NOT NULL,
  last_name TEXT    NULL    ,
  Active    INTEGER NULL     DEFAULT 0,
  sector    INTEGER NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (sector) REFERENCES user_sector (id)
);

CREATE TABLE created
(
  id         INTEGER NOT NULL UNIQUE,
  creat_user INTEGER NOT NULL,
  edit_user  INTEGER NOT NULL,
  creat_date INTEGER NOT NULL,
  edit_date  INTEGER NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (edit_user) REFERENCES user (id),
  FOREIGN KEY (creat_user) REFERENCES user (id)
);

CREATE TABLE lot
(
  id         INTEGER NOT NULL UNIQUE,
  quantity   INTEGER NULL    ,
  ean        INTEGER NOT NULL,
  product    INTEGER NOT NULL,
  price      INTEGER NOT NULL,
  date       INTEGER NOT NULL,
  expiration INTEGER NULL    ,
  name       TEXT    NULL    ,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (ean) REFERENCES ean (id),
  FOREIGN KEY (product) REFERENCES product (id),
  FOREIGN KEY (price) REFERENCES price (id),
  FOREIGN KEY (date) REFERENCES created (id)
);

CREATE TABLE provider
(
  id       INTEGER NOT NULL UNIQUE,
  name     TEXT    NOT NULL,
  contact  TEXT    NULL    ,
  email    TEXT    NULL    ,
  adress   INTEGER NOT NULL,
  prefix   TEXT    NULL    ,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (adress) REFERENCES adress (id)
);

CREATE TABLE sector_child
(
  id         INTEGER NOT NULL UNIQUE,
  parent     INTEGER NOT NULL,
  next_child INTEGER NULL    ,
  name       TEXT    NULL    ,
  cat        INTEGER NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (next_child) REFERENCES sector_child (id),
  FOREIGN KEY (cat) REFERENCES category (id)
);

CREATE TABLE sector
(
  id          INTEGER NOT NULL UNIQUE,
  name        TEXT    NULL    ,
  default_tax INTEGER NULL    ,
  childs      INTEGER NULL    ,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (default_tax) REFERENCES tax (id),
  FOREIGN KEY (childs) REFERENCES sector_child (id)
);

CREATE TABLE stock
(
  id      INTEGER NOT NULL UNIQUE,
  stock   INTEGER NULL    ,
  mim     INTEGER NULL    ,
  edit    INTEGER NULL    ,
  product INTEGER NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (edit) REFERENCES created (id),
  FOREIGN KEY (product) REFERENCES product (id)
);