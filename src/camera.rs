use tetra::glm::Vec2;

pub struct Camera{
    save_zone: Vec2,
    delta: u32,
    speed: u32,
    new_offset: Vec2,
    pub offset: Vec2,
    smooth: bool,
}

impl Camera{
    pub fn new<P>(params: P) ->Camera where P: Into<CameraParams>, {
        let params = params.into();
        Camera {
            save_zone: params.save_zone,
            delta: params.delta,
            speed: params.speed,
            new_offset: params.new_offset,
            offset: params.offset,
            smooth: params.smooth,
        }
    }

    pub fn update(&mut self){
        if self.smooth{
            self.delta += 1;
            if self.delta >= self.speed{
                self.delta = 0;
                let mut x = 0.0;
                let mut y = 0.0;
                if self.offset != self.new_offset {
                    if self.offset.x < self.new_offset.x {
                        x = 1.0;
                    }if self.offset.x > self.new_offset.x {
                        x = -1.0;
                    }
                    if self.offset.y < self.new_offset.y {
                        y = 1.0;
                    }if self.offset.y > self.new_offset.y {
                        y = -1.0;
                    }
                    self.offset += Vec2::new(x,y);
                }
            }
        }
    }

    pub fn center_on(&mut self,position: Vec2) -> &mut Self{
        if self.smooth{
            let dif = position - self.new_offset;
            if dif.x > self.save_zone.x || dif.x < -self.save_zone.x || dif.y > self.save_zone.y || dif.y < -self.save_zone.y{
                self.new_offset = position;
            }
            return self;
        }else{
            self.offset = position;
            return self;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CameraParams {
    save_zone: Vec2,
    delta: u32,
    speed: u32,
    new_offset: Vec2,
    offset: Vec2,
    smooth: bool,
}

impl CameraParams {
    /// Creates a new set of `CameraParams`.
    pub fn new() -> CameraParams {
        CameraParams::default()
    }

    /// Sets the speed.
    pub fn speed(mut self, speed: u32) -> CameraParams {
        self.speed = speed;
        self
    }

    /// Sets the smooth.
    pub fn smooth(mut self, smooth: bool) -> CameraParams {
        self.smooth = smooth;
        self
    }

    /// Sets the save zone.
    pub fn save_zone(mut self, save_zone: Vec2) -> CameraParams {
        self.save_zone = save_zone;
        self
    }

    /// Sets the position.
    pub fn position(mut self, position: Vec2) -> CameraParams {
        self.offset = position;
        self.new_offset = position;
        self
    }

}

impl Default for CameraParams {
    fn default() -> CameraParams {
        CameraParams {
            save_zone: Vec2::new(10.0,10.0),
            delta: 0,
            speed: 14,
            new_offset: Vec2::new(0.0,0.0),
            offset: Vec2::new(0.0,0.0),
            smooth: true,
        }
    }
}
impl From<Vec2> for CameraParams {
    fn from(init_position: Vec2) -> CameraParams {
        CameraParams {
            new_offset: init_position,
            offset: init_position,
            ..CameraParams::default()
        }
    }
}