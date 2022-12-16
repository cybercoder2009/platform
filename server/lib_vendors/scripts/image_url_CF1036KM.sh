mosquitto_pub \
-q 1 \
-h 45.79.116.50 \
-t test/refresh/queue \
-m '{"queueId":'$RANDOM',"deviceType":1,"deviceCode":"CF1036KM","deviceMac":"001284C2E461BDFF","deviceVersion":"4.3.9","refreshAction":3,"refreshArea":1,"content":[{"dataRef":"https://na0.reducing.ca/images/CF1036KM.jpg","layerEnd":true}]}'