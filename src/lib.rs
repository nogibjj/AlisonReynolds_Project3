use polars::prelude::*;

const WARNINGS: &str = "src/data/weather_alerts.csv";
const WEATHER: &str = "src/data/era5land_1980_2022.txt";

pub fn read_data() -> (DataFrame, DataFrame) {
    let warn_df = CsvReader::from_path(WARNINGS).unwrap().finish().unwrap();
    let weather_df = CsvReader::from_path(WEATHER).unwrap().finish().unwrap();
    (warn_df, weather_df)
}
