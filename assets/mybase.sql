
        
CREATE TABLE access
(
  id    INTEGER NOT NULL UNIQUE,
  level INTEGER NULL    ,
  name          NULL    ,
  PRIMARY KEY (id AUTOINCREMENT)
);

CREATE TABLE ean
(
  id        INTEGER NOT NULL UNIQUE,
  ean13     INTEGER NULL    ,
  qrcode    TEXT    NULL    ,
  eancuston INTEGER NULL    ,
  PRIMARY KEY (id AUTOINCREMENT)
);

CREATE TABLE tax
(
  id    INTEGER NOT NULL UNIQUE,
  name  TEXT    NULL    ,
  value REAL    NULL    ,
  PRIMARY KEY (id AUTOINCREMENT)
);

CREATE TABLE user
(
  id        INTEGER NOT NULL UNIQUE,
  Name      TEXT    NOT NULL,
  Last Name TEXT    NULL    ,
  Active    INTEGER NULL     DEFAULT 0,
  sector    INTEGER NOT NULL,
  acess     INTEGER NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (sector) REFERENCES sector (id),
  FOREIGN KEY (acess) REFERENCES access (id)
);

CREATE TABLE created
(
  id         INTEGER NOT NULL UNIQUE,
  creat_user INTEGER NOT NULL,
  edit_user  INTEGER NOT NULL,
  creat_date INTEGER NULL    ,
  edit_date  INTEGER NULL    ,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (edit_user) REFERENCES user (id),
  FOREIGN KEY (creat_user) REFERENCES user (id)
);

CREATE TABLE price
(
  id        INTEGER NOT NULL UNIQUE,
  tax       INTEGER NOT NULL,
  price     REAL    NULL    ,
  sell      REAL    NULL    ,
  tax_price REAL    NULL    ,
  discont           NULL    ,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (tax) REFERENCES tax (id)
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

CREATE TABLE sector
(
  id    INTEGER NOT NULL UNIQUE,
  Name  TEXT    NULL    ,
  acess INTEGER NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (acess) REFERENCES access (id)
);

CREATE TABLE product
(
  id      INTEGER NOT NULL UNIQUE,
  ean     INTEGER NOT NULL,
  name    TEXT    NULL    ,
  price   INTEGER NOT NULL,
  lot     INTEGER NOT NULL,
  created INTEGER NOT NULL,
  sector  INTEGER NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (ean) REFERENCES ean (id),
  FOREIGN KEY (price) REFERENCES price (id),
  FOREIGN KEY (lot) REFERENCES lot (id),
  FOREIGN KEY (created) REFERENCES created (id),
  FOREIGN KEY (sector) REFERENCES sector (id)
);

        
      