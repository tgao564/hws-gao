select avg(avgscore), maxplayers from games group by maxplayers having avgscore > 6 order by maxplayers;
