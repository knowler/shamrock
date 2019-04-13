use heck::{KebabCase, TitleCase};
use serde_derive::Serialize;
use tera::Tera;

use std::fs::File;
use std::io::Write;

#[derive(Serialize, Debug)]
pub struct Shamrock {
    filename: String,
    plugin_name: String,
    description: String,
    version: String,
    author: String,
    author_uri: String,
    license: String,
    text_domain: String,
}

impl Shamrock {
    pub fn new(
        name: String,
        description: String,
        version: String,
        author: String,
        author_uri: String,
        license: String,
    ) -> Self {
        Self {
            filename: format!("{}.php", name.to_kebab_case()),
            plugin_name: name.to_title_case(),
            description,
            version,
            author,
            author_uri,
            license,
            text_domain: name.to_kebab_case(),
        }
    }

    pub fn make(&self) {
        let tera = match Tera::new("/Users/knowler/.config/shamrock/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };

        let mut file = File::create(&self.filename).expect("Cannot create plugin.");
        let contents = tera.render("shamrock.php", &self);

        file.write_all(&contents.unwrap().as_bytes())
            .expect("Cannot create plugin.");

        println!("Created {} ☘", &self.filename);
    }
}
