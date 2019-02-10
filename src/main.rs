use warp::{Filter, Future, Stream};

fn main() {
    //pretty_env_logger::init();

    let routes = warp::path("echo")
        .and(warp::ws2())                           // The `ws2()` filter will prepare the Websocket handshake.
        .map(|ws: warp::ws::Ws2| {
            ws.on_upgrade(|websocket| {             // And then our closure will be called when it completes...
                let (tx, rx) = websocket.split();   // Just echo all messages back...

                rx.forward(tx).map(|_| ()).map_err(|e| {
                    eprintln!("websocket error: {:?}", e);
                })
            })
        });

    //warp::serve(routes).run(([127,0,0,1], 3030));
    warp::serve(routes).run(([0;16], 3030));
}
