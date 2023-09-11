extern crate reqwest;

use reqwest::blocking::get;
use serde_json::Value;


pub fn get_urlx_urly(alt: f64, lat: f64) -> Vec<f64> {
    let link_url = format!("https://map.kakao.com/link/map/{},{}", alt, lat);
    let res = get(link_url).unwrap();
    if res.status().is_success() {
        let body = res.text().unwrap();
        let urlx = body.split("\"urlX\":\"").collect::<Vec<_>>()[1].split("\"").collect::<Vec<_>>()[0];
        let urly = body.split("\"urlY\":\"").collect::<Vec<_>>()[1].split("\"").collect::<Vec<_>>()[0];
        return vec![urlx.parse::<f64>().unwrap(), urly.parse::<f64>().unwrap()];
    } else {
        println!("failed");
        return vec![0.0, 0.0];
    }
}


pub fn get_path(start: Vec<f64>, end: Vec<f64>) -> Vec<Vec<f64>> {
    let link_url = format!("https://map.kakao.com/route/bikeset.json?sX={}&sY={}&eX={}&eY={}", start[0], start[1], end[0], end[1]);
    let res = get(link_url).unwrap();
    if res.status().is_success() {
        let body: Value = serde_json::from_str(&res.text().unwrap()).unwrap();
        let elements = body.get("directions").and_then(
            |value| value.get(1)
        ).and_then(|value| value.get("gradientChart")).and_then(|value| value.get("points")).unwrap().as_str().unwrap().split("|").collect::<Vec<_>>(); 
        let mut x_li = vec![];
        let mut y_li = vec![];
        for idx in 0..elements.len() {
            let ee = elements[idx].split(",").collect::<Vec<_>>();
            x_li.push(ee[0].parse::<f64>().unwrap());
            y_li.push(ee[1].parse::<f64>().unwrap());
        }
        return vec![x_li, y_li];
    } else {
        println!("failed");
        return vec![vec![0.0, 0.0]];
    }
}