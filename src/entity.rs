use raylib::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct EntityData{
    pub position: Vector2,
    pub velocity: Vector2,
    pub grounded: bool,
}

pub trait Entity{
    fn data(&self) -> &EntityData;
    fn data_mut(&mut self) -> &mut EntityData;
    fn get_size(&self) -> Vector2;

    fn get_velocity(&self) -> Vector2{
        self.data().velocity
    }
    fn set_velocity(&mut self, velocity: Vector2){
        self.data_mut().velocity = velocity;
    }
    fn get_position(&self) -> Vector2{
        self.data().position
    }
    fn set_position(&mut self, position: Vector2){
        self.data_mut().position = position;
    }
    fn get_bounding_box(&self) -> Rectangle {
        let pos = self.get_position();
        let size = self.get_size();
        Rectangle{
            x: pos.x,
            y: pos.y,
            width: size.x,
            height: size.y,
        }
    }
    fn get_center(&self) -> Vector2{
        let bb = self.get_bounding_box();
        return Vector2 { x: bb.x + bb.width * 0.5, y: bb.y + bb.height * 0.5 };
    }

    fn is_on_ground(&self) -> bool{
        self.data().grounded
    }

}