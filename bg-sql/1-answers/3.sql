select count(*) as count, games.g_id from games group by games.g_id order by count desc, games.g_id ASC limit 1;
