use std::{env, io::Cursor, thread, time::Duration};

use rodio::{Decoder, OutputStream, Sink};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ok = include_bytes!("../ui_hacking_passgood.wav");
    let fail = include_bytes!("../ui_hacking_passbad.wav");

    let (_stream, stream_handle) = OutputStream::try_default().expect("wtf");
    let sink = Sink::try_new(&stream_handle)?;

    let src_file: &[u8] = match env::args().nth(1) {
        Some(s) if &s == "0" => ok,
        Some(_) => fail,
        _ => ok,
    };

    let source = Decoder::new_wav(Cursor::new(src_file)).expect("src error");

    sink.append(source);
    sink.sleep_until_end();

    // abrupt sound without it
    thread::sleep(Duration::from_millis(200));

    Ok(())
}
