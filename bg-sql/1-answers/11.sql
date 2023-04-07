SELECT g.name AS category, AVG(g.avgscore) AS avg
FROM gamecat gc
JOIN games g ON gc.g_id = g.g_id
JOIN categories c ON gc.c_id = c.c_id
GROUP BY g.name
ORDER BY avg DESC, g.name ASC
LIMIT 5;