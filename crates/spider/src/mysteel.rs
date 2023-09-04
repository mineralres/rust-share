use itertools::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub data_date: String,
    pub data_value: f64,
}

pub async fn get_common_data_list(category_code: &str, chart_code: &str) -> Vec<Row> {
    #[derive(Default, Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    struct ReqData {
        category_code: String,
        chart_code: String,
    }

    #[derive(Default, Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    struct Data {
        axis_type: i32,
        data_list: Vec<Row>,
        display: bool,
        index_code: String,
        index_name: String,
        r#type: String,
        unit: String,
    }
    #[derive(Default, Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    struct Resp {
        code: String,
        message: String,
        data: Vec<Data>,
        // success: bool,
        timestamp: i64,
    }
    let client = reqwest::Client::new();
    let url = "https://data.mysteel.com/data/site/commonDataList";
    let req_data = ReqData {
        category_code: category_code.to_string(),
        chart_code: chart_code.to_string(),
    };

    let resp = client.post(url).json(&req_data)
    .header("Accept", "application/json, text/javascript, */*; q=0.01")
    .header("Accept-Encoding", "gzip, deflate, br")
    .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8,zh-TW;q=0.7")
    .header("Host", "data.mysteel.com")
    .header("Referer", "https://data.mysteel.com/")
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")
    .send()
    .await.unwrap()
    .json::<Resp>().await.unwrap();
    //     .text().await.unwrap();

    let v = resp
        .data
        .iter()
        .flat_map(|d| d.data_list.iter())
        .map(|r| r.clone())
        .collect_vec();
    v
}
