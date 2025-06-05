mod whisper;

#[tokio::main]
async fn main() {
    let model_path = whisper::download_model(whisper::ModelType::Tiny)
        .await
        .unwrap();
    println!("{}", model_path);
}
