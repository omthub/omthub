DEFINE TABLE mother_tongues SCHEMALESS;

DEFINE FIELD name ON mother_tongues TYPE string;
DEFINE FIELD description ON mother_tongues TYPE string;
DEFINE FIELD is_vetted ON mother_tongues TYPE bool;
DEFINE FIELD meta ON mother_tongues TYPE object;
