SELECT g.name AS category, MAX(g.avgscore) AS avg
FROM gamecat gc
JOIN games g ON gc.g_id = g.g_id
JOIN categories c ON gc.c_id = c.c_id
GROUP BY g.name
HAVING COUNT(g.g_id) >= 1
ORDER BY avg DESC, category ASC
LIMIT 5;