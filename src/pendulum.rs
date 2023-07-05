use glam::DVec2;

struct Pendulum {
    rotation_center: DVec2,
    end_point: DVec2,
    length: f64,
    angle: f64,
    mass: f64,
}

impl Pendulum {
    pub fn new(rotation_center: &DVec2, length: &f64, angle: &f64, mass: &f64) -> Pendulum {
        return Pendulum {
            rotation_center: *rotation_center,
            end_point: *rotation_center + DVec2 { x: 0f64, y: -(*length) },
            length: *length,
            angle: *angle,
            mass: *mass
        }
    }
    pub fn default() -> Pendulum {
        return Pendulum {
            rotation_center: DVec2 { x: 0f64, y: 0f64 },
            end_point: DVec2 { x: 0f64, y: 0f64 } + DVec2 { x: 0f64, y: -1f64},
            length: 1f64,
            angle: 0f64,
            mass: 1f64
        }
    }
    pub fn update(self: &Self, ) {
        // should this function take in an a time in nano seconds representing the time passed since current state and change its angle acc
    }
}