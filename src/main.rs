use std::{path::Path,env};

mod image_resize;

mod navigate;

fn main(){
    //Get current directory
    let curr_dir = env::current_dir().unwrap();

    //User input CLI navigation to desired file
    let input_file = navigate::navigate(curr_dir).unwrap();

    /*
    if input_file.is_empty() {
        println!("Exiting...");
    } else {
        println!("Selected file: {}", input_file);
    }
    */

    //Create a path object from the file name
    let file_path = Path::new(&input_file);
    
    image_resize::image_resize(&file_path, &input_file);
}