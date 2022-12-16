mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/refresh/queue \
-m '{"queueId":'$RANDOM',"deviceType":1,"deviceCode":"CE100303","deviceMac":"0012383B2631FF04","deviceVersion":"4.0.0","refreshAction":3,"refreshArea":1,"content":[{"dataRef":"http://192.53.120.76/152x296_1.jpg","layerEnd":true}]}'
