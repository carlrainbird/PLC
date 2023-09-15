use fltk::image::SvgImage as FltkImage;
//use fltk::{app, enums::FrameType, frame::Frame,  prelude::*};

pub trait ImageWidget {
    fn create_image(&self) -> FltkImage;
}

pub struct ImageFactory;

impl ImageWidget for ImageFactory {
    //let mut Object = fltk::image::SvgImage;
    fn create_image(&self) -> FltkImage {
   
        let image = FltkImage::load("C:/Developer/PLC/src/images/arrow.svg").unwrap();     
        image
    }
}
