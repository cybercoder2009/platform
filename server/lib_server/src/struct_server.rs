use std::num::NonZeroUsize;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use lru::LruCache;
use rumqttc::{MqttOptions, AsyncClient};

use crate::struct_auth::Auth;
use crate::struct_db::DB;

pub struct Server {
    pub db: Arc<DB>, 
    pub mqtt: AsyncClient,
    pub images: String,
    pub cache: Mutex<LruCache<String, Auth>>,
}

impl Server {

    pub fn new(
        db: DB,
        mqtt: AsyncClient,
        images: String,
    ) -> Self {

        Self {
            db: Arc::new(db),
            mqtt,
            images,
            cache: Mutex::new(LruCache::new(NonZeroUsize::new(300).unwrap())), 
        }
    }

    pub fn _mock() -> Self {

        let mut mqttoptions = MqttOptions::new("rumqtt-async", "test.mosquitto.org", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        let (client, _) = AsyncClient::new(mqttoptions, 10);
        Self {
            db: Arc::new(DB::_mock()),
            // db: DB::new("db"),
            cache: Mutex::new(LruCache::new(NonZeroUsize::new(10).unwrap())),
            mqtt: client,
            images: String::from("http://localhost:8080/images"),
        }
    }
}