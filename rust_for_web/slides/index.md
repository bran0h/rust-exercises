---
theme: teach-rs
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust for Web"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust for Web"
---

# Rust


## Rust for web

---

# Introduction

- Branislav Hozza
- Student@FIIT
- Rust enthusiast
- [Github](https://github.com/brano-hozza)
- xhozza@stuba.sk

---
layout: default
---

# Learning objective

- What is web development
- Web-focused crates
- Tracing
- State managment
- Serverless
  
---
layout: default
---

# Contents

- Web development
- State of Art
- Front-end & WASM
- Tracing
- Back-end
- Database
  - SQL
  - NoSQL
- Serverless

---
layout: section
---

# What is web development?

---
layout: default
---

# Webpage components

- Structure
- Styles?
- Script?

```html {all|3-8|9-13|6,7,10|11-12}
<!doctype html>
<html>
    <head>
        <meta charset="utf-8" />
        <title>Yew App</title>
        <style>p{color: red;}</style>
        <link rel="stylesheet" src="./style.css" type="text/css">
    </head>
    <body>
        <p style="font-family: 'Roboto'">Hello world</p>
        <script>console.log("Hello script")</script>
        <script src="./my-script.js"> </script>
    </body>
</html>

```

---
layout: default
---

# Architecture - buzzwords

- static / dynamic
- SSR / SPA / SSG (Generation / Hydration)
- REST / WS / SOAP 💀
- serverless (AWS / Azure / Supabase / Firebase / ... )


---
layout: default
---

# Architecture - SSR vs SPA

<img src="/images/SPA_SSR.png" class="self-center mt-10 p-10 rounded shadow bg-white" />

[Source](https://www.velv.pt/updates/server-side-rendering)

---
layout: section
---

# State of Art

---
layout: default
---

# [Are we web yet?](https://www.arewewebyet.org/)

- "Yes! And it's freaking fast!"
- Several web frameworks exist
  - [`rocket`](https://rocket.rs/)
  - [`actix-web`](https://actix.rs/)
  - [`warp`](https://github.com/seanmonstar/warp)
  - [`axum`](https://github.com/tokio-rs/axum)
  - ...lots more
- Several DB drivers and ORMs
- Much more!

*Tip: have a look if you want to do web stuff in your final project*
---
layout: section
---

# Front-end & WASM


---
layout: default
---

# WASM

- Assembly-like language
- Blazzingly fast

```txt
(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add)
  (export "add" (func $add))
)
```

- Use in browser/server

```js
WebAssembly.instantiateStreaming(fetch("add.wasm")).then((obj) => {
  console.log(obj.instance.exports.add(1, 2)); // "3"
});
```

---
layout: default
---

# Yew 

- [Build modern apps with WebAssembly](https://yew.rs/docs/tutorial)
- Install WASM target `rustup target add wasm32-unknown-unknown`
- Install trunk `cargo install --locked trunk`
- You are ready to go!

---
layout: default
---

# Yew 

- HTML with `html!` macro

```rust
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <p> Hello </p>
        </div>
    }
}
```

---
layout: default
---

# Yew 

- CSS with `classes!` macro

```rust
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <p class={classes!("container", "px-2")}> Hello </p>
        </div>
    }
}
```

---
layout: default
---

# Yew 

- JS with RS

```rust
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let optionalClass = Some("class")

    html! {
        <div>
            <button class={classes!(optionalClass)} {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
```

---
layout: default
---

# Yew 

- Simple setup

```rust
fn main() {
    yew::Renderer::<App>::new().render();
}
```

- Create `index.html` in the root

```html
<!doctype html>
<html>
    <head>
        <meta charset="utf-8" />
        <title>Yew App</title>
    </head>
    <body></body>
</html>
```

- Run with `trunk serve`
---
layout: default
---

# Yew - trunk

- Custom configuration with `Trunk.toml`

```toml
[serve]
# The address to serve on LAN.
address = "127.0.0.1"
# The address to serve on WAN.
# address = "0.0.0.0"
# The port to serve on.
port = 8000
```
---
layout: section
---
# Tracing
---
layout: default
---

# Tracing

- span
- event
- subscriber
  
---
layout: default
---

# Tracing

- Crate `tracing` 
- Structured logging
```rust {all|6-12|all}
use tracing::{event, span, Level};

// records an event outside of any span context:
event!(Level::INFO, "something happened");

let span = span!(Level::INFO, "my_span");
let _guard = span.enter();

// records an event within "my_span".
event!(Level::INFO, "something happened inside my_span");

span.exit()
// records an event outside "my_span".
event!(Level::INFO, "something happened outside my_span");
```

---
layout: default
---

# Tracing subscription

- Crate `tracing-subscriber` 

```rust
// Standard output
let stdout_log = tracing_subscriber::fmt::layer().pretty();

// File output
let debug_file = File::create("debug.log").expect("Unable to create debug.log");
let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(debug_file));

tracing_subscriber::registry()
    .with(stdout_log.with_filter(filter::LevelFilter::INFO))
    .with(debug_log.with_filter(filter::LevelFilter::DEBUG))
    .init();

```
---
layout: default
---

# Tracing subscription - output

```txt
  2024-04-06T15:26:50.450537Z  INFO c06_tracing: something happened
    at src/main.rs:20

  2024-04-06T15:26:50.450799Z  INFO c06_tracing: something happened inside my_span
    at src/main.rs:27
    in c06_tracing::my_span

  2024-04-06T15:26:50.450901Z  INFO c06_tracing: something happened outside my_span
    at src/main.rs:31
```
---
layout: section
---
# Back-end

---
layout: default
---

# Backend

- Service
- Document storage
- Processing unit
- Synchronized & persistent state
- Accessible through network

---
layout: section
---
# Back-end
Tokio

---
layout: default
---

# Tokio

- Async runtime
- Built for network apps
- Large ecosystem



---
layout: default
---

# Tokio - Listener

```rust
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
```

---
layout: default
---

# Tokio - Shared state

- User status database (KV Storage)
- Key: username
- Value: status
- `Arc` + `Mutex` = ❤️

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::new()));

    // ...
}
```

---
layout: section
---
# Back-end
Axum

---
layout: default
---

# Axum

- [July 30, 2021](https://tokio.rs/blog/2021-07-announcing-axum)
- Tokio-based
- Simple usage
- Focused on web-development

---
layout: default
---

# Axum

```rust {all|6|15-17}
use axum::prelude::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
```
---
layout: default
---
# Axum - extractors

```rust {all|4-7|9}
use axum::{prelude::*, extract::Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

async fn create_user(Json(payload): Json<CreateUser>) {
    // `payload` is a `CreateUser`
}

let app = route("/users", post(create_user));
```
---
layout: default
---

# Axum - response data

- Handlers can return anything that implements IntoResponse and it will automatically be converted into a response

```rust {1-5|7-10|14-16}
// Returning a tuple of `StatusCode` and another `IntoResponse` will
// change the status code
async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "not found")
}

// `Html` gives a content-type of `text/html`
async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
```

---
layout: default
---

# Axum - routing

- Routes with params 
  
```rust
use axum::prelude::*;

let app = route("/", get(root))
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", get(show_user).delete(delete_user));
```

- Nesting

```rust
let user_routes = Router::new().route("/:id", get(|| async {}));

let team_routes = Router::new().route("/", post(|| async {}));

let api_routes = Router::new()
    .nest("/users", user_routes)
    .nest("/teams", team_routes);

let app = Router::new().nest("/api", api_routes);
```

---
layout: default
---

# Axum - state managment

- Local or DB
```rust {all|1-6|8-10|12-}
// Must be #[async-trait] implementation
type MyService = Arc<dyn service::MyService + Send + Sync>;

pub struct AppState {
    pub service: MyService,
}
// ...
let state = routes::RouterState {
    service: Arc::from(MyServiceImpl {}) as _,
};
// ... 
pub async fn get_all(State(state): State<RouterState>) -> Result<Json<Vec<Post>>, ApiError> {
    state
        .post_service
        .get_all()
        .await
        .map(Json)
        .map_err(Into::into)
}
```

---
layout: section
---

# Configuration

---
layout: default
---

# Configuration

- `dotenv` - Same implementation for multiple languages
- `figment` - Supports multiple config files

```rust {1-13|14-19|all}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostConfig {
    pub collection: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mongo: String,
    pub listen: SocketAddr,
    pub database: String,
    pub post: PostConfig,
}

pub fn config() -> Result<Config, figment::Error> {
    Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("MYAPP_"))
        .extract()
}
```


---
layout: section
---

# Database

---
layout: default
---

# Database - options

- Custom
- KV Storage
- SQL 
  - Postgres, MySQL, MariadDB
  - SQLite (Local DB)
- NoSQL
  - MongoDB, Cassandra
- Graph - Neo4j

---
layout: default
---

# Database - representation

- Raw - Queries
- ORM - Object relation modeling

---
layout: section
---

# Database
sqlx

---
layout: default
---

# sqlx

- Select DB with feature flags
- Support for PostgreSQL, MySQL, MariaDB and SQLite.

```rust
dotenv::dotenv().ok();
tracing_subscriber::fmt::init();

let app_state = AppState {
    pool: SqlitePoolOptions::new()
        .max_connections(5)
        .connect(
            env::var("DATABASE_URL")
                .unwrap_or("sqlite::memory:".to_string())
                .as_str(),
        )
        .await?,
};
```

---
layout: default
---

# sqlx - queries

- `query` method

```rust
 let result = sqlx::query(
        "
        INSERT INTO users (username)
        VALUES (?)
        ",
    )
    .bind(payload.username.clone())
    .execute(&app_state.pool)
    .await;
```

- powerful macros `query!` and `query_as!` (static query check)
  
```rust
let users = sqlx::query_as!(User,
    "SELECT * FROM users WHERE name = ?",
    name)
    .fetch_all(&pool) // -> Vec<User>
    .await?;
```

---
layout: section
---

# Database
diesel ORM

---
layout: default
---

# diesel

- ORM
- migrations
- schemas
- powerfull CLI `diesel_cli`
  

---
layout: default
---

# diesel - migration

- Commands to prepare diesel:
```sh
diesel setup
diesel migration generate create_users
```

- Migration to create user:
  
```sql
-- up.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL
);
-- down.sql
DROP TABLE users;
```

---
layout: default
---

# diesel - schema

- Run `diesel migration run` to generate schema

```rust
// Generated file src/schema.rs
diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}
```

- Create model

```rust
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
}
```
---
layout: default
---

# diesel - query

- Get users

```rust
pub async fn get_users() -> (StatusCode, Json<Vec<User>>) {
    let conn = &mut establish_connection();

    let users = users::table
        .load::<User>(conn)
        .expect("Error loading users");

    // this will be converted into a JSON response
    // with a status code of `200 OK`
    (StatusCode::OK, Json(users))
}
```





---
layout: section
---

# Serverless

---
layout: default
---

# AWS Lambda

- Code on the edge
- CLI
- `lambda` & `lambda_http` [crate](https://www.cargo-lambda.info/guide/what-is-cargo-lambda.html) for rust
- Cold boot == $$$

---
layout: default
---

# Simple lambda

```rust {all|3-10|12-21}
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request".into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
```

---
layout: default
---

# Lambda deploy

- Setup AWS CLI
- Build lambda `cargo lambda build --release`
- Deploy `cargo lambda deploy`


---
layout: default
---

# Cold boot

- [Experiment by Ervin Szilágyi](https://ervinszilagyi.dev/articles/running-serverless-lambdas-with-rust-aws.html)  

<img src="/images/cold_start.png" class="mx-25 w-170 rounded shadow" />

---
layout: section
---

# The end 🎉

---
layout: section
---

# Questions
