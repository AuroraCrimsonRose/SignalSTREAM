use futures::StreamExt;
use std::{collections::HashMap, convert::Infallible, net::SocketAddr, sync::Arc};
use tokio_stream::wrappers::BroadcastStream;
use warp::hyper::Body;

use crate::route::StreamRouter;
use crate::server::status::handle_status;
use crate::station::manager::StationManager;
use warp::{
    fs::{dir, File},
    http::{Response as HttpResponse, StatusCode},
    reply::Response as WarpResponse,
    Filter, Rejection, Reply,
};

/// Inject a map of `Arc<StreamRouter>` into your handlers
fn with_map(
    map: Arc<HashMap<String, Arc<StreamRouter>>>,
) -> impl Filter<Extract = (Arc<HashMap<String, Arc<StreamRouter>>>,), Error = Infallible> + Clone {
    warp::any().map(move || map.clone())
}

pub async fn serve_all_routes(managers: Vec<StationManager>) {
    // Start each StationManager & collect their routers
    let mut router_map = HashMap::new();
    for manager in managers {
        manager.start();
        router_map.insert(manager.config.id.clone(), manager.router.clone());
    }
    let shared_map = Arc::new(router_map);

    // MP3 streaming route
    let mp3_route = {
        let routers = shared_map.clone();
        warp::path!(String)
            .and(warp::path::end())
            .and_then(move |filename: String| {
                let routers = routers.clone();
                async move {
                    if filename.ends_with(".mp3") {
                        let station = filename.trim_end_matches(".mp3");
                        if let Some(router) = routers.get(station) {
                            let mut rx = router.tx.subscribe();
                            // Use an async stream that waits for audio data and never ends
                            let stream = BroadcastStream::new(rx).filter_map(|res| {
                                futures::future::ready(
                                    res.ok()
                                        .map(warp::hyper::body::Bytes::from)
                                        .map(|b| Ok::<_, Infallible>(b)),
                                )
                            });
                            let body = Body::wrap_stream(stream);
                            let response = HttpResponse::builder()
                                .header("Content-Type", "audio/mpeg")
                                .body(body)
                                .unwrap();
                            Ok::<_, Infallible>(response.into_response())
                        } else {
                            let not_found = HttpResponse::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::from("Not Found"))
                                .unwrap();
                            Ok::<_, Infallible>(not_found.into_response())
                        }
                    } else {
                        let not_found = HttpResponse::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("Not Found"))
                            .unwrap();
                        Ok::<_, Infallible>(not_found.into_response())
                    }
                }
            })
    };

    // DASH static under /dash/<station>/*
    let dash_route = warp::path("dash").and(warp::fs::dir("public/dash"));

    // Index at `/`
    let index = warp::path::end().map(|| WarpResponse::new(include_str!("player.html").into()));

    // Status endpoint at /status/{station}
    let status_route = warp::path!("status" / String)
        .and(with_map(shared_map.clone()))
        .and_then(handle_status);

    // Combine & serve
    let routes = mp3_route
        .or(dash_route)
        .or(index)
        .or(status_route)
        .with(warp::log("signalstream"));

    warp::serve(routes).run(([127, 0, 0, 1], 9090)).await;
}
