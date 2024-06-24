
DEFINE TABLE created_artifact SCHEMAFULL;

DEFINE FIELD in ON created_artifact TYPE record<users>;
DEFINE FIELD out ON created_artifact TYPE record<artifacts>;
DEFINE FIELD at ON created_artifact TYPE datetime;

-- Make sure that there can only be one created_artifact record per mother_artifact record
DEFINE INDEX unique_artifact_creator ON TABLE created_artifact COLUMNS out UNIQUE;
