use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tool {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,

    #[serde(default)]
    pub tool_name: String,

    #[serde(default)]
    pub tool_link: String,

    #[serde(default)]
    pub tags: String,

    #[serde(default)]
    pub tool_description: String,
    
    #[serde(default)]
    pub author_full_name: String,

    #[serde(default)]
    pub author_email: String,

    #[serde(default)]
    pub author_link: String,

    #[serde(default)]
    pub clicks: String, // To call in background

    #[serde(default)]
    pub likes: String,
    
    #[serde(default)]
    pub additional_data: String,

   #[serde(default)]
    pub meta_data: String, // To call in background

   #[serde(default)]
    pub created: String,

   #[serde(default)]
    pub state: bool,
}
