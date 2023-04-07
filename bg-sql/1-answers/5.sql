select avg(avgscore), maxplayers from games group by maxplayers having avg(avgscore) > 6 order by maxplayers;
