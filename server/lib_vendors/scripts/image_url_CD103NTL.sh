mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/refresh/queue \
-m '{"queueId":'$RANDOM',"deviceType":1,"deviceCode":"CD103NTL","deviceMac":"0012383B2630CC17","deviceVersion":"4.3.8","refreshAction":3,"refreshArea":1,"content":[{"dataRef":"https://na0.reducing.ca/images/CD103NTL.jpg","layerEnd":true}]}'