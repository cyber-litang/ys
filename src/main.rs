mod config;
mod send;

use std::collections::HashMap;

use anyhow::Result;
use reqwest::ClientBuilder;
use scraper::{Html, Selector};

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36";

#[tokio::main]
async fn main() -> Result<()> {
    let c = config::get_config();
    println!("{:?}", c);
    println!("Hello, world!");
    let value_map = {
        let mut m: HashMap<&str, &str> = HashMap::new();
        m.insert("tbID", &c.ys.user);
        m.insert("tbPwd", &c.ys.password);
        m.insert("ddSave", "0");
        m
    };

    let client = ClientBuilder::new()
        .user_agent(USER_AGENT)
        .cookie_store(true)
        .build()
        .expect("Failed to build client");
    let page = client
        .get("http://10.156.28.90/")
        .send()
        .await?
        .text()
        .await?;
    let page = Html::parse_document(&page);

    let form_selector = Selector::parse("input, select").expect("Failed to parse selector");
    let form_data = {
        let mut data = Vec::new();
        for element in page.select(&form_selector) {
            if let Some(name) = element.attr("name") {
                if let Some(value) = value_map.get(name) {
                    data.push((name, Some(*value)));
                } else {
                    data.push((name, element.attr("value")));
                }
            }
        }
        data
    };

    let login_resp = client
        .post("http://10.156.28.90/")
        .form(&form_data)
        .send()
        .await?;
    let page = login_resp.text().await?;

    let pending = page.contains("在审");
    if !pending {
        send::send_self_mail("不在审", "不在审").await?;
    } else if c.ys.always {
        send::send_self_mail("在审", "在审").await?;
    }

    Ok(())
}
