

DROP TABLE IF EXISTS admin_users;

CREATE TABLE admin_users (
-- sqlite genera automaticamente el serial
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
  	name varchar(255) NOT NULL,
  	url varchar(255) NOT NULL,
   	front_deploy varchar(255) NOT NULL,
   	email varchar(255) NOT NULL,
   	password varchar(200) NOT NULL,
   	phone varchar(20),
   	dni varchar(20),
    status varchar(20) NOT NULL,
    role varchar(20) NOT NULL,
    created_at TIMESTAMP DEFAULT (datetime('now', 'localtime')),
 	  updated_at TIMESTAMP NOT NULL
);



-- CREATE UNIQUE INDEX uq_user_url ON admin_users (url);
-- DROP TABLE people;

-- CREATE TABLE people (
--   -- sqlite genera automaticamente el serial
--    person_id INTEGER PRIMARY KEY,
--    first_name TEXT NOT NULL,
--    last_name TEXT NOT NULL
-- );

-- INSERT INTO people(first_name,last_name ) VALUES("Mau", "Jou");

SELECT * from admin_users;