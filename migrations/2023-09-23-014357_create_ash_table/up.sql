CREATE TABLE ashes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    ash TEXT NOT NULL
);

CREATE TABLE aschanges (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    ash_id INTEGER NOT NULL REFERENCES ash(id),
    ante_id INTEGER REFERENCES ash(id),
    time TIMESTAMP NOT NULL,
    duration INTEGER DEFAULT 1 NOT NULL, -- duration in seconds, adjust as needed
    sigma INTEGER DEFAULT 1 NOT NULL, -- using TINYINT to represent the smallest integer type
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
  (200, 'spec'),
    (201, 'contact'), (202, 'contact.name'), (203, 'contact.phone'),
  (300, 'custom'), (301, 'app.title')
;

INSERT INTO aschanges (id, ash_id, ante_id, time, duration, sigma, product_id, alias, rate) VALUES
  -- I18n localization
  (1, 1, 2, CURRENT_TIMESTAMP, 0, 1, 1, "Language", NULL),
  (2, 2, 2, CURRENT_TIMESTAMP, 0, 1, 1, "English", NULL),
  (3, 3, 2, CURRENT_TIMESTAMP, 0, 1, 1, "English (United States)", NULL),

  -- ordinary pagination settings
  (4, 100, NULL, CURRENT_TIMESTAMP, 0, 1, 100, "% / N", 20.0),
  -- system-supported field localization
  (5, 101, 2, CURRENT_TIMESTAMP, 0, 1, 5, "Name:", NULL),
  (6, 104, 2, CURRENT_TIMESTAMP, 0, 1, 6, "Phone:", NULL),
  (7, 105, 2, CURRENT_TIMESTAMP, 0, 1, 7, "Email:", NULL),

  -- spec/custom namespace; ordinary over their fields
  (8, 100, NULL, CURRENT_TIMESTAMP, 0, 1, 4, NULL, NULL),
  (9, 200, NULL, CURRENT_TIMESTAMP, 0, 1, 4, NULL, NULL),

  -- company spec
  (10, 101, 2, CURRENT_TIMESTAMP, 0, 1, 2, "Contact", NULL),
  (11, 102, 2, CURRENT_TIMESTAMP, 0, 1, 2, "Name", NULL),

  (12, 301, 3, CURRENT_TIMESTAMP, 0, 1, 3, "Scott J Aveda - 72nd Street", NULL)
;
