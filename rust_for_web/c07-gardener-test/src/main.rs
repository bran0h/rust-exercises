use std::sync::Arc;

use config::Config;
use model::garden::GardenField;
use mongodb::{Client, Database};
use service::garden::GardenServiceImpl;
use tower_http::cors::{Any, CorsLayer};
use tracing::event;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    run(config::config()?).await
}

async fn mock_db(database: &Database) -> eyre::Result<()> {
    // Drop existing collections
    database.drop().await?;
    let size = 4;
    let mut fields = Vec::<Option<GardenField>>::new();
    for _ in 0..size * size {
        fields.push(None);
    }

    // Create garden instance
    let garden = model::garden::Garden {
        id: uuid::Uuid::new_v4().to_string(),
        name: "My Garden".to_string(),
        size,
        fields,
    };

    // Insert garden instance into the database
    let collection = database.collection("garden");
    collection.insert_one(garden.clone()).await?;

    let plant1 = model::plant::Plant {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Tomato".to_string(),
        description: "Red and juicy".to_string(),
    };

    let plant2 = model::plant::Plant {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Cucumber".to_string(),
        description: "Green and crunchy".to_string(),
    };

    let plant3 = model::plant::Plant {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Carrot".to_string(),
        description: "Orange and sweet".to_string(),
    };

    let collection = database.collection("plant");

    collection.insert_one(plant1.clone()).await?;
    collection.insert_one(plant2.clone()).await?;
    collection.insert_one(plant3.clone()).await?;

    Ok(())
}

async fn run(config: Config) -> eyre::Result<()> {
    let client = Client::with_uri_str(config.mongo).await?;
    let database = client.database(&config.database);
    mock_db(&database).await?;

    let state = routes::RouterState {
        garden_service: Arc::from(GardenServiceImpl {
            collection: database.collection("garden"),
        }) as _,
        plant_service: Arc::from(service::plant::PlantServiceImpl {
            collection: database.collection("plant"),
        }) as _,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    event!(
        tracing::Level::INFO,
        "Configuring CORS policy: {:?}",
        cors.clone()
    );

    let app = routes::router(state).layer(cors);

    event!(tracing::Level::INFO, "Listening on {}", config.listen);

    axum_server::bind(config.listen)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
