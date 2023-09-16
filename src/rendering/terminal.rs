use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

/// Terminal represents a simple terminal-like renderer.
pub struct Terminal {
    char_width: usize,
    char_height: usize,
    font: DynamicImage,
    characters: HashMap<char, DynamicImage>,
    grid: Vec<Vec<char>>,
}

impl Terminal {
    /// Create a new Terminal instance.
    pub fn new(font_path: &str, char_width: usize, char_height: usize) -> Self {
        let font = image::open(Path::new(font_path)).expect("Failed to load font image");
        let characters = HashMap::new();
        let grid = vec![vec![' '; 80]; 25]; // Initialize a 80x25 character grid with spaces.

        Terminal {
            char_width,
            char_height,
            font,
            characters,
            grid,
        }
    }

    /// Load characters from the font image and store them in the characters map.
    pub fn load_characters(&mut self) {
        for ascii_code in 32..=127 {
            let character = char::from(ascii_code);
            let char_x = (ascii_code - 32) % 16 * self.char_width as u8;
            let char_y = (ascii_code - 32) / 16 * self.char_height as u8;

            let char_img = self
                .font
                .crop_imm(char_x as u32, char_y as u32, self.char_width as u32, self.char_height as u32);
            self.characters.insert(character, char_img);
        }
    }

    /// Set a character in the grid at the specified position.
    pub fn set_char(&mut self, x: usize, y: usize, character: char) {
        if x < self.grid[0].len() && y < self.grid.len() {
            self.grid[y][x] = character;
        }
    }

    /// Render the grid to an image.
    pub fn render(&self) -> DynamicImage {
        let mut img = ImageBuffer::new(
            self.char_width as u32 * self.grid[0].len() as u32,
            self.char_height as u32 * self.grid.len() as u32,
        );

        for (row, line) in self.grid.iter().enumerate() {
            for (col, character) in line.iter().enumerate() {
                if let Some(char_img) = self.characters.get(character) {
                    for (x, y, pixel) in char_img.pixels() {
                            img.put_pixel(
                                (col as u32 * self.char_width as u32 + x) as u32,
                                (row as u32 * self.char_height as u32 + y) as u32,
                                pixel,
                            );
                    }
                }
            }
        }

        DynamicImage::ImageRgba8(img)
    }

    /// Get an iterator over the raw grid data.
    pub fn get_raw_grid(&self) -> impl Iterator<Item = &Vec<char>> {
        self.grid.iter()
    }

    /// Get an Rc<RefCell<Terminal>> reference to itself.
    pub fn get_rc(self) -> Rc<RefCell<Terminal>> {
        Rc::new(RefCell::new(self))
    }
}
