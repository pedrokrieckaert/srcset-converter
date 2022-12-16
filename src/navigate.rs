use std:: {fs, io};
use std::path::{Path,PathBuf};

pub fn navigate(mut curr_dir: PathBuf) -> io::Result<String> {
    loop {
        println!("Current directory: {}", curr_dir.display());

        let entries = fs::read_dir(&curr_dir)?;
        let mut entries: Vec<_> = entries.map(|res| res.map(|e| e.path())).collect::<Result<_, io::Error>>()?;
        entries.sort();

        //Filter the entries in the CLI to only include image files and folders
        let mut filtered_entries = Vec::new();
        for entry in entries {
            let file_ext = entry.extension().unwrap_or_default().to_str().unwrap_or_default();
            let file_name = entry.file_stem().unwrap().to_str().unwrap();

            //Filter checking for file extension and excluding already moodified images.
            match file_ext {
                "jpg" | "png" | "gif" if !file_name.ends_with("-s") && !file_name.ends_with("-m") && !file_name.ends_with("-l") => filtered_entries.push(entry),
                _ if entry.is_dir() => filtered_entries.push(entry),
                _ => (),
            }
        }

        for (index, entry) in filtered_entries.iter().enumerate() {
            let file_name = entry.file_name().unwrap().to_str().unwrap();
            let file_type = if entry.is_dir() { "folder" } else { "file" };
            println!("[{}] {} ({})", index + 1, file_name, file_type);
        }

        println!("Enter the index of the file or directory you want to navigate to (0 to go up a directory, q to quit, or enter a file path):");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "q" {
            println!("Exiting...");
            //return Ok(String::new());
            std::process::exit(0);
        } else {
            let input_path = Path::new(input);

            if input_path.is_file() {
                println!("Selected image: {}", input_path.display());
                return Ok(input.to_string());
            } else {
                let input: usize = input.parse().unwrap();

                if input == 0 {
                    curr_dir = curr_dir.parent().unwrap().to_path_buf();
                } else {
                    let selected_entry = &filtered_entries[input - 1];
                    if selected_entry.is_dir() {
                        curr_dir = selected_entry.to_path_buf();
                    } else {
                        println!("Selected image: {}", selected_entry.display());
                        return Ok(selected_entry.to_str().unwrap().to_string());
                    }
                }
            }
        }
    }
}