mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/device/config \
-m '{"queueId":'$RANDOM',"action":1,"deviceType":1,"deviceCode":"CG1016VL","deviceMac":"0012383B2653F1DB","content":["online", "version", "power", "temperature"]}'
