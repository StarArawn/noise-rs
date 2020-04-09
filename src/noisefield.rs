pub trait NoiseField {}

const MAX_GRID_SIZE: u16 = 32_767;

#[derive(Copy, Clone, Debug)]
struct GridSize {
    width: usize,
    height: usize,
}

// pub struct NoiseField1D {
//     x: Vec<f64>,
//
//     values: Vec<f64>,
// }

#[derive(Clone, Debug)]
pub struct NoiseField2D {
    grid_size: GridSize,

    // field_size: (f64, f64),
    // field_origin: (f64, f64),

    pub x: Vec<f64>,
    pub y: Vec<f64>,

    pub values: Vec<f64>,
}

// pub struct NoiseField3D {
//     x: Vec<f64>,
//     y: Vec<f64>,
//     z: Vec<f64>,
//
//     values: Vec<f64>,
// }
//
// pub struct NoiseField4D {
//     x: Vec<f64>,
//     y: Vec<f64>,
//     z: Vec<f64>,
//     w: Vec<f64>,
//
//     values: Vec<f64>,
// }

// impl NoiseField for NoiseField1D {}
impl NoiseField for NoiseField2D {}
// impl NoiseField for NoiseField3D {}
// impl NoiseField for NoiseField4D {}

impl NoiseField2D {
    // pub fn new(grid_size: (usize, usize), field_size: (f64, f64), field_origin: (f64, f64)) -> Self {
    pub fn new(grid_width: usize, grid_height: usize) -> Self {
        // let (grid_width, grid_height) = grid_size;

        // Check for invalid grid width or height.
        //TODO: Return an error here instead of panicking
        assert!(grid_width < MAX_GRID_SIZE as usize);
        assert!(grid_height < MAX_GRID_SIZE as usize);

        // let (field_width, field_height) = field_size;
        //
        // // Check for invalid field width or height
        // //TODO: Return an error here instead of panicking
        // assert!((0.0 - field_width).abs() < std::f64::EPSILON);
        // assert!((0.0 - field_height).abs() < std::f64::EPSILON);

        let grid_size = grid_width * grid_height;

        Self {
            grid_size: GridSize {
                width: grid_width,
                height: grid_height,
            },

            // field_size,
            // field_origin,

            x: vec![0.0; grid_size],
            y: vec![0.0; grid_size],

            values: vec![0.0; grid_size],
        }
    }

    pub fn get_grid_size(&self) -> (usize, usize) {
        (self.grid_size.width, self.grid_size.height)
    }

    pub fn set_field_coord(&mut self, grid_point: (usize, usize), coordinate: (f64, f64)) {
        let (x, y) = coordinate;

        let index = self.get_index_for_grid_point(grid_point);

        self.x[index] = x;
        self.y[index] = y;

    }

    pub fn get_field_coord(&self, grid_point: (usize, usize)) -> (f64, f64) {
        let index = self.get_index_for_grid_point(grid_point);

        (self.x[index], self.y[index])
    }

    pub fn get_value_at_grid_point(&self, grid_point: (usize, usize)) -> f64 {
        let index = self.get_index_for_grid_point(grid_point);

        self.values[index]
    }

    fn get_index_for_grid_point(&self, grid_point: (usize, usize)) -> usize {
        // Y
        // |
        // 2 | 6 7 8
        // 1 | 3 4 5
        // 0 | 0 1 2
        // --|------
        //   | 0 1 2 - X

        let (x, y) = grid_point;

        x + (self.grid_size.width * y)
    }

    pub fn initialize() -> Self {
        Self {
            grid_size: GridSize {
                width: 0,
                height: 0,
            },

            // field_size: (0.0, 0.0),
            // field_origin: (0.0, 0.0),

            x: Vec::new(),
            y: Vec::new(),

            values: Vec::new(),
        }
    }
}

// impl Default for NoiseField2D {
//     fn default() -> Self {
//         Self::initialize()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_noisefield2d() {
        let noisefield = NoiseField2D::new(1, 1);

        assert_eq!(noisefield.grid_size.width, 1);
        assert_eq!(noisefield.grid_size.height, 1);
    }

    #[test]
    #[should_panic]
    fn create_too_small_noisefield2d_x() {
        let noisefield = NoiseField2D::new(0, 1);
    }

    #[test]
    #[should_panic]
    fn create_too_small_noisefield2d_y() {
        let noisefield = NoiseField2D::new(1, 0);
    }

    #[test]
    fn get_index() {
        let index = NoiseField2D::new(3, 3).get_index_for_grid_point((1,1));

        assert_eq!(index, 4);
    }

    #[test]
    fn set_coord() {
        let grid_point = (1, 1);
        let mut noisefield = NoiseField2D::new(3, 3);
        noisefield.set_field_coord(grid_point, (0.1, 1.5));
        let index = noisefield.get_index_for_grid_point(grid_point);

        assert_eq!(0.1, noisefield.x[index]);
        assert_eq!(1.5, noisefield.y[index]);
    }
}