use std::fs::File;
use std::io::Error;
use std::result::Result;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SoundComposeYamlData {
    version: String,
}

//pub fn read_file(file_path_str: String) -> Result<SoundComposeYamlData, Error> {
pub fn read_file(file_path_str: String) -> Result<File, Error> {
    println!("yaml.rs read_file()");
    println!("{}", file_path_str);

    // TODO: ? 演算子について（Errの場合は即Err内のエラーを返却してくれる）
    //       https://qiita.com/nirasan/items/321e7cc42e0e0f238254#-%E6%BC%94%E7%AE%97%E5%AD%90
    // MEMO: ファイル読み込み
    //       https://doc.rust-lang.org/std/io/index.html
    //       https://doc.rust-lang.org/std/io/trait.Read.html
    let file = File::open(file_path_str)?;
    // Readerから
    // let mut buffer = String::new();
    // f.read_to_string(&mut buffer)?;

//    let data: SoundComposeYamlData = serde_yaml::from_reader(file)?;

    return Ok(file);
}

pub fn deserialize_file(yaml_file: File) -> Result<SoundComposeYamlData, serde_yaml::Error> {
    println!("yaml.rs deserialize_file()");
    println!("{:?}", yaml_file);

    /*
     * MEMO: シリアライズ・デシリアライズ
     *       https://qiita.com/fujitayy/items/ed0033f8bb036d23710c
     *       https://github.com/dtolnay/serde-yaml
     *       https://docs.serde.rs/serde_yaml/index.html
     */
    let yaml_data = serde_yaml::from_reader(yaml_file)?;

    return Ok(yaml_data
    );
}