#[derive(Debug)]
enum ColorDepth {
    Rgb565,
    Rgb888,
    Rgb8888,
}

const MAX_WIDTH_IN_PX: u16 = 25;
const MAX_HEIGHT_IN_PX: u16 = 25;

#[derive(Debug)]
struct FrameBuffer {
    width: u16,
    height: u16,
    depth: ColorDepth,
    buffer: Vec<u8>,
}

impl FrameBuffer {
    fn new(width_px: u16, height_px: u16, color_scheme: ColorDepth) -> FrameBuffer {

        assert!(width_px < MAX_WIDTH_IN_PX + 1);
        assert!(height_px < MAX_HEIGHT_IN_PX + 1);

        let frame_buffer_len= width_px as usize * height_px as usize;

        FrameBuffer {
            width: width_px,
            height: height_px,
            depth: color_scheme,
            buffer: Vec::with_capacity(frame_buffer_len),
        }
    }
}

fn main() {
    let tft_display = FrameBuffer::new(10, 5, ColorDepth::Rgb888);

    println!("The framebuffer has been created: {}x{} px", tft_display.width, tft_display.height);
}
