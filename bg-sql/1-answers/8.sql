SELECT TRIM(name) AS name
FROM games
WHERE LOWER(name) LIKE '%arc%'
UNION
SELECT TRIM('designer') AS name
FROM games
WHERE LOWER('designer') LIKE '%arc%';