use std::cmp::Ordering;

use sfml::{graphics::{Color, Image, Sprite, Texture}, system::{Vector2f, Vector3i}};
use sfml::graphics::Transformable;


pub struct Block<'a> {
    pub coords: Vector3i,
    pub sprite: Sprite<'a>,
    pub faces: &'a BlockFaces<'a>
}

impl std::fmt::Debug for Block<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
         .field("coords", &self.coords)
         .finish()
    }
}

impl<'a> Block<'a> {
    pub fn new(coords: Vector3i, texture: &'a Texture, faces: &'a BlockFaces) -> Self {
        let mut sprite = Sprite::new();
        sprite.set_texture(&texture, false);
        sprite.set_position(grid_to_screen(&coords));
        return Self { coords, sprite, faces };
    }

    /// Returns grid coordinates of a block placed on clicked face of this block.
    /// Coordinates are in 2d world space.
    /// If there's no intersections, returns None
    pub fn get_coords_of_clicked_face(&self, point: &Vector2f) -> Option<Vector3i> {
        if self.sprite.global_bounds().contains(*point) {
            let x = point.x - self.sprite.global_bounds().left;
            let y = point.y - self.sprite.global_bounds().top;

            if self.faces.face_x.pixel_at(x as u32, y as u32).eq(&Color::WHITE) {
                return Some((1,0,0).into());
            }
            if self.faces.face_y.pixel_at(x as u32, y as u32).eq(&Color::WHITE) {
                return Some((0,1,0).into());
            }
            if self.faces.face_z.pixel_at(x as u32, y as u32).eq(&Color::WHITE) {
                return Some((0,0,1).into());
            }
        }
        return None; 
        
    }
}

impl Ord for Block<'_> {
    fn cmp(&self, o: &Self) -> Ordering {
        match self.coords.z.cmp(&o.coords.z) {
            std::cmp::Ordering::Equal => {
                 match self.coords.y.cmp(&o.coords.y) {
                     std::cmp::Ordering::Equal => return self.coords.x.cmp(&o.coords.x),
                     n => return n
                 }
            },
            n => return n
        }
    }
}


impl PartialOrd for Block<'_> {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        match self.coords.z.cmp(&o.coords.z) {
            std::cmp::Ordering::Equal => {
                 match self.coords.y.cmp(&o.coords.y) {
                     std::cmp::Ordering::Equal => return Some(self.coords.x.cmp(&o.coords.x)),
                     n => return Some(n)
                 }
            },
            n => return Some(n)
        }
    }
}

impl PartialEq for Block<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.coords.eq(&other.coords)
    }
}
impl Eq for Block<'_> { }




fn grid_to_screen(c: &Vector3i) -> (f32,f32) {
    return (
        ((c.x * 18 - c.y * 18) as f32),
        ((c.x *  9 + c.y * 9 - c.z * 22) as f32)
    );

}

pub struct BlockFaces<'a> {
    pub face_x: &'a Image,
    pub face_y: &'a Image,
    pub face_z: &'a Image
}
