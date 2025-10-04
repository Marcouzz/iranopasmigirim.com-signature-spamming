use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use uuid::Uuid;
use std::io::{self, Write};
use rand::seq::SliceRandom;

fn gen_chrome_version() -> String {
    let mut rng = rand::thread_rng();
    let major: u32 = rng.gen_range(100..=116);
    let build: u32 = rng.gen_range(4800..=5100);
    format!("{}.0.{}.0", major, build)
}

fn gen_user_agent() -> (String, &'static str) {
    let chrome = gen_chrome_version();
    let choices = vec![
        ("Windows NT 10.0; Win64; x64", "Windows"),
        ("Macintosh; Intel Mac OS X 10_15_7", "macOS"),
        ("X11; Linux x86_64", "Linux"),
    ];
    let pick = choices[rand::thread_rng().gen_range(0..choices.len())];
    (
        format!(
            "Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",
            pick.0, chrome
        ),
        pick.1,
    )
}
fn gen_sec_ch_ua(chrome_version: &str) -> String {
    let major: &str = chrome_version.split('.').next().unwrap_or("100");
    let mut labels = vec![
        format!("\"Google Chrome\";v=\"{}\"", major),
        "\"Not?A_Brand\";v=\"8\"".to_string(),
        format!("\"Chromium\";v=\"{}\"", major),
    ];
    labels.shuffle(&mut rand::thread_rng());
    labels.join(", ")
}

fn gen_ga_cookie() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let client_id = format!("{}.{now}", rand::thread_rng().gen_range(10000000..100000000));
    format!("GA1.1.{client_id}")
}

fn gen_ga_dynamic() -> (String, String) {
    let suffix = Uuid::new_v4()
        .to_string()
        .replace("-", "")
        .to_uppercase()
        .chars()
        .take(12)
        .collect::<String>();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let value = format!("GS2.1.s{now}$o1$g1$t{now}$j{}$l0$h0", rand::thread_rng().gen_range(1..=100));
    (suffix, value)
}

#[tokio::main]
async fn main() {
    print!("Enter number of requests: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().unwrap_or(1);

    let url = "https://iranopasmigirim.com/fa";

    for i in 0..count {
        let (ua_string, ua_platform) = gen_user_agent();
        let chrome_version = ua_string.split("Chrome/").nth(1).unwrap_or("100.0.0.0");
        let sec_ch_ua = gen_sec_ch_ua(chrome_version);
        let sec_ch_ua_mobile = "?0";
        let sec_ch_ua_platform = format!("\"{}\"", ua_platform);

        let mut headers = HeaderMap::new();
        headers.insert("accept", HeaderValue::from_static("text/x-component"));
        headers.insert("accept-language", HeaderValue::from_static("en-IR,en;q=0.9,fa-IR;q=0.8,fa;q=0.7,az-IR;q=0.6,az;q=0.5,en-GB;q=0.4,en-US;q=0.3"));
        headers.insert("cache-control", HeaderValue::from_static("no-cache"));
        headers.insert("content-type", HeaderValue::from_static("text/plain;charset=UTF-8"));
        headers.insert("next-action", HeaderValue::from_str(&(Uuid::new_v4().to_string() + &Uuid::new_v4().to_string()[..8])).unwrap());
        headers.insert("next-router-state-tree", HeaderValue::from_static("%5B%22%22%2C%7B%22children%22%3A%5B%5B%22lang%22%2C%22fa%22%2C%22d%22%5D%2C%7B%22children%22%3A%5B%22(used)%22%2C%7B%22children%22%3A%5B%22(main)%22%2C%7B%22children%22%3A%5B%22__PAGE__%22%2C%7B%7D%2C%22%2Ffa%22%2C%22refresh%22%5D%7D%5D%7D%5D%7D%5D%7D%2Cnull%2Cnull%2Ctrue%5D"));
        headers.insert("pragma", HeaderValue::from_static("no-cache"));
        headers.insert("priority", HeaderValue::from_static("u=1, i"));
        headers.insert("sec-ch-ua", HeaderValue::from_str(&sec_ch_ua).unwrap());
        headers.insert("sec-ch-ua-mobile", HeaderValue::from_static(sec_ch_ua_mobile));
        headers.insert("sec-ch-ua-platform", HeaderValue::from_str(&sec_ch_ua_platform).unwrap());
        headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
        headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
        headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
        headers.insert("x-deployment-id", HeaderValue::from_str(&format!("dpl_{}", &Uuid::new_v4().to_string()[..16])).unwrap());
        headers.insert("Referer", HeaderValue::from_static("https://iranopasmigirim.com/fa"));

        let ga = gen_ga_cookie();
        let (ga_suffix, ga_value) = gen_ga_dynamic();
        let now_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let cookie_header = format!("NEXT_LOCALE=fa; {}; _ga_{}={}; munich_sign_count=1; munich_last_sign={}", ga, ga_suffix, ga_value, now_ms);
        headers.insert("cookie", HeaderValue::from_str(&cookie_header).unwrap());

        let body_payload = vec![json!({
            "name": "Chrome Desktop User",
            "signatureType": "typed",
            "signatureData": format!("{} {}", ua_string, Uuid::new_v4().to_string().replace("-", ""))
        })];

        let client = reqwest::Client::new();

        match client.post(url)
            .headers(headers)
            .body(serde_json::to_string(&body_payload).unwrap())
            .timeout(Duration::from_secs(15))
            .send()
            .await
        {
            Ok(resp) => {
                let status = resp.status().as_u16();
                println!("{}/{} -> {} | body (first 1000 chars):\n{}", i+1, count, status);
            }
            
            Err(e) => {
                println!("{}/{} -> error: {}", i+1, count, e);
            }
        }
    }
}














