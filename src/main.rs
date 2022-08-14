use std::fmt::format;
use actix::{Addr, SyncArbiter};
use resizer::* ;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web::web::{Data, Payload, Query};
use std::future::Future;
use futures_util::stream::StreamExt as _;
use resizer::errors::ImageErrors;
use resizer::image_actor::{ResizeActor, ResizeRequest};
use serde::Deserialize;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
        // lunching the resize actor with 4 threads
        let resize_actor = SyncArbiter::start(4 , ||ResizeActor,);

        HttpServer::new( move || {
            App::new()
                .app_data(Data::new(
                    State {
                        resizer : resize_actor.clone()
                    }
                ))
                .route("/resize", web::post().to(handle_image_request))
        })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await

   
}

async fn handle_image_request (state : Data<State>, params : Query<ImageResizingRequest>, mut payload: Payload) ->  actix_web::Result<String> {
    let (height,width) = (params.height,params.width);
    let resize =state.resizer.clone();
    // todo : apply the resizer actor to the request body with the QueryParams
    let mut bytes = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        bytes.extend_from_slice(&item?);
    }
    let msg = ResizeRequest{
        data: bytes.to_vec(),
        width,
        height

    } ;
    // todo : find a way to write the new image
    let _ = resize.send(msg) ;
    Ok("done".to_string())

}
struct State {
    resizer : Addr<ResizeActor>
}
#[derive(Deserialize)]
struct ImageResizingRequest {
    width : u32 ,
    height : u32 ,
}