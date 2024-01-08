slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

const RANDOM_INDICES: [usize;10] = [8,4,1,6,2,5,3,7,9,0];

fn main() -> Result<(), slint::PlatformError> {
    let current_random_quote_index = 0;
    let quotes = vec![
        String::from("Keep filling even it's overflowing!"),
        String::from("Keep filling even it's overflowing!"),
        String::from("Keep filling even it's overflowing!"),
        String::from("Keep filling even it's overflowing!"),
        String::from("Keep filling even it's overflowing!"),
        String::from("Keep filling even it's overflowing!"),
        String::from("Keep filling even it's overflowing!"),
        String::from("Keep filling even it's overflowing!"),
        String::from("Keep filling even it's overflowing!"),
        String::from("Keep filling even it's overflowing!")
    ];
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    match quotes.get(RANDOM_INDICES[current_random_quote_index]) {
        Some(quote) => {
            ui.set_label(quote.into());
        },
        None => println!("There is no quote!"),
    }

    ui.on_divide_income(move |income| {
        let ui = ui_handle.unwrap();
        let num: f64 = income.trim().parse().unwrap();
        let tax: f64 = num * TAXPER;
        let owner: f64 = num * OWNERPER;
        let profit: f64 = num * PROFITPER;
        let opex: f64 = num * OPEXPER;
        ui.set_results(format_results(tax, owner, profit, opex).into())
    });

    ui.run()
}

fn format_results(tax: f64,owner:f64, profit:f64, opex:f64) -> String {
   return format!("\
   Taxes: {:.2} (rent,travel,cloth,etc)\n\
   Owner: {:.2} (for mom)\n\
   Profit: {:.2} (save)\n\
   Opex: {:.2} (daily exp)",tax, owner, profit, opex) 
}