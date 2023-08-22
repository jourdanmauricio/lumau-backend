use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub database: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");

        Config {
            port: env::var("PORT")
                .expect("No port defined")
                .parse()
                .expect("Bad Port definition"),
            host: env::var("HOST").expect("No host defined"),
            database: env::var("DATABASE_URL").expect("No Database defined"),
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}

// pub fn read_config() -> Config {
//     dotenv().ok();
//     let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//     let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
//     let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");

//     Config {
//         port: env::var("PORT")
//             .expect("No port defined")
//             .parse()
//             .expect("Bad Port definition"),
//         host: env::var("HOST").expect("No host defined"),
//         database: env::var("DATABASE_URL").expect("No Database defined"),
//         jwt_secret,
//         jwt_expires_in,
//         jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
//     }
// }
