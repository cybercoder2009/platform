mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/device/config \
-m '{"queueId":'$RANDOM',"action":1,"deviceType":1,"deviceCode":"CH100CSA","deviceMac":"0012383B26320214","content":["online", "version", "power", "temperature"]}'
