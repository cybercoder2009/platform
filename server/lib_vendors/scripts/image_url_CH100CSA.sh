mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/refresh/queue \
-m '{"queueId":'$RANDOM',"deviceType":1,"deviceCode":"CH100CSA","deviceMac":"0012383B26320214","deviceVersion":"4.0.0","refreshAction":3,"refreshArea":1,"content":[{"dataRef":"https://na0.reducing.ca/images/CH100CSA_1.jpg","layerEnd":true}]}'