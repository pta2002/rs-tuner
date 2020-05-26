use aubio_rs::{Pitch, PitchMode};
use jack::PortFlags as PF;
use std::convert::TryInto;
use tuners::note::Note;
use tuners::ui::Gui;

fn main() {
    // Initialize JACK
    let (client, _) = jack::Client::new("tuner", jack::ClientOptions::NO_START_SERVER).unwrap();

    let input_port = client
        .register_port("input", jack::AudioIn)
        .expect("Failed to create JACK port");

    let input_name = input_port.name().unwrap();

    let mut pitch_detector = Pitch::new(
        PitchMode::Yinfft,
        16384,
        client.buffer_size().try_into().unwrap(),
        client.sample_rate().try_into().unwrap(),
    )
    .expect("Failed to initialize pitch detector");

    let (tx, rx) = crossbeam_channel::unbounded();

    let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| {
        let audio_input = input_port.as_slice(ps).to_owned();
        tx.send(audio_input).unwrap();

        // let mut detector = pitch_detector.clone().lock().unwrap();

        // println!("Freq: {}", detector.do_result(audio_input).unwrap_or(0.0));

        jack::Control::Continue
    };

    let process = jack::ClosureProcessHandler::new(process_callback);
    let async_client = client.activate_async(Notifications, process).unwrap();
    let client = async_client.as_client();

    let ports = client.ports(None, None, PF::IS_OUTPUT | PF::IS_PHYSICAL);
    let output_port = ports.get(0).unwrap();
    client
        .connect_ports_by_name(&output_port, &input_name)
        .unwrap();

    let mut gui = Gui::new();

    ctrlc::set_handler(move || {
        // Ui::enable_cursor();
        std::process::exit(0);
    })
    .expect("Failed to set Ctrl+C handler");

    // I should probably run the UI on a separate thread!
    // I mean it's already running but yk, update every 60 seconds or so

    while let Ok(audio_input) = rx.recv() {
        let freq = pitch_detector.do_result(audio_input).unwrap_or(0.0);

        //ui.show(Note::new(440.0, freq));
    }
}

struct Notifications;

impl jack::NotificationHandler for Notifications {
    fn shutdown(&mut self, status: jack::ClientStatus, reason: &str) {
        panic!(
            "JACK:s shutdown with status {:?} because \"{}\"",
            status, reason
        );
    }
}
