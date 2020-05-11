use aubio_rs::Pitch;
use jack::Client as JackClient;
use jack::PortFlags as PF;

fn main() {
    // Initialize JACK
    let (client, _) = JackClient::new("tuner", jack::ClientOptions::NO_START_SERVER).unwrap();

    let process = jack::ClosureProcessHandler::new(process_callback);
    let async_client = client.activate_async(Notifications, process).unwrap();
    let client = async_client.as_client();

    let input_port = client
        .register_port("input", jack::AudioIn)
        .expect("Failed to create JACK port")
        .name()
        .unwrap();
    let ports = client.ports(None, None, PF::IS_OUTPUT | PF::IS_PHYSICAL);
    let output_port = ports.get(0).unwrap();
    client
        .connect_ports_by_name(&output_port, &input_port)
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

fn process_callback(client: &jack::Client, ps: &jack::ProcessScope) -> jack::Control {
    println!("Got {} frames!", ps.n_frames());
    jack::Control::Continue
}
