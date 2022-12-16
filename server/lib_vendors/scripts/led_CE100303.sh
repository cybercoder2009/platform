mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/device/config \
-m '{"queueId":'$RANDOM',"action":2,"deviceType":1,"deviceCode":"CE100303","deviceMac":"0012383B2631FF04","content":{"led_info":{"color":4294901760,"period":1000,"count":5}}}'
