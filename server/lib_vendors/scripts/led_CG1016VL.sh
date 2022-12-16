mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/device/config \
-m '{"queueId":'$RANDOM',"action":2,"deviceType":1,"deviceCode":"CG1016VL","deviceMac":"0012383B2653F1DB","content":{"led_info":{"color":4294901760,"period":5000,"count":5}}}'
