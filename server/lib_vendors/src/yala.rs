#![allow(non_snake_case)]

use serde::{Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct Content {
    pub dataRef: String,
    pub layerEnd: bool,
}

/**
 * image url example
 * {
 *      "queueId":"1004", // required
 *      "deviceType":1, // required
 *      "deviceCode":"CE100303", //
 *      "deviceMac":"0012383B2631FF04", // required
 *      "deviceVersion":"4.0.0", // required
 *      "refreshAction":3, // required
 *      "refreshArea":1, // required
 *      "content":[{"dataRef":"http://192.53.120.76/152x296_1.jpg","layerEnd":true}]
 * }
 * image factory example
 * {
 *      "queueId":'$RANDOM',
 *      "deviceType":1,
 *      "deviceCode":"CD103NTL",
 *      "deviceMac":"0012383B2630CC17",
 *      "deviceVersion":"4.0.0",
 *      "refreshAction":2,
 *      "refreshArea":0,
 *      "content": []
 * }
 */
#[derive(Serialize, Debug, Clone)]
pub struct Message {
    pub queueId: u16,
    pub deviceType: u32,
    pub deviceCode: String,
    pub deviceMac: String,
    pub deviceVersion: String,
    pub refreshAction: u32,
    pub refreshArea: u32,
    pub content: Vec<Content>,
}

impl Message {
    pub fn new(
        queueId: u16,
        deviceCode: &str,
        deviceMac: &str,
        deviceVersion: &str,
        refreshAction: u32,
        refreshArea: u32,
        content: Vec<Content>,
    ) -> Message {
        let Message: Message = Message {
            queueId,
            deviceType: 1,
            deviceCode: String::from(deviceCode),
            deviceMac: String::from(deviceMac),
            deviceVersion: String::from(deviceVersion),
            refreshAction,
            refreshArea,
            content,
        };
        Message
    }
}

/**

 */

#[cfg(test)]
mod test {

    use crate::yala::{Message, Content};

    #[test]
    fn test_Message() {

        let mut content: Vec<Content> = Vec::new();
        content.push(Content{
            dataRef: String::from("http://192.53.120.76/152x296_1.jpg"),
            layerEnd: true,
        });
        let Message = Message::new(
            1234u16,
            "abc",
            "0012383B2631FF04",
            "4.0.0",
            3,
            1,
            content,
        );

        let json_Message = serde_json::to_string(&Message).unwrap();
        println!("{}", json_Message);

        assert_eq!(
            r#"{"queueId":1234,"deviceType":1,"deviceCode":"abc","deviceMac":"0012383B2631FF04","deviceVersion":"4.0.0","refreshAction":3,"refreshArea":1,"content":[{"dataRef":"http://192.53.120.76/152x296_1.jpg","layerEnd":true}]}"#,
            &json_Message
        );
    }
}