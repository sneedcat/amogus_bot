use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Currencies {
    pub rates: Rates,
}

#[derive(Debug, Deserialize)]
pub struct Rates {
    #[serde(rename = "USD")]
    pub usd: f64,
    #[serde(rename = "EUR")]
    pub eur: f64,
    #[serde(rename = "JPY")]
    pub jpy: f64,
    #[serde(rename = "GBP")]
    pub gbp: f64,
    #[serde(rename = "AUD")]
    pub aud: f64,
    #[serde(rename = "CAD")]
    pub cad: f64,
    #[serde(rename = "CHF")]
    pub chf: f64,
    #[serde(rename = "RON")]
    pub ron: f64,
    #[serde(rename = "BTC")]
    pub btc: f64,
    #[serde(rename = "BRL")]
    pub brl: f64,
    #[serde(rename = "IDR")]
    pub idr: f64,
}

use std::error::Error;
const URL: &str = "https://api.exchangerate.host/latest";
use crate::{escape::escape, statics::CLIENT};
pub async fn currency(from: String, amount: f64) -> Result<String, Box<dyn Error + Send + Sync>> {
    let url = format!("{}?amount={}&base={}", URL, amount, from);
    let data: Currencies = CLIENT.get(url).send().await?.json().await?;
    let rates = data.rates;
    let caption = format!("*{} {}* to:\n\\- {} USD 🇺🇸\n\\- {} EUR 🇪🇺\n\\- {} JPY 🇯🇵\n\\- {} GBP 🇬🇧\n\\- {} AUD 🇦🇺\n\\- {} CAD 🇨🇦\n\\- {} CHF 🇨🇭\n\\- {} RON 🇷🇴\n\\- {} BTC ₿\n\\- {} BRL 🇧🇷\n\\- {} IDR 🇮🇷",
        escape(&amount.to_string()), from,
        escape(&rates.usd.to_string()),
        escape(&rates.eur.to_string()),
        escape(&rates.jpy.to_string()),
        escape(&rates.gbp.to_string()),
        escape(&rates.aud.to_string()),
        escape(&rates.cad.to_string()),
        escape(&rates.chf.to_string()),
        escape(&rates.ron.to_string()),
        escape(&rates.btc.to_string()),
        escape(&rates.brl.to_string()),
        escape(&rates.idr.to_string()),
    );
    println!("{}", caption);
    Ok(caption)
}
