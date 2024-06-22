DEFINE TABLE mother_tongues SCHEMALESS;

DEFINE FIELD name ON mother_tongues TYPE string;
DEFINE FIELD description ON mother_tongues TYPE string;
DEFINE FIELD is_vetted ON mother_tongues TYPE bool;
DEFINE FIELD meta ON mother_tongues TYPE object;

-- Define an analyzer that separates queries on spaces and lowercases them
DEFINE ANALYZER space_analyzer TOKENIZERS class FILTERS edgengram(1,15),lowercase;

-- Defining two full-text indexes on the 'name' and 'description' fields
DEFINE INDEX mother_tongues_name ON mother_tongues FIELDS name SEARCH ANALYZER space_analyzer BM25;
DEFINE INDEX mother_tongues_description ON mother_tongues FIELDS description SEARCH ANALYZER space_analyzer BM25;
