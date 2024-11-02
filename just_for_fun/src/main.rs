//! 
pub(crate) const MAX_WIDTH_IN_PX: usize = 1024;
pub(crate) const MAX_HEIGHT_IN_PX: usize = 768;

#[derive(Debug)]
struct CanvasBuffer {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
}

impl CanvasBuffer {
    fn new(width_px: usize, height_px: usize) -> CanvasBuffer {
        assert!(width_px < MAX_WIDTH_IN_PX + 1);
        assert!(height_px < MAX_HEIGHT_IN_PX + 1);

        CanvasBuffer {
            width: width_px,
            height: height_px,
            buffer: vec![0; width_px * height_px],
        }
    }
    
    //todo: implement Display trait!
    fn show_buffer(&self) {
        assert!(!self.buffer.is_empty());

        let mut lower_bound = 0;
        let mut higher_bound = self.width;

        for row in 0..self.height {
            let buffer_row = &self.buffer[lower_bound..higher_bound];

            lower_bound += self.width;
            higher_bound += self.width;

            println!("{row}:    {:?}", buffer_row);
        }
        println!();
    }

    fn draw_pixel(&mut self, x_pos: usize, y_pos: usize) {
        assert!(!self.buffer.is_empty());
        assert!(x_pos < self.width);
        assert!(y_pos < self.height);

        // let pixel_position: usize = ((y_pos + 1) * self.width) - (self.width - x_pos);
        let pixel_position: usize = x_pos + y_pos * self.width;

        assert!(pixel_position < self.buffer.len());

        self.buffer[pixel_position] = 1;
    }

    //Bresenham's line algorithm
    fn draw_line(&mut self, x_coord: (usize, usize), y_coord: (usize, usize)) {
        // Check whether line coordinates are correct or not
        assert!((0..self.width).contains(&x_coord.0));
        assert!((0..self.width).contains(&x_coord.1));
        assert!((0..self.height).contains(&y_coord.0));
        assert!((0..self.height).contains(&y_coord.1));

        let delta_x = x_coord.1.abs_diff(x_coord.0);
        let delta_y = y_coord.1.abs_diff(y_coord.0);

        let mut error = (delta_x as isize) - (delta_y as isize);

        let sign_x = if x_coord.0 < x_coord.1 { 1 } else { -1 };

        let sign_y = if y_coord.0 < y_coord.1 { 1 } else { -1 };

        let mut x: isize = x_coord.0 as isize;
        let mut y: isize = y_coord.0 as isize;

        'inf_loop: loop {
            self.draw_pixel(x as usize, y as usize);

            if x as usize == x_coord.1 && y as usize == y_coord.1 {
                break 'inf_loop;
            }

            let error_2 = error * 2;

            if error_2 > -(delta_y as isize) {
                error -= delta_y as isize;
                x += sign_x;
            }

            if error_2 < delta_x as isize {
                error += delta_x as isize;
                y += sign_y;
            }
        }
    }

    fn draw_rect(&mut self, x_coord: (usize, usize), y_coord: (usize, usize)) {
        self.draw_line((x_coord.0, x_coord.1), (y_coord.0, y_coord.0)); // Top line
        self.draw_line((x_coord.0, x_coord.1), (y_coord.1, y_coord.1)); // Bottom line

        self.draw_line((x_coord.0, x_coord.0), (y_coord.0, y_coord.1)); // Left line
        self.draw_line((x_coord.1, x_coord.1), (y_coord.0, y_coord.1)); // Right line
    }

    fn fill_rect(&mut self, x_coord: (usize, usize), y_coord: (usize, usize), value: u8) {
        // Check whether line coordinates are correct or not
        assert!((0..self.width).contains(&x_coord.0));
        assert!((0..self.width).contains(&x_coord.1));
        assert!((0..self.height).contains(&y_coord.0));
        assert!((0..self.height).contains(&y_coord.1));

        for row in y_coord.0..=y_coord.1 {
            let lower_bound = x_coord.0 + row * self.width;
            let higher_bound = x_coord.1 + row * self.width;

            println!("Row value is {row}");
            self.buffer[lower_bound..=higher_bound].fill(value);
        }
    }

    fn fill_buffer(&mut self, value: u8) {
        self.buffer = vec![value; self.width * self.height];
    }
}

fn main() {
    let mut tft_display = CanvasBuffer::new(10, 10);

    println!(
        "The framebuffer has been created: {}x{} px",
        tft_display.width, tft_display.height
    );

    tft_display.fill_rect((0, tft_display.width - 1), (0, tft_display.height - 1), 1);
    tft_display.show_buffer();
}
