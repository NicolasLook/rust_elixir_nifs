use std::collections::HashMap;

#[rustler::nif]
fn compute_all(
    stock: f64,
    strike: f64,
    rate: f64,
    sigma: f64,
    maturity: f64,
) -> HashMap<String, f64> {
    let prices_and_greeks = black_scholes::compute_all(stock, strike, rate, sigma, maturity);
    HashMap::from([
        ("call_price".to_string(), prices_and_greeks.call_price),
        ("call_delta".to_string(), prices_and_greeks.call_delta),
        ("call_gamma".to_string(), prices_and_greeks.call_gamma),
        ("call_theta".to_string(), prices_and_greeks.call_theta),
        ("call_vega".to_string(), prices_and_greeks.call_vega),
        ("call_rho".to_string(), prices_and_greeks.call_rho),
        ("call_vanna".to_string(), prices_and_greeks.call_vanna),
        ("call_vomma".to_string(), prices_and_greeks.call_vomma),
        ("call_charm".to_string(), prices_and_greeks.call_charm),
        ("put_price".to_string(), prices_and_greeks.put_price),
        ("put_delta".to_string(), prices_and_greeks.put_delta),
        ("put_gamma".to_string(), prices_and_greeks.put_gamma),
        ("put_theta".to_string(), prices_and_greeks.put_theta),
        ("put_vega".to_string(), prices_and_greeks.put_vega),
        ("put_rho".to_string(), prices_and_greeks.put_rho),
        ("put_vanna".to_string(), prices_and_greeks.put_vanna),
        ("put_vomma".to_string(), prices_and_greeks.put_vomma),
        ("put_charm".to_string(), prices_and_greeks.put_charm),
    ])
}

#[rustler::nif]
fn implied_black_volatility(
    option_price: f64,
    forward: f64,
    strike: f64,
    expiry: f64,
    is_call: bool,
) -> f64 {
    implied_vol::implied_black_volatility(option_price, forward, strike, expiry, is_call)
}

#[rustler::nif]
fn implied_normal_volatility(
    option_price: f64,
    forward: f64,
    strike: f64,
    expiry: f64,
    is_call: bool,
) -> f64 {
    implied_vol::implied_normal_volatility(option_price, forward, strike, expiry, is_call)
}
rustler::init!(
    "Elixir.RustElixirNifs.NifFunction",
    [
        compute_all,
        implied_black_volatility,
        implied_normal_volatility
    ]
);
