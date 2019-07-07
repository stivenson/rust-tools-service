use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tool {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub link: String,

    #[serde(default)]
    pub description: String,
    
    #[serde(default)]
    pub tags: String,
    
    #[serde(default)]
    pub additional_data: String,
}
