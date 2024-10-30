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

        let pixel_position: usize = ((y_pos + 1)*self.width ) - (self.width - x_pos);

        assert!(pixel_position < self.buffer.len());

        self.buffer[pixel_position] = 1;

    }

    fn draw_line(&mut self) {
        todo!("Not implemented yet!");
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
    tft_display.draw_pixel(3, 2);

    tft_display.show_buffer();

    tft_display.fill_rect();
}
