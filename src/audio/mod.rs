//! A multi-server audio subsystem. Supports JACK, and PulseAudio support is planned.

pub struct AudioConnection {
    server: AudioConnectionServer,
}

impl AudioConnection {
    /// Creates a new connection.
    pub fn new(server: AudioConnectionServer) -> Option<AudioConnection> {
        match server {
            JACK => AudioConnection::connect_jack(),
            Pulse => AudioConnection::connect_pulse(),
        }
    }

    fn connect_jack() -> Option<AudioConnection> {
        AudioConnection { server: JACK }
    }

    fn connect_pulse() -> Option<AudioConnection> {
        AudioConnection { server: Pulse }
    }
}

pub enum AudioConnectionServer {
    JACK,
    Pulse,
}

enum AudioClient {
    JACK { client: jack::Client },
    Pulse { client: psimple::Simple },
}
