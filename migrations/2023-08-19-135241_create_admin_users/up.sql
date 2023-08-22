CREATE TABLE admin_users (
-- sqlite genera automaticamente el serial
    user_id INTEGER NOT NULL PRIMARY KEY,
  	name varchar(255) NOT NULL,
  	url varchar(255) NOT NULL UNIQUE,
   	front_deploy varchar(255) NOT NULL,
   	email varchar(255) NOT NULL,
   	password varchar(200) NOT NULL,
   	phone varchar(20),
   	dni varchar(20),
    status varchar(20) NOT NULL,
    role varchar(20) NOT NULL,
    -- created_at TIMESTAMP DEFAULT (datetime('now', 'localtime')),
		created_at TEXT NOT NULL,
 	  -- updated_at TIMESTAMP NOT NULL
		updated_at TEXT
)

