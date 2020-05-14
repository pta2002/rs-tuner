use aubio_rs::{Pitch, PitchMode};
use jack::Client as JackClient;
use jack::PortFlags as PF;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};

fn main() {
    // Initialize JACK
    let (client, _) = JackClient::new("tuner", jack::ClientOptions::NO_START_SERVER).unwrap();

    let input_port = client
        .register_port("input", jack::AudioIn)
        .expect("Failed to create JACK port");

    let input_name = input_port.name().unwrap();

    let mut pitch_detector = Arc::new(Mutex::new(
        Pitch::new(
            PitchMode::Yinfft,
            16384,
            client.buffer_size().try_into().unwrap(),
            client.sample_rate().try_into().unwrap(),
        )
        .expect("Failed to initialize pitch detector"),
    ));

    let process_callback = |client: &jack::Client, ps: &jack::ProcessScope| {
        println!("Got {} frames!", ps.n_frames());

        let audio_input = input_port.as_slice(ps);
        let mut detector = pitch_detector.clone().lock().unwrap();

        println!("Freq: {}", detector.do_result(audio_input).unwrap_or(0.0));

        jack::Control::Continue
    };

    let process = jack::ClosureProcessHandler::new(process_callback, pitch_detector.clone());
    let async_client = client.activate_async(Notifications, process).unwrap();
    let client = async_client.as_client();

    let ports = client.ports(None, None, PF::IS_OUTPUT | PF::IS_PHYSICAL);
    let output_port = ports.get(0).unwrap();
    client
        .connect_ports_by_name(&output_port, &input_name)
        .unwrap();

    loop {}
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
