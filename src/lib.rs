use log::debug;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

#[derive(Copy, Clone, Debug)]
pub enum Interval {
    Day1,
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Day1 => write!(f, "1d"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct YahooResult {}

#[derive(Clone, Debug)]
pub enum Error {
    Initialize(String),
    Request(String),
    Json(String),
}

pub struct YahooBuilder {
    ticker: Option<String>,
    period_begin: Option<chrono::NaiveDate>,
    period_end: Option<chrono::NaiveDate>,
    interval: Option<Interval>,
}

impl YahooBuilder {
    pub fn new() -> YahooBuilder {
        Self {
            ticker: None,
            period_begin: None,
            period_end: None,
            interval: None,
        }
    }

    pub fn set_ticker<T: Into<String>>(&mut self, ticker: T) -> &mut Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn set_period<T1: Into<chrono::NaiveDate>, T2: Into<chrono::NaiveDate>>(
        &mut self,
        begin: T1,
        end: T2,
    ) -> &mut Self {
        self.period_begin = Some(begin.into());
        self.period_end = Some(end.into());
        self
    }

    pub fn set_interval<T: Into<Interval>>(&mut self, interval: T) -> &mut Self {
        self.interval = Some(interval.into());
        self
    }

    pub fn request_chart(&self) -> Result<YahooResult, Error> {
        let url = self.make_request_chart_url()?;
        debug!("request chart on '{url}'");

        let output = self
            .make_http_client()?
            .get(&url)
            .send()
            .map_err(|error| Error::Request(format!("failed to request '{url}' because:{error}")))?
            .text()
            .map_err(|error| {
                Error::Request(format!("failed to read body '{url}' because:{error}"))
            })?;
        debug!("request result: {output}");

        let request: YahooResult = serde_json::from_reader(output.as_bytes()).map_err(|error| {
            Error::Json(format!("failed to parse json because {error} : {output} "))
        })?;
        Ok(request)
    }

    fn make_request_chart_url(&self) -> Result<String, Error> {
        let ticker = self.ticker.as_ref().ok_or(Error::Initialize(
            "ticker argument is mandatoty".to_string(),
        ))?;
        let period1 = self.period_begin.map_or(String::new(), |date| {
            format!(
                "period1={}",
                date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp()
            )
        });
        let period2 = self.period_end.map_or(String::new(), |date| {
            format!(
                "period2={}",
                date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp()
            )
        });
        let interval = self
            .interval
            .as_ref()
            .map_or(String::new(), |interval| interval.to_string());

        let prefix = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{ticker}?{period1}{period2}{interval}"
        );
        Ok(prefix)
    }

    fn make_http_client(&self) -> Result<Client, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Connection", HeaderValue::from_static("keep-alive"));
        headers.insert("Expires", HeaderValue::from_static("-1"));
        headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
        headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.99 Safari/537.36"));

        let client = Client::builder()
            .cookie_store(true)
            .default_headers(headers)
            .build()
            .map_err(|error| Error::Initialize(format!("failed to init reqwest : {error}")))?;

        Ok(client)
    }
}

impl Default for YahooBuilder {
    fn default() -> Self {
        Self::new()
    }
}
