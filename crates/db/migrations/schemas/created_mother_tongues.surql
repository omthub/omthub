DEFINE TABLE created_mother_tongue SCHEMALESS;

DEFINE FIELD in ON created_mother_tongue TYPE record<users>;
DEFINE FIELD out ON created_mother_tongue TYPE record<mother_tongues>;
