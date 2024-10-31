const MAX_WIDTH_IN_PX: usize = 25;
const MAX_HEIGHT_IN_PX: usize = 25;

#[derive(Debug)]
struct FrameBuffer {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
}

impl FrameBuffer {
    fn new(width_px: usize, height_px: usize) -> FrameBuffer {
        assert!(width_px < MAX_WIDTH_IN_PX + 1);
        assert!(height_px < MAX_HEIGHT_IN_PX + 1);

        FrameBuffer {
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
        println!("");
    }

    fn draw_pixel(&mut self, x_pos: usize, y_pos: usize) {
        assert!(!self.buffer.is_empty());
        assert!(x_pos < self.width);
        assert!(y_pos < self.height);

        let pixel_position: usize = ((y_pos + 1) * self.width) - (self.width - x_pos);

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
            println!("x_pos= {}, y_pos = {}",x as usize, y as usize);
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

    fn draw_rect(&mut self) {
        todo!("Not implemented yet!");
    }

    fn fill_rect(&mut self) {
        todo!("Not implemented yet!");
    }
}

fn main() {
    let mut tft_display = FrameBuffer::new(25, 25);

    println!(
        "The framebuffer has been created: {}x{} px",
        tft_display.width, tft_display.height
    );

    tft_display.show_buffer();

    tft_display.draw_line((0, 15), (1, 5));
    tft_display.show_buffer();
}
