mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/device/config \
-m '{"queueId":'$RANDOM',"action":1,"deviceType":1,"deviceCode":"DC10067X","deviceMac":"001284C22631FD2F","content":["online", "version", "power", "temperature"]}'
