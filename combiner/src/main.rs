mod arguments;

use arguments::Arguments;
use std::{io::BufReader, fs::File};
use image::{io::Reader, DynamicImage, ImageFormat, imageops::FilterType::Triangle, GenericImageView, ImageError};
use std::convert::TryInto;

// Deriving Trait `Debug` for `ImageFormatError`
#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToFormatImage(String),
    // Using `ImageError` from Crate
    UnableToDecodeImage(ImageError),
    UnableToSaveImage(ImageError),
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    // Function on Struct
    fn new(width: u32, height: u32, name: String) -> Self {
        // Multiply by 4 because each Pixel contains of Red, Green, Blue and Alpha
        let buffer_capacity = height * width * 4;
        // Converting Buffer Capacity from `u32` into `usize`
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }
    // Method on Struct that taking the Struct as first argument
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Arguments::new();
    // Propagate the Return Value with `?` that will unwrap and return the Value
    let (first_image, first_image_format) = find_image_from_path(args.first_image)?;
    let (second_image, second_image_format) = find_image_from_path(args.second_image)?;
    //println!("Value {:?}", args);

    if first_image_format != second_image_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    // Redefine Images with standardized Dimensions
    let (first_image, second_image) = standardize_dimension(first_image, second_image);

    // Using `first_image` because the Dimensions are now identical of both Images
    let mut result_image = FloatingImage::new(first_image.width(), first_image.height(), args.result_image);

    let combined_data = combine_images(first_image, second_image);
    // Propagate the Return Value with `?` that will unwrap and return the Value
    result_image.set_data(combined_data)?;

    // Saving Result Image or destructuring the Error
    if let Err(error) = image::save_buffer_with_format(
        result_image.name,
        &result_image.data,
        result_image.width,
        result_image.height,
        image::ColorType::Rgba8,
        first_image_format,
    ) {
        Err(ImageDataErrors::UnableToSaveImage(error))
    } else {
        Ok(())
    }
}

fn find_image_from_path(path: String) -> Result<(DynamicImage, ImageFormat), ImageDataErrors> {
    //let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    match Reader::open(&path) {
        Ok(image_reader) => {
            //let image_format: ImageFormat = image_reader.format().unwrap();
            if let Some(image_format) = image_reader.format() {
                // let image: DynamicImage = image_reader.decode().unwrap();
                match image_reader.decode() {
                    // Returning Tuple with Image and Image Format
                    Ok(image) => Ok((image, image_format)),
                    Err(error) => Err(ImageDataErrors::UnableToDecodeImage(error))
                }
            } else {
                return Err(ImageDataErrors::UnableToFormatImage(path));
            }
        }
        Err(error) => Err(ImageDataErrors::UnableToReadImageFromPath(error))
    }
}

fn retrieve_smallest_dimension(first_dimension: (u32, u32), second_dimension: (u32, u32)) -> (u32, u32) {
    // Accessing Numbers in Tuple
    let pixels_in_first_dimension = first_dimension.0 * first_dimension.1;
    let pixels_in_second_dimension = second_dimension.0 * second_dimension.1;

    if pixels_in_first_dimension < pixels_in_second_dimension {
        // Implicit Return
        first_dimension
    } else {
        // Implicit Return
        second_dimension
    }
}

fn standardize_dimension(first_image: DynamicImage, second_image: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (smallest_width, smallest_height) = retrieve_smallest_dimension(first_image.dimensions(), second_image.dimensions());
    println!("Smallest Width: {} and Height: {}\n", smallest_width, smallest_height);

    if second_image.dimensions() == (smallest_width, smallest_height) {
        // using FilterType Triangle for Resizing
        // Resize first Image and implicit return it and second Image
        (first_image.resize_exact(smallest_width, smallest_height, Triangle), second_image)
    } else {
        // Resize second Image and implicit return it and first Image
        (first_image, second_image.resize_exact(smallest_width, smallest_height, Triangle))
    }
}

fn combine_images(first_image: DynamicImage, second_image: DynamicImage) -> Vec<u8> {
    let first_image_vec = first_image.to_rgba8().into_vec();
    let second_image_vec = second_image.to_rgba8().into_vec();

    // Implicit Return
    alternate_pixels(first_image_vec, second_image_vec)
}

fn alternate_pixels(first_image: Vec<u8>, second_image: Vec<u8>) -> Vec<u8> {
    // Using `vec` Macro that will create a Vector for a given Length
    let mut combined_data = vec![0u8; first_image.len()];

    // Loop over all Pixels
    let mut i = 0;
    while i < first_image.len() {
        if i % 8 == 0 {
            // Passing the Reference / Pointer instead of the Value / Vector
            combined_data.splice(i..=i + 3, set_rgba(&first_image, i, i + 3));
        } else {
            // Passing the Reference / Pointer instead of the Value / Vector
            combined_data.splice(i..=i + 3, set_rgba(&second_image, i, i + 3));
        }
        // Adding 4 because each Pixel contains of Red, Green, Blue and Alpha
        i += 4;
    }
    // Implicit Return
    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba_vec = Vec::new();
    // Range is inclusive the `end` Value
    for i in start..=end {
        let value: u8 = match vec.get(i) {
            // Dereference the Value of `value_exists`
            Some(value_exists) => *value_exists,
            None => panic!("Index out of Bounds")
        };
        rgba_vec.push(value);
    }
    // Implicit Return
    rgba_vec
}