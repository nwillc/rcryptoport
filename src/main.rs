mod prices;
mod model;
mod config;

fn main() {
   match config::get_config() {
       Err(err) => println!("Unable to open config {}", err),
       Ok(config) => {
           println!("{:?}", config);
            let currencies: Vec<String> = config.portfolio.positions.into_iter()
               .map(|position| position.currency)
               .collect();

           println!("{:?}", currencies);
           // match prices::prices(config.app_id, &currencies) {
           //     Err(err) => println!("unable to get prices {}", err),
           //     Ok(prices) => {
           //         println!("{:?}", prices);
           //         for position in config.portfolio.positions.iter() {
           //             prices.iter().find(|position| position.currency.eq())
           //         }
           //     }
           // }
       }
   }
}
