select name, length(name) as namelen from games where (minplayers >= 3 and maxplayers >< 5) order by namelen desc limit 10;