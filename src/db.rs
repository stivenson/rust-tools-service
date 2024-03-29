use bson::{from_bson, oid::ObjectId, to_bson, Bson, Document};
use mongodb::{
    coll::{Collection, options::{FindOptions, CursorType}},
    db::ThreadedDatabase,
    Client, ThreadedClient,
};

use crate::allerrors::Error;
use crate::model::Tool;
use std::env;

pub struct Db {
    client: Client,
    db_name: String,
}

impl Db {
    pub fn new<S>(db_name: S) -> Db
    where
        S: ToString,
    {
        let db_name = db_name.to_string();
        let client = Client::connect(&env::var("TOOLS_SERVICE_MONGO_URI").unwrap(), 27017).expect("Failed to initialize client.");
        Db { client, db_name }
    }

    pub fn list_tools(&self, skip: String, limit: String) -> Result<Vec<Tool>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("tools");
        let options = Some(FindOptions {
            skip: Some(skip.to_string().parse::<i64>().unwrap()),
            limit: Some(limit.to_string().parse::<i64>().unwrap()),
            allow_partial_results: false,
            no_cursor_timeout: true,
            oplog_replay: false,
            cursor_type: CursorType::default(),
            batch_size: None,
            comment: None,
            max_time_ms: None,
            modifiers: None,
            projection: None,
            sort: None,
            read_preference: None
        });
        // Document
        let cursor = coll.find(None, options)?;
        let res: Result<Vec<_>, _> = cursor
            .map(|row| row.and_then(|item| Ok(from_bson::<Tool>(Bson::Document(item))?)))
            .collect();
        Ok(res?)
    }

    pub fn get_tool(&self, id: &str) -> Result<Option<Tool>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("tools");
        let cursor: Option<Document> = coll.find_one(Some(doc! { "_id": ObjectId::with_string(id)? }), None)?;
        cursor
            .map(|doc| Ok(from_bson::<Tool>(Bson::Document(doc))?))
            .map_or(Ok(None), |v| v.map(Some))
    }

    pub fn save_tool(&self, prod: Tool) -> Result<Option<Tool>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("tools");

        if let Bson::Document(mut doc) = to_bson(&prod)? {
            doc.remove("_id");
            let res = coll.insert_one(doc, None)?;
            if let Some(exception) = res.write_exception {
                return Err(Error::from(exception));
            }
            if let Some(inserted_id) = res.inserted_id {
                if let Bson::ObjectId(id) = inserted_id {
                    self.get_tool(&id.to_hex())
                } else {
                    Err(Error::Custom("No valid id returned after insert".into()))
                }
            } else {
                Err(Error::Custom("No data returned after insert".into()))
            }
        } else {
            Err(Error::Custom("Invalid document".into()))
        }
    }

    pub fn update_tool(&self, prod: Tool) -> Result<Option<Tool>, Error> {
        let coll: Collection = self.client.db(&self.db_name).collection("tools");

        if let Bson::Document(doc) = to_bson(&prod)? {
            if let Some(ref id) = prod.id {
                let filter = doc!{ "_id": Bson::ObjectId(id.clone()) };
                let payload = doc!{"$set": doc};
                let res = coll.update_one(filter, payload, None)?;
                if res.modified_count > 0 {
                    self.get_tool(&id.to_hex())
                } else {
                    Err(Error::Custom("0 registers modified".into()))
                }
            } else {
                Err(Error::Custom("The Id is neccessary".into()))
            }
        } else {
            Err(Error::Custom("Invalid document".into()))
        }
    }
}