SELECT TRIM(name) AS name
FROM (
  SELECT TRIM(name) AS name
  FROM games
  WHERE (name) LIKE '%arc%'
  UNION
  SELECT TRIM(d.designer) AS name
  FROM designers d
  WHERE (d.designer) LIKE '%arc%'
) AS combined_names
ORDER BY name ASC;