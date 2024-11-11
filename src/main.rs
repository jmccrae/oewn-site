#[macro_use] extern crate rocket;

use clap::Parser;
use rocket::config::Config as RocketConfig;
use rocket::fs::FileServer;
use rocket::response::content::RawHtml;


#[derive(Parser,Debug)]
#[command(name = "English WordNet Interface")]
#[command(version = "1.0")]
#[command(author = "John P. McCrae <john@mccr.ae>")]
struct Config {
    #[arg(short, long, default_value = "8000", help = "The port to start the server on")]
    port: u16,
    #[arg(long, help = "Reload the wordnet from the given folder")]
    wn: Option<String>
}

fn prepare_server(config : &Config) -> Result<(), String> {

    Ok(())
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../dist/index.html"))
}

#[launch]
fn rocket() -> _ {
    let config = Config::parse();
    match prepare_server(&config) {
        Ok(state) => {
            let mut rocket_config = RocketConfig::release_default();
            rocket_config.port = config.port;
            rocket_config.workers = 30;
            rocket::custom(&rocket_config)
                .manage(state)
                .mount("/assets", FileServer::from("dist/assets"))
                .mount("/", routes![index])
        },
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }
}
