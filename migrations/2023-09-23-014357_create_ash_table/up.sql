CREATE TABLE ashes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    ash TEXT NOT NULL
);

CREATE TABLE aschanges (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    ash_id INTEGER NOT NULL REFERENCES ash(id),
    ante_id INTEGER NOT NULL REFERENCES ash(id),
    time TIMESTAMP NOT NULL,
    sigma INTEGER DEFAULT 1 NOT NULL,
    product_id INTEGER NOT NULL REFERENCES ash(id),
    alias TEXT,
    rate FLOAT, -- you can adjust the precision as needed
    FOREIGN KEY (ash_id) REFERENCES ash(id),
    FOREIGN KEY (ante_id) REFERENCES ash(id),
    FOREIGN KEY (product_id) REFERENCES ash(id)
);

INSERT INTO ashes (id, ash) VALUES
  (1, 'lang'), (2, 'lang.en'), (3, 'lang.en-US'),
  (100, 'ordinary'), (101, 'name'),
    (102, 'key'), (103, 'uri'),
    (104, 'phone'), (105, 'email'),
    (106, 'host'),
  (200, 'spec'),
    (201, 'contact'), (202, 'contact.name'), (203, 'contact.phone'),
  (300, 'custom'),
  (900, 'my'),
  (901, 'my.title')
;

INSERT INTO aschanges (id, ash_id, ante_id, time, sigma, product_id, alias, rate) VALUES
  -- I18n localization
  (1, 1, 2, CURRENT_TIMESTAMP, 1, 1, "Language", NULL),
  (2, 2, 2, CURRENT_TIMESTAMP, 1, 1, "English", NULL),
  (3, 3, 2, CURRENT_TIMESTAMP, 1, 1, "English (United States)", NULL),

  -- ordinary pagination settings
  (4, 100, 1, CURRENT_TIMESTAMP, 1, 100, "% / N", 20.0),
  -- system-supported field localization
  (5, 101, 2, CURRENT_TIMESTAMP, 1, 101, "Name:", NULL),
  (6, 104, 2, CURRENT_TIMESTAMP, 1, 104, "Phone:", NULL),
  (7, 105, 2, CURRENT_TIMESTAMP, 1, 105, "Email:", NULL),

  -- spec/custom namespace; ordinary over their fields
  (8, 100, 1, CURRENT_TIMESTAMP, 1, 100, NULL, NULL),
  (9, 200, 1, CURRENT_TIMESTAMP, 1, 100, NULL, NULL),

  -- company spec
  (10, 101, 2, CURRENT_TIMESTAMP, 1, 2, "Contact", NULL),
  (11, 102, 2, CURRENT_TIMESTAMP, 1, 2, "Name", NULL),

  (12, 301, 3, CURRENT_TIMESTAMP, 1, 3, "Hairbraint USA", NULL)
;
