use slint::SharedString;

slint::include_modules!();

const TAXPER: f64=0.30;
const OWNERPER: f64=0.55;
const PROFITPER: f64=0.05;
const OPEXPER: f64=0.10;
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_divide_income(move |string:SharedString| {
            let ui = ui_handle.unwrap();
            let income:f64 = string.parse().unwrap();
            let tax = income*TAXPER;
            let owner = income*OWNERPER;
            let profit = income*PROFITPER;
            let opex = income*OPEXPER;
            let result = format!("Tax: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpex: {:.2}", tax, owner, profit, opex);

            ui.set_results(result.into());
    });

    ui.run()
}
