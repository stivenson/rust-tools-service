use bson::oid::ObjectId;
use juniper::FieldResult;

use super::Context;
use crate::model::Tool;

pub struct Query;
pub struct Mutations;

graphql_object!(Tool: Context |&self| {
    field id() -> String { if let Some(ref id) = self.id { id.to_hex() } else { "".into() } }
    field name() -> &str { self.name.as_str() }
    field link() -> &str { self.link.as_str() }
    field description() -> &str { self.description.as_str() }
    field tags() -> &str { self.tags.as_str() }
    field additional_data() -> &str { self.additional_data.as_str() }
});

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
        name: String,
        link: String,
        description: Option<String>,
        tags: String,
        additional_data: Option<String>
    ) -> FieldResult<Option<Tool>> {
        let context = executor.context();
        let id = id.map(|id| ObjectId::with_string(&id)).map_or(Ok(None), |v| v.map(Some))?;

        let tool = Tool {
            id: id,
            name: name,
            link: link,
            description: description.unwrap_or_else( || "".into()),
            tags: tags,
            additional_data: additional_data.unwrap_or_else( || "".into()),
        };

        Ok(context.db.save_tool(tool)?)
    }
});