use log::debug;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Deserializer};

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum Interval {
    #[serde(rename = "1d")]
    Day1,
    #[serde(rename = "5d")]
    Day5,
    #[serde(rename = "1mo")]
    Month1,
    #[serde(rename = "3mo")]
    Month3,
    #[serde(rename = "6mo")]
    Month6,
    #[serde(rename = "1y")]
    Year1,
    #[serde(rename = "2y")]
    Year2,
    #[serde(rename = "5y")]
    Year5,
    #[serde(rename = "10y")]
    Year10,
    #[serde(rename = "ytd")]
    YearToDate,
    #[serde(rename = "max")]
    Max,
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Day1 => write!(f, "1d"),
            Interval::Day5 => write!(f, "5d"),
            Interval::Month1 => write!(f, "1mo"),
            Interval::Month3 => write!(f, "3mo"),
            Interval::Month6 => write!(f, "6mo"),
            Interval::Year1 => write!(f, "1y"),
            Interval::Year2 => write!(f, "2y"),
            Interval::Year5 => write!(f, "5y"),
            Interval::Year10 => write!(f, "10y"),
            Interval::YearToDate => write!(f, "ytd"),
            Interval::Max => write!(f, "max"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct YahooResult {
    pub chart: YahooChart,
}
#[derive(Debug, Deserialize)]
pub struct YahooChart {
    pub result: Vec<YahooChartResult>,
}

#[derive(Debug, Deserialize)]
pub struct YahooChartResult {
    pub meta: YahooMeta,
    #[serde(deserialize_with = "from_opt_vector_timestamp")]
    pub timestamp: Option<Vec<chrono::NaiveDateTime>>,
    pub indicators: YahooChartIndicators,
}

#[derive(Debug, Deserialize)]
pub struct TradingPeriod {
    #[serde(deserialize_with = "from_timestamp")]
    pub start: chrono::NaiveDateTime,
    #[serde(deserialize_with = "from_timestamp")]
    pub end: chrono::NaiveDateTime,
    pub gmtoffset: i64,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
pub struct CurrentTradingPeriod {
    pub pre: Option<TradingPeriod>,
    pub regular: Option<TradingPeriod>,
    pub post: Option<TradingPeriod>,
}

#[derive(Debug, Deserialize)]
pub struct YahooMeta {
    pub currency: Option<String>,
    pub symbol: Option<String>,
    #[serde(rename = "exchangeName")]
    pub exchange_name: Option<String>,
    #[serde(rename = "fullExchangeName")]
    pub full_exchange_name: Option<String>,
    #[serde(rename = "instrumentType")]
    pub instrument_type: Option<String>,
    #[serde(rename = "firstTradeDate", deserialize_with = "from_opt_timestamp")]
    pub first_trade_date: Option<chrono::NaiveDateTime>,
    #[serde(rename = "regularMarketTime", deserialize_with = "from_opt_timestamp")]
    pub regular_market_time: Option<chrono::NaiveDateTime>,
    #[serde(rename = "hasPrePostMarketData")]
    pub has_pre_post_market_data: Option<bool>,
    pub gmtoffset: Option<i64>,
    pub timezone: Option<String>,
    #[serde(rename = "exchangeTimezoneName")]
    pub exchange_timezone_name: Option<String>,
    #[serde(rename = "regularMarketPrice")]
    pub regular_market_price: Option<f64>,
    #[serde(rename = "fiftyTwoWeekHigh")]
    pub fifty_two_week_high: Option<f64>,
    #[serde(rename = "fiftyTwoWeekLow")]
    pub fifty_two_week_low: Option<f64>,
    #[serde(rename = "regularMarketDayHigh")]
    pub regular_market_day_high: Option<f64>,
    #[serde(rename = "regularMarketDayLow")]
    pub regular_market_day_low: Option<f64>,
    #[serde(rename = "regularMarketVolume")]
    pub regular_market_volume: Option<f64>,
    #[serde(rename = "longName")]
    pub long_name: Option<String>,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    #[serde(rename = "chartPreviousClose")]
    pub chart_previous_close: Option<f64>,
    #[serde(rename = "previousClose")]
    pub previous_close: Option<f64>,
    pub scale: Option<f64>,
    #[serde(rename = "priceHint")]
    pub price_hint: Option<f64>,
    #[serde(rename = "currentTradingPeriod")]
    pub current_trading_period: Option<CurrentTradingPeriod>,
    #[serde(rename = "tradingPeriods")]
    pub trading_periods: Option<Vec<Vec<TradingPeriod>>>,
    #[serde(rename = "dataGranularity")]
    pub data_granularity: Option<String>,
    pub range: Option<Interval>,
    #[serde(rename = "validRanges")]
    pub valid_ranges: Option<Vec<Interval>>,
}

#[derive(Debug, Deserialize)]
pub struct YahooChartIndicators {
    pub quote: Vec<YahooChartQuote>,
}

#[derive(Debug, Deserialize)]
pub struct YahooChartQuote {
    pub low: Option<Vec<Option<f64>>>,
    pub open: Option<Vec<Option<f64>>>,
    pub close: Option<Vec<Option<f64>>>,
    pub high: Option<Vec<Option<f64>>>,
    pub volume: Option<Vec<Option<f64>>>,
}

fn from_timestamp<'de, D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let data: i64 = Deserialize::deserialize(deserializer)?;
    let result = chrono::DateTime::from_timestamp(data, 0)
        .ok_or_else(|| Error::custom(format!("invalid timestamp '{data}'")))?
        .naive_local();
    Ok(result)
}

fn from_opt_timestamp<'de, D>(deserializer: D) -> Result<Option<chrono::NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let data: Option<i64> = Deserialize::deserialize(deserializer)?;
    if let Some(data) = data {
        let result = chrono::DateTime::from_timestamp(data, 0)
            .ok_or_else(|| Error::custom(format!("invalid timestamp '{data}'")))?
            .naive_local();
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

fn from_opt_vector_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<chrono::NaiveDateTime>>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let data: Option<Vec<i64>> = Deserialize::deserialize(deserializer)?;
    if let Some(data) = data {
        let mut result = Vec::with_capacity(data.len());
        for value in data.into_iter() {
            let timestamp = chrono::DateTime::from_timestamp(value, 0)
                .ok_or_else(|| Error::custom(format!("invalid timestamp '{value}'")))?
                .naive_local();
            result.push(timestamp);
        }
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

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
        let ticker = self.ticker.as_ref().ok_or(Error::Initialize(
            "ticker argument is mandatoty".to_string(),
        ))?;
        let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{ticker}");
        let query = self.make_chart_query();

        debug!("request chart on '{url}' with query: {:?}", query);

        let output = self
            .make_http_client()?
            .get(&url)
            .query(&query)
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

    fn make_chart_query(&self) -> Vec<(&str, String)> {
        let mut query = Vec::new();
        if let Some(date) = self.period_begin {
            query.push((
                "period1",
                date.and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp()
                    .to_string(),
            ));
        }
        if let Some(date) = self.period_end {
            query.push((
                "period2",
                date.and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp()
                    .to_string(),
            ));
        }
        if let Some(interval) = self.interval {
            query.push(("interval", interval.to_string()));
        }
        query
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
