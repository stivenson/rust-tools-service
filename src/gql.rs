extern crate chrono;

use bson::oid::ObjectId;
use juniper::FieldResult;
use chrono::{DateTime, Utc};

use super::Context;
use crate::model::Tool;

pub struct Query;
pub struct Mutations;

graphql_object!(Tool: Context |&self| {
    field id() -> String { if let Some(ref id) = self.id { id.to_hex() } else { "".into() } }
    field tool_name() -> &str { self.tool_name.as_str() }
    field tool_link() -> &str { self.tool_link.as_str() }
    field tags() -> &str { self.tags.as_str() }
    field tool_description() -> &str { self.tool_description.as_str() } // optional
    field author_full_name() -> &str {self.author_full_name.as_str()} // optional
    field author_email() -> &str {self.author_email.as_str()} // optional
    field author_link() -> &str {self.author_link.as_str()} // optional
    field clicks() -> &str { self.clicks.as_str() } // optional
    field likes() -> &str { self.likes.as_str() } // optional
    field additional_data() -> &str { self.additional_data.as_str() } // optional
    field meta_data() -> &str { self.meta_data.as_str() } // optional
    field created() -> &str { self.created.as_str() } // optional
    field state() -> bool { self.state } // optional
});


// @TODO for here check to create pagination
graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
      "1.0"
    }

    field tools(&executor) -> FieldResult<Vec<Tool>> {
    let context = executor.context();
        Ok(context.db.list_tools()?)
    }

    field tool(&executor, id: String) -> FieldResult<Option<Tool>> {
      let context = executor.context();
      Ok(context.db.get_tool(&id)?)
    }
});

graphql_object!(Mutations: Context |&self| {
    field saveTool(&executor,
        id: Option<String>,
        tool_name: String,
        tool_link: String,
        tags: String,
        tool_description: Option<String>,
        author_full_name: Option<String>,
        author_email: Option<String>,
        author_link: Option<String>,
        clicks: Option<String>,
        likes: Option<String>,
        additional_data: Option<String>,
        meta_data: Option<String>, 
        created: Option<String>, 
        state: bool
    ) -> FieldResult<Option<Tool>> {
        let context = executor.context();
        let id = id.map(|id| ObjectId::with_string(&id)).map_or(Ok(None), |v| v.map(Some))?;
        let now: DateTime<Utc> = Utc::now();
        let tool = Tool {
            id: id,
            tool_name: tool_name,
            tool_link: tool_link,
            tags: tags,
            tool_description: tool_description.unwrap_or_else( || "".into()),
            author_full_name: author_full_name.unwrap_or_else( || "".into()),
            author_email: author_email.unwrap_or_else( || "".into()),
            author_link: author_link.unwrap_or_else( || "".into()),
            clicks: clicks.unwrap_or_else( || "0".into() ),
            likes: likes.unwrap_or_else( || "0".into() ),
            additional_data: additional_data.unwrap_or_else( || "".into()),
            meta_data: meta_data.unwrap_or_else( || "".into()),
            created: created.unwrap_or_else( || format!("{:?}",now.format("%a %b %e %T %Y")) ),
            state: false
        };

        Ok(context.db.save_tool(tool)?)
    }
});