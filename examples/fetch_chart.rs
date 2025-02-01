use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use log::info;
use log::LevelFilter;
use std::io::Write;

use yahoo_finance_api::Interval;
use yahoo_finance_api::YahooBuilder;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// ticket yahoo
    #[clap(short, long, value_parser)]
    ticker: String,

    // period yyyymmdd-yyyymmdd
    #[clap(short, long, value_parser = parse_period)]
    period: Option<(chrono::NaiveDate, chrono::NaiveDate)>,

    // interval
    #[clap(short, long, value_parser = parse_interval)]
    interval: Option<Interval>,
}

fn parse_period(arg: &str) -> Result<(chrono::NaiveDate, chrono::NaiveDate), clap::Error> {
    let period: Vec<&str> = arg.split('-').collect();
    if period.len() != 2 {
        Err(clap::Error::new(clap::error::ErrorKind::ValueValidation))
    } else {
        let begin = chrono::NaiveDate::parse_from_str(period[0], "%Y%m%d")
            .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;
        let end = chrono::NaiveDate::parse_from_str(period[1], "%Y%m%d")
            .map_err(|_| clap::Error::new(clap::error::ErrorKind::ValueValidation))?;
        Ok((begin, end))
    }
}

fn parse_interval(arg: &str) -> Result<Interval, clap::Error> {
    match arg {
        "1d" => Ok(Interval::Day1),
        "5d" => Ok(Interval::Day5),
        "1mo" => Ok(Interval::Month1),
        "3mo" => Ok(Interval::Month3),
        "6mo" => Ok(Interval::Month6),
        "1y" => Ok(Interval::Year1),
        "2y" => Ok(Interval::Year2),
        "5y" => Ok(Interval::Year5),
        "10y" => Ok(Interval::Year10),
        "ytd" => Ok(Interval::YearToDate),
        "max" => Ok(Interval::Max),
        "empty" => Ok(Interval::Empty),
        _ => Err(clap::Error::new(clap::error::ErrorKind::ValueValidation)),
    }
}

fn main() {
    //
    // cli arg
    let args = Args::parse();

    //
    // logger
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    let mut builder = YahooBuilder::new();
    builder.set_ticker(args.ticker);
    if let Some((begin, end)) = args.period {
        builder.set_period(begin, end);
    }
    if let Some(interval) = args.interval {
        builder.set_interval(interval);
    }
    let result = builder.request_chart().expect("request failed");
    info!("{:?}", result);
}
