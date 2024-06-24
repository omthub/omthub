DEFINE TABLE users SCHEMALESS;

DEFINE FIELD name ON users TYPE string;
DEFINE FIELD email ON users TYPE string;
DEFINE FIELD pw_hash ON users TYPE string;
DEFINE FIELD is_active ON users TYPE bool;
DEFINE FIELD registered_at ON users TYPE datetime;
