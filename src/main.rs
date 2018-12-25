#[macro_use] extern crate nickel;
extern crate systemstat;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate serde_derive;

use nickel::{Nickel, MediaType, status::StatusCode};
use systemstat::{System, Platform};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LoadInformation {
    one_minute: f32,
    five_minutes:  f32,
    fifteen_minutes: f32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Error {
    error_code: String,
    error_message: String
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
                        one_minute: loadavg.one,
                        five_minutes: loadavg.five,
                        fifteen_minutes: loadavg.fifteen
                    };

                    serde_json::to_string(&result).unwrap()

                },
                
                
                
                Err(_x) => {

                    _res.set(StatusCode::ServiceUnavailable);   // HTTP Error 503

                    let result = Error {
                        error_code: String::from("E_LOADAVG_UNAVAILABLE"),
                        error_message: String::from("Error while fetching load average from system stats.")
                    };

                    serde_json::to_string(&result).unwrap()

                }

            }

        }
    });

    server.listen("127.0.0.1:6767").expect("Failed to bind port");
}