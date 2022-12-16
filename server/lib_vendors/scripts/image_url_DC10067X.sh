mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/refresh/queue \
-m '{"queueId":'$RANDOM',"deviceType":1,"deviceCode":"DC10067X","deviceMac":"001284C22631FD2F","deviceVersion":"4.0.0","refreshAction":3,"refreshArea":1,"content":[{"dataRef":"http://192.53.120.76/200x200_3.jpg","layerEnd":true}]}'