pub struct Obstacle{
    
    pub height: f32, 
    pub width: f32, 
    pub x_coordinate: f32, 
    pub y_coordinate: f32, 


}

impl Obstacle{

    pub fn contains(&self, point: (f32, f32)) -> bool{

        point.0 >= self.x_coordinate && point.0 <= self.x_coordinate+self.width && point.1 >= self.y_coordinate && point.1 <= self.y_coordinate+self.height 
          
    }
}