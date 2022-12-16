mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/refresh/queue \
-m '{"queueId":'$RANDOM',"deviceType":1,"deviceCode":"CH100CSA","deviceMac":"0012383B26320214","deviceVersion":"4.3.9","refreshAction":2,"refreshArea":0}'