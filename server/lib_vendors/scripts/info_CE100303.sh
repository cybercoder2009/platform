mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/device/config \
-m '{"queueId":'$RANDOM',"action":1,"deviceType":1,"deviceCode":"CE100303","deviceMac":"001284C2E461BDFF","content":["online", "version", "power", "temperature"]}'
