use structopt::StructOpt;

mod shamrock;

use self::shamrock::Shamrock;

#[derive(StructOpt, Debug)]
enum Cli {
    /// name of the plugin
    #[structopt(name = "new")]
    New {
        plugin: String,

        /// description of the plugin
        #[structopt(
            short = "d",
            long = "desc",
            default_value = "A plugin scaffolded with Shamrock ☘"
        )]
        description: String,

        /// version
        #[structopt(long = "version", default_value = "0.1.0")]
        version: String,

        /// author of the plugin
        #[structopt(long = "author", default_value = "Nathan Knowler")]
        author: String,

        /// author of the plugin
        #[structopt(long = "author_uri", default_value = "https://knowlerkno.ws")]
        author_uri: String,

        /// license type
        #[structopt(long = "license", default_value = "MIT License")]
        license: String,
    },
}

fn main() {
    let input = Cli::from_args();

    match input {
        Cli::New {
            plugin,
            description,
            version,
            author,
            author_uri,
            license,
        } => Shamrock::new(plugin, description, version, author, author_uri, license).make(),
    }
}
