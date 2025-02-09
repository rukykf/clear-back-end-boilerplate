use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "A user's photo entry")]
#[derive(Debug)]
pub struct PhotoEntry {
    pub entry_id: String,
    pub created_at: String,
    pub base64_image: String,
}
