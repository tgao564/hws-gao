SELECT c.category AS category, AVG(g.avgscore) AS avg
FROM gamecat gc
JOIN games g ON gc.g_id = g.g_id
JOIN categories c ON gc.c_id = c.c_id
GROUP BY c.c_id, c.category
ORDER BY avg DESC, category ASC
LIMIT 5;
