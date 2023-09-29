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
  (1, 'lang'),
    (2, 'lang.en'), (3, 'lang.en-US'),
    (4, 'lang.es'), (5, 'lang.he'),
  (90, 'xml'), (91, 'json'), (92, 'html'),
  (100, 'ordinary'), (101, 'name'),
    (102, 'key'), (103, 'uri'),
    (104, 'phone'), (105, 'email'),
    (106, 'contact'),
  (150, 'currency'), (151, 'USD'),
  (200, 'spec'),
    (201, 'spec.contact'), (202, 'spec.contact.name'), (203, 'spec.contact.phone'), (204, 'spec.contact.email'),
    (205, 'spec.contact.nearby'), (206, 'spec.contact.ancestors'), (207, 'spec.contact.descendants'),
    (210, 'contacts'), (211, 'contacts#'), (212, 'contacts#1'),
  (300, 'custom'),
    (301, 'custom.company'), (302, 'custom.company.name'),
    (310, 'companies#'),
  (900, 'my'), (901, 'my.name'), (902, 'my.phone'), (903, 'my.company'),
  (1000, 'service.csp'), (1001, 'service.csp#1')
;

INSERT INTO aschanges (id, ash_id, ante_id, time, sigma, product_id, alias, rate) VALUES

  -- I18n localization
  (1, 1, 2, CURRENT_TIMESTAMP, 1, 2, "Language", NULL),
  (2, 2, 2, CURRENT_TIMESTAMP, 1, 2, "English", NULL),
  (3, 3, 2, CURRENT_TIMESTAMP, 1, 2, "English (United States)", NULL),
  (14, 4, 1, CURRENT_TIMESTAMP, 1, 2, "Spanish", NULL),
  (15, 4, 4, CURRENT_TIMESTAMP, 1, 4, "Espanol", NULL),

  -- ordinary pagination settings
  (4, 100, 1, CURRENT_TIMESTAMP, 1, 100, "% / N", 20.0),
  -- system-supported field localization
  (5, 201, 2, CURRENT_TIMESTAMP, 1, 101, "Name:", NULL),
  (6, 203, 2, CURRENT_TIMESTAMP, 1, 104, "Phone:", NULL),
  (7, 204, 2, CURRENT_TIMESTAMP, 1, 105, "Email:", NULL),

  -- spec namespace; ordinary over their fields
  (8, 200, 1, CURRENT_TIMESTAMP, 1, 100, NULL, NULL),
  (9, 300, 1, CURRENT_TIMESTAMP, 1, 100, NULL, NULL),

  -- spec/contact spec
  (10, 201, 2, CURRENT_TIMESTAMP, 1, 2, "Contact", NULL),
  (11, 202, 201, CURRENT_TIMESTAMP, 1, 101, "{}", NULL),
  (17, 210, 201, CURRENT_TIMESTAMP, 1, 2, "Contacts", NULL),
  (18, 211, 201, CURRENT_TIMESTAMP, 1, 100, "# of %", 20),
  (19, 212, 211, CURRENT_TIMESTAMP, 1, 106, NULL, NULL),
  (20, 106, 201, CURRENT_TIMESTAMP, 1, 106, NULL, NULL),
  (25, 106, 90, CURRENT_TIMESTAMP, 1, 90, "<contact><name>{.name}</name></contact>", NULL),

  -- custom
  (16, 301, 2, CURRENT_TIMESTAMP, 1, 201, NULL, NULL),

  -- my
  (12, 900, 2, CURRENT_TIMESTAMP, 1, 201, NULL, NULL),
  (13, 901, 202, CURRENT_TIMESTAMP, 1, 5, "Mind Yabiz-Ness", NULL),
  (21, 901, 202, CURRENT_TIMESTAMP, 1, 3, "Jabez Christi", NULL),

  (22, 1000, 2, CURRENT_TIMESTAMP, 1, 1, "Color; Single Process", NULL),
  (23, 1001, 1000, CURRENT_TIMESTAMP, 1, 1000, "40g 7N + 40g 8N + 80g 20 vol", 1.00),
  (24, 1001, 1000, CURRENT_TIMESTAMP, 1, 151, "VISA xxx4", 110.0)

;
