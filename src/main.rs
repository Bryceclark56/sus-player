use std::fs::File;
use std::io::BufReader;
use async_std::task;
use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Initializing Sus Player");

    let mut app = tide::new();
    app.at("/sus/meeting").post(verify_imposter);
    app.listen("127.0.0.1:50010").await?;

    Ok(())
}

async fn verify_imposter(_req: Request<()>) -> tide::Result {
    println!("Are you the imposter?");

    task::spawn(async {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = BufReader::new(File::open("./test").unwrap());

        let source = Decoder::new(file).unwrap();

        if sink.empty() { sink.append(source); sink.sleep_until_end(); }
        
    });

    Ok(format!("That's kinda sus...\n").into())
}