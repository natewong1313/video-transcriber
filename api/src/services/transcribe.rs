use tokio::{
    sync::mpsc::{self, Receiver},
    task,
};

pub struct TranscribeTask {
    pub url: String,
    pub user_id: i64,
}

pub async fn transcribe_worker_loop(mut rx: Receiver<TranscribeTask>) {
    while let Some(transcribe_task) = rx.recv().await {
        println!("Received: {}", transcribe_task.url);
        task::spawn(do_transcribe(transcribe_task));
    }
}

async fn do_transcribe(task: TranscribeTask) {
    let url = format!("https://videos3.nate-wong.com/{}", task.url);
    let samples: Vec<i16> = hound::WavReader::open(url)
        .unwrap()
        .into_samples::<i16>()
        .map(|x| x.unwrap())
        .collect();
}
