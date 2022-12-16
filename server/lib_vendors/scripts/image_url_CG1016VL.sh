mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/refresh/queue \
-m '{"queueId":'$RANDOM',"deviceType":1,"deviceCode":"CG1016VL","deviceMac":"0012383B2653F1DB","deviceVersion":"4.3.8","refreshAction":2,"refreshArea":1,"content":[{"dataRef":"https://na0.reducing.ca/images/CG1016VL.jpg","layerEnd":true}]}'