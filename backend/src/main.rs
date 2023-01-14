use axum::{
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
    routing::{get, post},
    Router,
};
use clap::Parser;
use config::Config;
use handlebars::Handlebars;
use lazy_static::lazy_static;
use static_init::dynamic;
use std::net::SocketAddr;
use tower::ServiceExt;
use tower_http::services::ServeDir;

mod archer;
mod config;

#[dynamic()]
pub static mut CONFIG: Config = Config::default();

#[dynamic()]
pub static mut HANDLEBARS: Handlebars<'static> = Handlebars::new();

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(long, default_value_t = String::from("."))]
    config_dir: String,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    *CONFIG.write() = load_config(&std::path::PathBuf::from(&args.config_dir).join("config.toml"));
    {
        let mut handlebars = HANDLEBARS.write();
        handlebars.set_strict_mode(true);
        handlebars.set_dev_mode(cfg!(debug_assertions));
        handlebars
            .register_template_file(
                "user_mail",
                std::path::PathBuf::from(args.config_dir).join("user_mail.tpl"),
            )
            .unwrap();
    }

    let api = Router::new()
        .route("/archers", post(archer::create_archer))
        .route("/archers", get(archer::list_archers));
    let app = Router::new()
        .nest_service("/", get(handler))
        .nest_service("/api", api);

    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.read().port));
    println!("listening on {}", addr);
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
            std::env::var("WEBPAGE").unwrap_or("../frontend/dist".to_string());
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

fn load_config(path: &std::path::Path) -> Config {
    let toml_config =
        std::fs::read_to_string(path).expect(&format!("Couldn't read file from path {:?}", path));
    toml::from_str(&toml_config).expect(&format!("Couldn't parse config content!"))
}
