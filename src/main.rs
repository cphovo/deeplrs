use std::env;

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

    if args.len() == 3 {
        source = &args[2];
    }

    if args.len() == 4 {
        source = &args[2];
        target = &args[3];
    }

    let r = deepl::req(text, source, target).await.unwrap();
    println!("{}", r.result.texts[0].alternatives[0].text);
}
