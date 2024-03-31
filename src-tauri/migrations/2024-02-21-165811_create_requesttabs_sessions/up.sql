-- Your SQL goes here
CREATE TABLE requesttabs_sessions (
	uuid TEXT PRIMARY KEY NOT NULL,
	requesttabs_uuid TEXT references requesttabs(uuid) NOT NULL,
	sessions_uuid TEXT references sessions(uuid) NOT NULL
)