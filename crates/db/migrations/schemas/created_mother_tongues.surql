
DEFINE TABLE created_mother_tongue SCHEMAFULL;

DEFINE FIELD in ON created_mother_tongue TYPE record<users>;
DEFINE FIELD out ON created_mother_tongue TYPE record<mother_tongues>;
DEFINE FIELD at ON created_mother_tongue TYPE datetime;

-- Make sure that there can only be one created_mother_tongue record per mother_tongue record
DEFINE INDEX unique_mother_tongue_creator ON TABLE created_mother_tongue COLUMNS out UNIQUE;
