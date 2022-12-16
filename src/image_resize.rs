use image::{GenericImageView, imageops::{self, FilterType}};
use std::path::Path;

pub fn image_resize(file_path: &Path, input_file: &String) {
    //Get the file name without the extension
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();

    //Get the file extension and make it lowercase
    let file_ext = file_path.extension().unwrap().to_str().unwrap().to_lowercase();

    //Get the directory path of the image
    let file_dir = file_path.parent().unwrap();

    //An array of widths for the srcset 
    let widths = [300, 600, 1200];

    //Load the image from a file and read its dimensions
    let img = image::open(&input_file).unwrap();
    let dimensions = img.dimensions();

    //Iterate over the array of widths
    for width in widths.iter() {
        //Determine the name extension based on the width
        let size = match *width {
            300 => "s",
            600 => "m",
            _ => "l"
        };

        //Calculate the new dimensions based on the original aspect ratio
        let new_dimensions = calc_new_dimensions(dimensions, *width);

        //Resize the image and maintain the aspect ratio
        let resized = imageops::resize(&img, new_dimensions.0, new_dimensions.1, FilterType::Triangle);

        //Create an output name for the resized image using the orignal name, file extension, and a custom suffix for the new size
        let output_file_name = format!("{}-{}.{}",file_name ,size, file_ext);

        //Create an output pathe for the resized image using the newly generated name.
        let output_file_path = file_dir.join(output_file_name);
        
        resized
            .save(output_file_path)
            .unwrap();

        println!("Saved {} size image", size);
    }
}

//Function to calculate a new height to retain the original aspect ration when resizing
fn calc_new_dimensions(dimensions: (u32, u32), new_width: u32) -> (u32, u32) {
    let new_height = (dimensions.1 as f32 / dimensions.0 as f32 * new_width as f32) as u32;
    (new_width, new_height)
}