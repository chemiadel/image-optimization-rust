use actix::{Actor, Handler, Message, SyncContext};
use image::imageops::FilterType;
use image::ImageResult;

type ImageData = Vec<u8> ;
pub struct ResizeRequest {
    pub data : ImageData ,
    pub width : u32 ,
    pub height : u32 ,
}
impl Message for ResizeRequest {
    type Result = ImageResult<ImageData> ;
}
pub struct ResizeActor ;
impl Actor for ResizeActor {
    type Context = SyncContext<Self>;
}
impl Handler<ResizeRequest> for ResizeActor {
    type Result = ImageResult<ImageData>;

    fn handle(&mut self, msg: ResizeRequest, _: &mut Self::Context) -> Self::Result {
        let mut new_image_data = Vec::new(); // todo : init with capacity
        let image_mime = image::guess_format(&msg.data)?;
        let resized_image = image::load_from_memory(
            &msg.data
        )?
            .resize(msg.width , msg.height , FilterType::Nearest);
        resized_image.write_to(&mut new_image_data, image_mime)?;
        Ok(new_image_data)
    }
}