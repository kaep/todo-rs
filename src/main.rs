use axum::{
    Router,
    routing::{delete, get, post, put},
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/todos", get(get_todos))
        .route("/todos", post(add_todo))
        .route("/todos/{id}", put(complete_todo))
        .route("/todos/{id}", delete(delete_todo));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_todos() -> String {
    "Getting todos!".into()
}

async fn add_todo() -> String {
    "Adding todo!".into()
}

async fn complete_todo() -> String {
    "Completing todo!".into()
}

async fn delete_todo() -> String {
    "Deleting todo!".into()
}
