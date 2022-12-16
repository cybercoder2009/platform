mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/refresh/queue \
-m '{"queueId":'$RANDOM',"deviceType":1,"deviceCode":"DC10067X","deviceMac":"001284C22631FD2F","deviceVersion":"4.3.6","refreshAction":2,"refreshArea":0}'