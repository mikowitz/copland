use crate::regression_data_streamer::*;
use reqwest::Client;

pub async fn load_duration_new() -> Vec<Vec<i32>> {
    load_all_i32_data("https://raw.githubusercontent.com/mikowitz/satie_regression_data/main/data/duration/new.txt").await
}

pub async fn load_duration_add() -> Vec<Vec<i32>> {
    load_all_i32_data("https://raw.githubusercontent.com/mikowitz/satie_regression_data/main/data/duration/add.txt").await
}

pub async fn load_duration_subtract() -> Vec<Vec<i32>> {
    load_all_i32_data("https://raw.githubusercontent.com/mikowitz/satie_regression_data/main/data/duration/subtract.txt").await
}

pub async fn load_duration_multiply() -> Vec<Vec<i32>> {
    load_all_i32_data("https://raw.githubusercontent.com/mikowitz/satie_regression_data/main/data/duration/multiply.txt").await
}

pub async fn load_duration_multiply_by_int() -> Vec<Vec<i32>> {
    load_all_i32_data("https://raw.githubusercontent.com/mikowitz/satie_regression_data/main/data/duration/multiply_by_int.txt").await
}

pub async fn load_duration_divide() -> Vec<Vec<i32>> {
    load_all_i32_data("https://raw.githubusercontent.com/mikowitz/satie_regression_data/main/data/duration/divide.txt").await
}

pub async fn load_duration_divide_by_int() -> Vec<Vec<i32>> {
    load_all_i32_data("https://raw.githubusercontent.com/mikowitz/satie_regression_data/main/data/duration/divide_by_int.txt").await
}

pub async fn load_duration_printable() -> Vec<(i32, i32, bool)> {
    let client = Client::new();

    let data = download_file(&client, "https://raw.githubusercontent.com/mikowitz/satie_regression_data/main/data/duration/printable.txt").await.unwrap();

    let lines = data.lines();

    let res: Vec<_> = lines
        .map(|line| {
            let segments = line.split(":").collect::<Vec<&str>>();

            (
                segments[0].parse::<i32>().unwrap(),
                segments[1].parse::<i32>().unwrap(),
                match segments[2] {
                    "True" => true,
                    _ => false,
                },
            )
        })
        .collect();
    res
}

async fn load_all_i32_data(url: &str) -> Vec<Vec<i32>> {
    let client = Client::new();

    let data = download_file(&client, url).await.unwrap();

    let lines = data.lines();

    let res: Vec<_> = lines
        .map(|line| {
            line.split(":")
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    res
}
