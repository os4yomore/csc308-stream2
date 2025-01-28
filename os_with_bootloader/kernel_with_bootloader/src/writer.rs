use noto_sans_mono_bitmap::{
  get_raster_width, FontWeight, RasterHeight,
};

/// Constants for the usage of the [`noto_sans_mono_bitmap`] crate.
pub mod font_constants {
  use super::*;

  /// Height of each character raster. The font size is ~0.84% of this.
  /// This defines the line height that enables multiple characters to appear optically in one line naturally.
  pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

  /// The width of each single symbol of the monospaced font.
  pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

  /// Font weight used for rendering characters.
  pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;

  /// Backup character if a desired symbol is not available in the font.
  pub const BACKUP_CHAR: char = '\u{0008}';
}




use core::{
  fmt::{self, Write},
  ptr,
};
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
pub use font_constants::{BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT};
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};

/// Constants for rendering adjustments.
const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;

/// Retrieve the rasterized character. If the character is unavailable, fall back to the backup character.
fn get_char_raster(c: char) -> RasterizedChar {
  fn get(c: char) -> Option<RasterizedChar> {
      get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
  }

  get(c).unwrap_or_else(|| {
      get(BACKUP_CHAR).expect("Should get raster of backup char.")
  })
}

/// FrameBufferWriter manages writing text to the framebuffer.
pub struct FrameBufferWriter {
  framebuffer: &'static mut [u8],
  info: FrameBufferInfo,
  x_pos: usize,
  y_pos: usize,
  pub colour: u8, //wasn't in struct initially
  //max_scroll_lines: usize,
  padding: usize,
}

impl FrameBufferWriter {
  /// Creates a new framebuffer writer.
  pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
      let mut writer = Self {
          framebuffer,
          info,
          x_pos: BORDER_PADDING,
          y_pos: BORDER_PADDING,
          colour: 0x0E,
          //max_scroll_lines: 15,
          padding: 0,
      };
      writer.clear();
      writer
  }

  pub fn set_padding(&mut self, padding: usize) {
    self.padding = padding;
}
  //dynamic cursor positioning with bounds checking
  pub fn set_cursor(&mut self, x: usize, y: usize) -> bool {
    // Check if the position is within bounds
    if x >= self.width() || y >= self.height() {
        return false;
    }

    self.x_pos = x;
    self.y_pos = y;
    true
}

//pub fn get_cursor_position(&self) -> (usize, usize) {
  //  (self.x_pos, self.y_pos)
//}

  //advanced scrolling mechanism

 /*  fn scroll_screen(&mut self, lines: usize) {
    let line_height = CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
    let scroll_distance = line_height * lines;

    // Ensure we don't scroll more than the screen height
    let max_scroll = self.height().saturating_sub(line_height);
    let scroll_distance = scroll_distance.min(max_scroll);

    unsafe {
        // Shift buffer content up
        ptr::copy(
            self.framebuffer.as_ptr().add(scroll_distance * self.info.stride),
            self.framebuffer.as_mut_ptr(),
            self.framebuffer.len().saturating_sub(scroll_distance * self.info.stride)
        );

        // Clear the newly exposed lines at the bottom
        let start_clear = self.height().saturating_sub(scroll_distance);
        for y in start_clear..self.height() {
            for x in 0..self.width() {
                self.write_pixel(x, y, 0);
            }
        }
    }

    // After scrolling, reset cursor to the bottom with current padding
    self.y_pos = self.height().saturating_sub(line_height);
    self.x_pos = self.padding;  // Respect current padding setting
}

  /// Move to the next line, updating the y-position and resetting the x-position.
  fn newline(&mut self) {
      self.y_pos += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
      self.carriage_return();
  }

  /// Reset the x-position to the starting point (left side).
  fn carriage_return(&mut self) {
      self.x_pos = BORDER_PADDING;
  }
*/
  /// Clear the framebuffer and reset cursor positions.
  pub fn clear(&mut self) {
      self.x_pos = BORDER_PADDING;
      self.y_pos = BORDER_PADDING;
      self.framebuffer.fill(0);
  }

  /// Get the width of the framebuffer.
  fn width(&self) -> usize {
      self.info.width
  }

  /// Get the height of the framebuffer.
  fn height(&self) -> usize {
      self.info.height
  }

  /// Write a character to the framebuffer.
  pub fn write_char(&mut self, c: char) {
    match c {
        '\n' => {
            self.y_pos = self.y_pos.saturating_add(CHAR_RASTER_HEIGHT.val() + LINE_SPACING);
            self.x_pos = self.padding;  // Reset to current padding value

            if self.y_pos >= self.height() {
                //self.scroll_screen(1);
            }
        },
        '\r' => {
            self.x_pos = self.padding;  // Reset to current padding value
        },
        c => {
            let char_width = font_constants::CHAR_RASTER_WIDTH;

            // Handle horizontal overflow with current padding
            if self.x_pos + char_width >= self.width() {
                self.y_pos = self.y_pos.saturating_add(CHAR_RASTER_HEIGHT.val() + LINE_SPACING);
                self.x_pos = self.padding;  // Reset to current padding value
            }

            // Handle vertical overflow
            if self.y_pos >= self.height() {
                //self.scroll_screen(1);
            }

            let raster = get_char_raster(c);
            self.write_rendered_char(raster);
        }
    }
}


  /// Render a character's raster data onto the framebuffer.
  pub fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
      for (y, row) in rendered_char.raster().iter().enumerate() {
          for (x, byte) in row.iter().enumerate() {
              self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
          }
      }
      self.x_pos += rendered_char.width() + LETTER_SPACING;
  }

  /// Write a single pixel to the framebuffer.
  fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {

    if x >= self.width() || y >= self.height() {
        return;}
      let pixel_offset = y * self.info.stride + x;

      let colour = match self.info.pixel_format {
        PixelFormat::Rgb => match self.colour {
            0x0E => [intensity, intensity, 0, 0],  // Yellow
            0x26 => [intensity, 0, intensity, 0],  // Pink
            _ => [intensity, intensity, intensity, 0]  // White default
        },
        PixelFormat::Bgr => match self.colour {
            0x0E => [0, intensity, intensity, 0],  // Yellow
            0x26 => [intensity, 0, intensity, 0],  // Pink
            _ => [intensity, intensity, intensity, 0]  // White default
        },
        PixelFormat::U8 => [intensity, 0, 0, 0],
        _ => panic!("Unsupported pixel format")
    };
    
      let bytes_per_pixel = self.info.bytes_per_pixel;
      let byte_offset = pixel_offset * bytes_per_pixel;
      self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
          .copy_from_slice(&colour[..bytes_per_pixel]);

      // Prevent compiler optimizations.
      unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
  }

}
unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl Write for FrameBufferWriter {
  /// Write a string to the framebuffer by rendering each character.
  fn write_str(&mut self, s: &str) -> fmt::Result {
      for c in s.chars() {
          self.write_char(c);
      }
      Ok(())
  }
}

#[macro_export]
macro_rules! write_styled {
    ($writer:expr, $text:expr) => {{
        let mut chars = $text.chars();
        let mut current_colour = 0x0E;
        $writer.colour = current_colour;
        
        while let Some(c) = chars.next() {
            match c {
                '\n' => {
                    $writer.write_char('\n');
                }
                '\t' => {
                    for _ in 0..4 {
                        $writer.write_char(' ');
                    }
                }
                '\\' => {
                    match chars.next() {
                        Some('c') => {
                            current_colour = 0x26;
                            $writer.colour = current_colour;
                        }
                        Some('r') => {
                            current_colour = 0x0E;
                            $writer.colour = current_colour;
                        }
                        Some(unknown) => {
                            $writer.write_char(unknown);
                        }
                        None => break,
                    }
                }
                _ => {
                    $writer.colour = current_colour;
                    $writer.write_char(c);
                }
            }
        }
    }};
}


 /*Error handling:


Bounds checking in write_pixel
Cursor position validation in set_cursor


Overflow handling:


Text wraps when exceeding width
Screen scrolls when exceeding height
Implemented in write_char and scroll_screen


Escape sequences support:


\n: Newline
\t: Tab (4 spaces)
\c: Change to pink
\r: Reset to yellow */