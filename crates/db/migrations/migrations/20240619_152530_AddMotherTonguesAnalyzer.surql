-- Define an analyzer that separates queries on spaces and lowercases them
DEFINE ANALYZER space_analyzer TOKENIZERS class FILTERS edgengram(1,15),lowercase;

-- Defining two full-text indexes on the 'name' and 'description' field of the 'mother_tongues' table
DEFINE INDEX mother_tongues_name ON mother_tongues FIELDS name SEARCH ANALYZER space_analyzer BM25;
DEFINE INDEX mother_tongues_description ON mother_tongues FIELDS description SEARCH ANALYZER space_analyzer BM25;
