use super::graphics::consts;

#[derive(Clone, PartialEq)]
pub struct Position{
    pub x: i16,
    pub y: i16
}

impl Position {

    pub fn add_x(&mut self, val: i16){
        self.x += val;

        while self.x > consts::WORLD_WIDTH - 1{
            self.x -= 1;
        }


        while self.x < 0{
            self.x += 1;
        }
    }

    pub fn add_y(&mut self, val: i16){
        self.y += val;

        while self.y > consts::WORLD_HEIGHT - consts::STATUS_BAR_HEIGHT - 1{
            self.y -= 1;
        }

        while self.y <= 0{
            self.y += 1;
        }
    }

}
