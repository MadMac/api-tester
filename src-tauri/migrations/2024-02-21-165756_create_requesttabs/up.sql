-- Your SQL goes here
CREATE TABLE requesttabs (
  	uuid TEXT PRIMARY KEY NOT NULL,
	tabdata TEXT NOT NULL,
	tabdata_saved TEXT,
	saved_timestamp DATETIME
)