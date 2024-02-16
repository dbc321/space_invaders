use ggez::{graphics::{Canvas, Image}, mint::Point2, Context};


pub(crate) enum GameObjectPosition {
    Left,
    Right,
    Top,
    Bottom
}

pub(crate) struct GameObject {
   texture: Image,
   pub width: f32,
   pub height: f32
}

impl GameObject {
    pub fn new(_ctx: &Context, image: Image) -> GameObject {
        let width = image.width() as f32;
        let height = image.height() as f32;
        
        GameObject {
           texture: image,
           width: width,
           height: height
        }
    }

    pub fn move_to_position(&self, position: Point2<f32>, canvas: &mut Canvas) {
        canvas.draw(
            &self.texture,
            position,
        );
    }

    
}
