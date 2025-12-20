use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
/// Production GraphQL Framework.
/// Provides schema definition and query execution engine using async-graphql.
use fusion_http::{Request, Response};

// Define the Query Root
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello from Fusion GraphQL!"
    }

    async fn version(&self) -> &str {
        "0.2.0"
    }

    // Example: User query
    async fn user(&self, id: i32) -> User {
        User {
            id,
            name: format!("User_{}", id),
            email: format!("user{}@fusion-lang.org", id),
        }
    }
}

// Example User type
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[Object]
impl User {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn email(&self) -> &str {
        &self.email
    }
}

pub struct GraphQLServer {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

impl GraphQLServer {
    pub fn new() -> Self {
        let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

        Self { schema }
    }

    /// Handles an incoming HTTP request containing a GraphQL query.
    pub async fn handle_request(&self, req: Request<Vec<u8>>) -> Response<Vec<u8>> {
        // Parse JSON body to extract GraphQL request
        let body = String::from_utf8_lossy(req.body());

        match serde_json::from_str::<async_graphql::Request>(&body) {
            Ok(graphql_req) => {
                // Execute the query
                let response = self.schema.execute(graphql_req).await;

                // Serialize response to JSON
                let json = serde_json::to_string(&response).unwrap_or_else(|_| {
                    r#"{"errors":[{"message":"Failed to serialize response"}]}"#.to_string()
                });

                Response::builder()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(json.into_bytes())
                    .unwrap()
            }
            Err(_) => {
                let error = r#"{"errors":[{"message":"Invalid GraphQL request"}]}"#;
                Response::builder()
                    .status(400)
                    .header("Content-Type", "application/json")
                    .body(error.as_bytes().to_vec())
                    .unwrap()
            }
        }
    }
}
