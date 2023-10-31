use cfg_if::cfg_if;

cfg_if!{
if #[cfg(feature = "ssr")] {
    use leptos::LeptosOptions;
    use axum::extract::FromRef;

    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        leptos_options: LeptosOptions,
    }
}}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body as AxumBody,
        extract::{Path, RawQuery, State},
        response::{IntoResponse, Response},
        routing::{post, get},
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
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config)
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

    
    async fn leptos_routes_handler(
        session_store: SessionPgSession,
        State(app_state): State<AppState>,
        req: Request<AxumBody>,
    ) -> Response {
        let handler = leptos_axum::render_app_to_stream_with_context(
            app_state.leptos_options.clone(),
            move || {
                provide_context(session_store.clone());
            },
            App,
        );
        handler(req).await.into_response()
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

    let app_state = AppState{
        leptos_options,
    };

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .layer(SessionLayer::new(session_store))
        .fallback(file_and_error_handler)
        .with_state(app_state);

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
