#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body as AxumBody,
        extract::{Path, RawQuery},
        response::IntoResponse,
        routing::post,
        Router,
    };
    use axum_session::*;
    use dotenv;
    use http::{HeaderMap, Request};
    use leptos::*;
    use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
    use staff::app::*;
    use staff::fileserv::file_and_error_handler;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    dotenv::dotenv().ok();

    staff::database::init_db()
        .await
        .expect("Should create database pool");

    let pool = staff::database::get_db();

    let session_config = SessionConfig::default().with_table_name("user_sessions");
    let session_store: SessionStore<SessionPgPool> =
        SessionStore::new(Some(pool.clone().into()), session_config)
            .await
            .expect("session store could not be created");

    async fn server_fn_handler(
        session_store: SessionPgSession,
        path: Path<String>,
        headers: HeaderMap,
        raw_query: RawQuery,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        log::info!("{:?}", path);

        handle_server_fns_with_context(
            path,
            headers,
            raw_query,
            move || {
                provide_context(session_store.clone());
            },
            request,
        )
        .await
    }

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes(&leptos_options, routes, App)
        .layer(SessionLayer::new(session_store))
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
