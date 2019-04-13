use structopt::StructOpt;

mod shamrock;

use self::shamrock::Shamrock;

#[derive(StructOpt, Debug)]
enum Cli {
    // Name of the plugin
    #[structopt(name = "new")]
    New { plugin: String },
}

fn main() {
    let input = Cli::from_args();

    match input {
        Cli::New { plugin } => Shamrock::new(plugin).make(),
    }
}
