use stb_image::image::{Image, LoadResult, load_with_depth};
use std::env;
use std::fs;
use std::path::Path;

pub struct RtwImage {
    bytes_per_pixel: usize,
    fdata: Option<Vec<f32>>, // Linear floating-point pixel data
    bdata: Option<Vec<u8>>,  // Linear 8-bit pixel data
    image_width: usize,
    image_height: usize,
    bytes_per_scanline: usize,
}

impl RtwImage {
    pub fn default() -> Self {
        RtwImage {
            bytes_per_pixel: 3,
            fdata: None,
            bdata: None,
            image_width: 0,
            image_height: 0,
            bytes_per_scanline: 0,
        }
    }
    pub fn new(image_filename: &str) -> Self {
        let mut rtw_image = RtwImage {
            bytes_per_pixel: 3,
            fdata: None,
            bdata: None,
            image_width: 0,
            image_height: 0,
            bytes_per_scanline: 0,
        };

        let filename = image_filename.to_string();
        let imagedir = env::var("RTW_IMAGES").ok();

        // Hunt for the image file in some likely locations
        if let Some(dir) = imagedir {
            if rtw_image.load(&format!("{}/{}", dir, image_filename)) {
                return rtw_image;
            }
        }

        let search_paths = [
            "",
            "images",
            "../images",
            "../../images",
            "../../../images",
            "../../../../images",
            "../../../../../images",
            "../../../../../../images",
        ];

        for path in search_paths {
            let full_path = format!("{}/{}", path, filename);
            println!("Searching: {}", full_path);
            if rtw_image.load(&full_path) {
                return rtw_image;
            }
        }

        eprintln!("ERROR: Could not load image file '{}'.", image_filename);
        rtw_image
    }

    pub fn load(&mut self, filename: &str) -> bool {
        let res = load_with_depth(filename, self.bytes_per_pixel, true);
        match res {
            LoadResult::ImageF32(image) => {
                self.fdata = Some(image.data);
                self.image_width = image.width;
                self.image_height = image.height;
                self.bytes_per_scanline = self.image_width * self.bytes_per_pixel;
                self.convert_to_bytes();
                println!("Image loaded.");
                true
            }
            LoadResult::ImageU8(image) => {
                self.bdata = Some(image.data);
                self.image_width = image.width;
                self.image_height = image.height;
                self.bytes_per_scanline = self.image_width * self.bytes_per_pixel;
                println!("Image loaded.");
                true
            }
            _ => false,
        }
    }

    pub fn width(&self) -> usize {
        self.image_width
    }

    pub fn height(&self) -> usize {
        self.image_height
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> &[u8] {
        static MAGENTA: [u8; 3] = [255, 0, 255];
        if self.bdata.is_none() {
            return &MAGENTA;
        }

        let x = Self::clamp(x, 0, self.image_width);
        let y = Self::clamp(y, 0, self.image_height);

        let offset = y * self.bytes_per_scanline + x * self.bytes_per_pixel;
        &self.bdata.as_ref().unwrap()[offset..offset + 3]
    }

    fn convert_to_bytes(&mut self) {
        if let Some(fdata) = &self.fdata {
            let total_bytes = self.image_width * self.image_height * self.bytes_per_pixel;
            let mut bdata = Vec::with_capacity(total_bytes);

            for &value in fdata.iter() {
                bdata.push(Self::float_to_byte(value));
            }

            self.bdata = Some(bdata);
        }
    }

    fn clamp(value: usize, min: usize, max: usize) -> usize {
        if value < min {
            min
        } else if value < max {
            value
        } else {
            max - 1
        }
    }

    fn float_to_byte(value: f32) -> u8 {
        if value <= 0.0 {
            0
        } else if value >= 1.0 {
            255
        } else {
            (value * 256.0) as u8
        }
    }
}
