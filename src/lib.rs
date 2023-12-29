use std::{io::Read, time::UNIX_EPOCH};

use brotli::Decompressor;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

const DEEPL_URL: &str = "https://www2.deepl.com/jsonrpc";

#[derive(Serialize, Deserialize, Debug)]
pub struct Alternative {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub text: String,
    pub alternatives: Vec<Alternative>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Res {
    pub texts: Vec<Text>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResponse {
    pub result: Res,
}

pub async fn req(
    text: &str,
    source: &str,
    target: &str,
) -> Result<JsonResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "*/*".parse().unwrap());
    headers.insert("x-app-os-name", "iOS".parse().unwrap());
    headers.insert("x-app-os-version", "16.3.0".parse().unwrap());
    headers.insert("Accept-Language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
    headers.insert("x-app-device", "iPhone13,2".parse().unwrap());
    headers.insert(
        "User-Agent",
        "DeepL-iOS/2.9.1 iOS 16.3.0 (iPhone13,2)".parse().unwrap(),
    );
    headers.insert("x-app-build", "510265".parse().unwrap());
    headers.insert("x-app-version", "2.9.1".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());

    let response = client
        .post(DEEPL_URL)
        .headers(headers)
        .body(post_data(text, 3, source, target))
        .send()
        .await?;

    let encoding_header = response.headers().get("Content-Encoding").cloned();
    let bytes = response.bytes().await?;

    let data = match encoding_header {
        Some(br) if br == "br" => {
            // parsing brotli-formatted data
            decompress_brotli(&bytes)?
        }
        _ => String::from_utf8(bytes.to_vec()).expect("Found invalid UTF-8"),
    };

    let json_obj = serde_json::from_str(&data).expect("JSON was not well-formatted");

    Ok(json_obj)
}

fn random_number() -> i64 {
    let seed = UNIX_EPOCH.elapsed().unwrap().as_secs();
    let mut r: StdRng = SeedableRng::seed_from_u64(seed);
    let num: i64 = r.gen_range(8300000..=8399999);
    num * 1000
}

fn get_timestamp(text: &str) -> u128 {
    // let i_count = text.chars().filter(|&x| x == 'i').count();
    let mut i_count = text.matches('i').count() as u128;
    let timestamp = UNIX_EPOCH.elapsed().unwrap().as_millis();
    if i_count == 0 {
        return timestamp;
    }
    i_count += 1;
    timestamp - timestamp % i_count + i_count
}

fn post_data(text: &str, number_alternative: u8, source: &str, target: &str) -> String {
    let id = random_number();
    let timestamp = get_timestamp(text);
    let post_str = format!(
        r#"{{"jsonrpc":"2.0","method":"LMT_handle_texts","id":{},"params":{{"texts":[{{"text":"{}","requestAlternatives":{}}}],"splitting":"newlines","lang":{{"source_lang_user_selected":"{}","target_lang":"{}"}},"timestamp":{},"commonJobParams":{{"wasSpoken":false,"transcribe_as":""}}}}}}"#,
        id,
        text,
        number_alternative,
        source.to_uppercase(),
        target.to_uppercase(),
        timestamp
    );

    if (id + 5) % 29 == 0 || (id + 3) % 13 == 0 {
        return post_str.replace("\"method\":\"", "\"method\" : \"");
    }
    return post_str.replace("\"method\":\"", "\"method\": \"");
}

fn decompress_brotli(bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut decompressor = Decompressor::new(bytes.as_ref(), 4096);
    let mut decompressed_data = String::new();
    decompressor.read_to_string(&mut decompressed_data)?;
    Ok(decompressed_data)
}
