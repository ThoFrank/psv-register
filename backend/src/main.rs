use axum::{
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
    response::AppendHeaders,
    routing::{get, post},
    Router,
};
use clap::Parser;
use config::Config;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use handlebars::Handlebars;
use lazy_static::lazy_static;
use static_init::dynamic;
use std::net::SocketAddr;
use tower::ServiceExt;
use tower_http::services::ServeDir;

mod archer;
mod config;
mod db;
mod error;
mod models;
mod schema;

#[dynamic()]
pub static mut CONFIG: Config = Config::default();

#[dynamic()]
pub static mut HANDLEBARS: Handlebars<'static> = {
    let mut hlbs = Handlebars::new();
    hlbs.set_strict_mode(true);
    hlbs
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = config::CliConfig::parse();
    let mut figment = Figment::new().merge(Toml::file("config.toml")); // TODO add some sane default directories
    for config in &args.config_file {
        figment = figment.merge(Toml::file(config));
    }
    let config: Config = figment
        .merge(Env::prefixed("PSV_").split("__"))
        .merge(Serialized::defaults(args))
        .extract()
        .unwrap_or_else(|e| {
            eprintln!("Could not aggregate correct configuration!");
            eprintln!("{e}");
            std::process::exit(1)
        });
    *CONFIG.write() = config;

    db::establish_connection(&CONFIG.read().database_path)
        .run_pending_migrations(MIGRATIONS)
        .expect("Could not migrate database");
    {
        let mut handlebars = HANDLEBARS.write();
        // handlebars.set_strict_mode(true);
        handlebars.set_dev_mode(cfg!(debug_assertions));
        handlebars
            .register_template_string("user_mail_en", include_str!("../user_mail_en.tpl"))
            .unwrap();
        handlebars
            .register_template_string("user_mail", include_str!("../user_mail.tpl"))
            .unwrap();
    }

    let api = Router::new()
        .route("/archers", post(archer::create_archers))
        .route("/archers", get(archer::list_archers));
    let app = Router::new()
        .nest_service(
            "/",
            get(handler).then(|res| async move {
                Ok((
                    AppendHeaders([(axum::http::header::CACHE_CONTROL, "no-cache")]),
                    res,
                ))
            }),
        )
        .nest_service("/api", api);

    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.read().port));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = get_static_file(uri.clone()).await?;

    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    lazy_static! {
        static ref HTML_PATH: String =
            std::env::var("WEBPAGE").unwrap_or_else(|_| "../frontend/dist".to_string());
    }

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    match ServeDir::new(&*HTML_PATH).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
