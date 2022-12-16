mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/refresh/queue \
-m '{"queueId":'$RANDOM',"deviceType":1,"deviceCode":"DC10077A","deviceMac":"0012383B2631FE58","deviceVersion":"4.3.8","refreshAction":2,"refreshArea":0, "content":[]}'