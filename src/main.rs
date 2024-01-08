slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

const RANDOM_INDICES: [usize;10] = [8,4,1,6,2,5,3,7,9,0];

fn main() -> Result<(), slint::PlatformError> {
    let mut current_random_quote_index = 0;
    let quotes = vec![
        String::from("Keep filling even it's overflowing!"),
        String::from("Let Go!"),
        String::from("Enough!"),
        String::from("Break Free, Grow!"),
        String::from("Rage, Destroy, Finish!"),
        String::from("Confident!"),
        String::from("Better tomorrow!"),
        String::from("Kneel!"),
        String::from("Bear It 'Til You Can't!"),
        String::from("Just Begin!")
    ];
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_divide_income( move |income| {
        let ui = ui_handle.unwrap();
        let num: f64 = income.trim().parse().unwrap();
        let tax: f64 = num * TAXPER;
        let owner: f64 = num * OWNERPER;
        let profit: f64 = num * PROFITPER;
        let opex: f64 = num * OPEXPER;
        ui.set_results(format_results(tax, owner, profit, opex).into());
        let value = ui.get_slider_value();
        match quotes.get(RANDOM_INDICES[current_random_quote_index]) {
            Some(quote) => {
                ui.set_quote(quote.into());
            },
            None => (),
        }
        current_random_quote_index += 1;
        if current_random_quote_index >= RANDOM_INDICES.len() {
            current_random_quote_index = 0;
        }
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