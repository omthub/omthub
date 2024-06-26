pub mod fileserv;

use axum::{
  body::Body,
  extract::{FromRef, State},
  http::Request,
  response::{IntoResponse, Response},
  routing::get,
  Router,
};
use color_eyre::eyre::{Context, Result};
use leptos::*;
use leptos_axum::{
  generate_route_list, handle_server_fns_with_context, LeptosRoutes,
};
use leptos_router::RouteListing;
use site_app::App;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tracing_subscriber::prelude::*;

use self::fileserv::file_and_error_handler;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
  pub leptos_options: LeptosOptions,
  pub routes:         Vec<RouteListing>,
  pub db:             db::DbConnection,
}

async fn server_fn_handler(
  session: tower_sessions::Session,
  auth_session: auth::AuthSession,
  State(app_state): State<AppState>,
  request: Request<Body>,
) -> impl IntoResponse {
  handle_server_fns_with_context(
    move || {
      provide_context(auth_session.clone());
      provide_context(session.clone());
      provide_context(core_types::LoggedInUser(
        auth_session.user.clone().map(core_types::PublicUser::from),
      ));
      provide_context(app_state.db.clone());
    },
    request,
  )
  .await
}

async fn leptos_routes_handler(
  session: tower_sessions::Session,
  auth_session: auth::AuthSession,
  State(app_state): State<AppState>,
  req: Request<Body>,
) -> Response {
  let handler = leptos_axum::render_route_with_context(
    app_state.leptos_options.clone(),
    app_state.routes.clone(),
    move || {
      provide_context(auth_session.clone());
      provide_context(session.clone());
      provide_context(core_types::LoggedInUser(
        auth_session.user.clone().map(core_types::PublicUser::from),
      ));
      provide_context(app_state.db.clone());
    },
    site_app::App,
  );
  handler(req).await.into_response()
}

#[tokio::main]
async fn main() -> Result<()> {
  color_eyre::install().expect("Failed to install color_eyre");

  #[cfg(not(feature = "chrome-tracing"))]
  {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
      .unwrap_or(tracing_subscriber::EnvFilter::new(
        "info,site_server=debug,site_app=debug",
      ));
    let error_layer = tracing_error::ErrorLayer::default();
    tracing_subscriber::registry()
      .with(filter)
      .with(error_layer)
      .with(tracing_subscriber::fmt::layer())
      .init();
  }
  #[cfg(feature = "chrome-tracing")]
  let guard = {
    let (chrome_layer, guard) =
      tracing_chrome::ChromeLayerBuilder::new().build();
    tracing_subscriber::registry().with(chrome_layer).init();
    guard
  };

  let db = db::DbConnection::new().await?;
  db.run_migrations()
    .await
    .wrap_err("failed to run db migrations")?;
  log::info!("ran migrations");

  // Setting get_configuration(None) means we'll be using cargo-leptos's env
  // values For deployment these variables are:
  // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
  // Alternately a file can be specified such as Some("Cargo.toml")
  // The file would need to be included with the executable when moved to
  // deployment
  let conf = get_configuration(None).await.unwrap();
  let leptos_options = conf.leptos_options;
  let addr = leptos_options.site_addr;
  let routes = generate_route_list(App);
  let state = AppState {
    leptos_options,
    routes: routes.clone(),
    db,
  };

  let auth_layer = auth::build_auth_layer().await?;

  // build our application with a route
  let app = Router::new()
    .route(
      "/api/*fn_name",
      get(server_fn_handler).post(server_fn_handler),
    )
    .leptos_routes_with_handler(routes, get(leptos_routes_handler))
    .fallback(file_and_error_handler)
    .layer(
      ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(auth_layer),
    )
    .with_state(state);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log::info!("listening on http://{}", &addr);
  let socket = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(socket, app).await.unwrap();

  #[cfg(feature = "chrome-tracing")]
  drop(guard);

  Ok(())
}
