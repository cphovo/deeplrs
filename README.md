# rsdeepl
Access DeepL translations using Rust.

## How to use

1. install
   ```bash
   > cargo install --path .
   ```

2. run command 
   ```bash
   deepl text [source, [target]]
   ```

   - text: Text to be translated
   - source: Current language of the text
   - target: Target language of the text

   The language code can be seen on the homepage of [DeepL](https://www.deepl.com/translator), such as `en` for English, `zh` for Chinese, and so on.

3. example
   ```bash
   > deepl 快乐是怎样消失的呢 zh en
   How does joy disappear?
   ```

   or quickly translate Chinese to English.
   ```bash
   > deepl 快乐是怎样消失的呢
   How does joy disappear?
   ```

## Thanks

This demo reference [@OwO-Network](https://github.com/OwO-Network/PyDeepLX)'s open source project, many thanks!🙏