#[macro_use] extern crate nickel;
extern crate systemstat;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate serde_derive;

use nickel::Nickel;
use nickel::MediaType;
use systemstat::{System, Platform};
//extern crate rustc_serialize;

#[derive(Serialize, Deserialize, Debug)]
struct LoadInformation {
    oneMinute: f32,
    fiveMinutes:  f32,
    fifteenMinutes: f32
}

fn main() {

    let mut server = Nickel::new();

    server.utilize(router! {

        get "/load" => |_req, mut _res| {

            _res.set(MediaType::Json);
            
            let sys = System::new();

            match sys.load_average() {

                Ok(loadavg) => {

                    let result = LoadInformation {
                        oneMinute: loadavg.one,
                        fiveMinutes: loadavg.five,
                        fifteenMinutes: loadavg.fifteen
                    };

                    serde_json::to_string(&result).unwrap()

                },
                
                
                
                Err(x) => {

                    //serde_json::to_string(&result).unwrap()
                    String::from("Failure")

                }

            }

        }

        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:6767");
}