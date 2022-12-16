use std::sync::Mutex;
use std::collections::BTreeSet;
use serde::Serialize;
use serde::de::DeserializeOwned;
use rusty_leveldb::{DB as LevelDB, Options, in_memory};
use lib_utilities::hash::sha3_256;

use crate::struct_user::User;
use crate::struct_group::Group;
use crate::constants::*;

pub const KEY_USERS: &str = "u-all";

pub const KEY_GROUPS: &str = "g-all";

// u-$id_user ------> User
pub fn key_user(id_user: &str)                         -> String { format!("u-{}", id_user) }

// g-$id_group ------> Group
pub fn key_group(id_group: &str)                       -> String { format!("g-{}", id_group) }

// g-$id_group-t-$id_template ------> Template
pub fn key_template(id_group: &str, id_template: &str) -> String { format!("g-{}-t-{}", id_group, id_template) }

// g-$id_group-i-$id_item ------> Item
pub fn key_item(id_group: &str, id_item: &str)         -> String { format!("g-{}-i-{}", id_group, id_item) }

// g-$id_group-l-$id_label ------> Label
pub fn key_label(id_group: &str, id_label: &str)       -> String { format!("g-{}-l-{}", id_group, id_label) }

// g-$id_group-i-$id_item-labels ------> [$id_label, ...]  
pub fn key_item_labels(id_group: &str, id_item: &str)  -> String { format!("g-{}-i-{}-labels", id_group, id_item) }

pub fn id_template(keyword: &str, width: u32, height: u32) -> String { format!("{}-{}x{}", keyword, width, height) } 

pub struct DB {
    db: Mutex<LevelDB>,
}

impl DB {

    pub fn new(
        path: &str,
    ) -> Self{
        let db: DB = DB { db: Mutex::new(LevelDB::open(path, Options::default()).unwrap()) };
        db.seed();
        db
    }

    pub fn _mock(
    ) -> Self {
        let db: DB = DB { db: Mutex::new(LevelDB::open("db", in_memory()).unwrap()) };
        db.seed();
        db
    }

    pub fn write<T: Serialize>(
        &self,
        k: &str,
        v: &T,
    ){
        let json: String = serde_json::to_string::<T>(v).unwrap();
        let mut _db = self.db.lock().unwrap(); 
        let _ = _db.put(k.as_bytes(), json.as_bytes());
        let _ = _db.flush();
    }

    pub fn write_batch<T: Serialize>(
        &self,
        ks: &Vec<String>,
        vs: &Vec<T>,
    ){
        let mut _db = self.db.lock().unwrap();
        for i in 0 .. ks.len() {
            let _ = _db.put(
                ks[i].as_bytes(), 
                serde_json::to_string::<T>(&vs[i]).unwrap().as_bytes()
            );
        }
        let _ = _db.flush();
    }

    pub fn write_batch_if_not_exist<T: Serialize>(
        &self,
        ks: &Vec<String>,
        vs: &Vec<T>,
    ){
        let mut _db = self.db.lock().unwrap();
        for i in 0 .. ks.len() {
            if _db.get(ks[i].as_bytes()).is_none() {
                let _ = _db.put(
                    ks[i].as_bytes(), 
                    serde_json::to_string::<T>(&vs[i]).unwrap().as_bytes()
                );
            }
        }
        let _ = _db.flush();
    }

    pub fn read<T: DeserializeOwned>(
        &self,
        k: &str,
    ) -> Option<T> {
    
        match self.db.lock().unwrap().get(k.as_bytes()) {
            Some(bytes) => {
                let json: String = String::from_utf8_lossy(&bytes).into();
                Some(serde_json::from_str::<T>(&json).unwrap())
            },
            None => None
        }
    }
    
    pub fn read_batch<T: DeserializeOwned>(
        &self,
        ks: &Vec<String>,
    ) -> Vec<T> {
        let mut vs: Vec<T> = vec![];
        {
            let mut _db = self.db.lock().unwrap();
            for k in ks {
                if let Some(v) = _db.get(k.as_bytes()) {
                    let json: String = String::from_utf8_lossy(&v).into();
                    vs.push(serde_json::from_str::<T>(&json).unwrap());
                }
            }
        }
        vs
    }
    
    pub fn delete(
        &self,
        k: &str,
    ) {
        let mut _db = self.db.lock().unwrap();
        let _ = _db.delete(k.as_bytes());
        let _ = _db.flush();
    }

    fn seed<'r>(&self) {
    
        if let None = self.read::<BTreeSet<String>>(KEY_USERS) {

            // update u-all
            let mut users: BTreeSet<String> = BTreeSet::new();
            users.insert(ID_ADMIN.to_string());
            users.insert(ID_USER.to_string());
            self.write::<BTreeSet<String>>(KEY_USERS, &users);

            // update g-all
            let mut groups: BTreeSet<String> = BTreeSet::new();
            groups.insert(ID_GROUP_ADMIN.to_string());
            groups.insert(ID_GROUP_USER.to_string());
            self.write::<BTreeSet<String>>(KEY_GROUPS, &groups);

            // update u-$id_user
            let _key_admin: String = key_user(ID_ADMIN);
            let mut id_groups: BTreeSet<String> = BTreeSet::new();
            id_groups.insert(ID_GROUP_ADMIN.to_string());
            self.write::<User>(&_key_admin,
                &User{
                    password: sha3_256(PW_ADMIN),
                    role: Role::Admin,
                    token: TOKEN_ADMIN.to_string(),
                    id_groups,
                },
            );

            // update u-$id_user
            let _key_user: String = key_user(ID_USER);
            let mut id_groups: BTreeSet<String> = BTreeSet::new();
            id_groups.insert(ID_GROUP_USER.to_string());
            self.write::<User>(&_key_user,
                &User{
                    password: sha3_256(PW_USER),
                    role: Role::User,
                    token: TOKEN_USER.to_string(),
                    id_groups,
                }
            );
     
            // update g-$id_group
            let _key_group: String = key_group(ID_GROUP_ADMIN);
            self.write::<Group>(&_key_group,
                &Group {
                    id: ID_GROUP_ADMIN.to_string(),
                    name: "My First Group".to_string(),
                    id_bases: BTreeSet::new(),
                    id_templates: BTreeSet::new(),
                    id_associates: BTreeSet::new(),
                    id_items: BTreeSet::new(),
                    id_labels: BTreeSet::new(),
                }
            );

            // update g-$id_group
            let _key_group: String = key_group(ID_GROUP_USER);
            self.write::<Group>(&_key_group,
                &Group {
                    id: ID_GROUP_USER.to_string(),
                    name: "My First Group".to_string(),
                    id_bases: BTreeSet::new(),
                    id_templates: BTreeSet::new(),
                    id_associates: BTreeSet::new(),
                    id_items: BTreeSet::new(),
                    id_labels: BTreeSet::new(),
                }
            );
        }
    }
}