pub trait Layout{
    fn set_position(&mut self,x:i32,y:i32,width:i32,height:i32)->&Self;
}

pub struct Position{
    x : i32,
    y : i32,
}

impl Layout for Position{
    fn set_position(&mut self,x:i32,y:i32,width:i32,height:i32)->&Self{
    self.y = y;
    self.x = width + x;
    self
    }
}