use std::env;

use whatlang::{detect, Lang};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./deepl text [source, [target]]");
        std::process::exit(1);
    }

    let text = &args[1];
    let mut source = "zh";
    let mut target = "en";
    if args.len() == 2 {
        let lang_info = detect(text).unwrap();
        // 如果检测语言不是普通话，且未指定语言
        // 一律视为英文翻译为中文
        // 包含英文的普通话会被检测为 Tgl，检测的可信度小于 0.2 认为不可信
        if lang_info.lang() != Lang::Cmn
            && lang_info.lang() != Lang::Tgl
            && lang_info.confidence() > 0.2
        {
            source = "en";
            target = "zh";
        }
    }

    if args.len() == 3 {
        source = &args[2];
    }

    if args.len() == 4 {
        source = &args[2];
        target = &args[3];
    }

    match deepl::req(text, source, target).await {
        Ok(r) => println!("{}", r.result.texts[0].alternatives[0].text),
        Err(err) => println!("{}", err),
    }
}
