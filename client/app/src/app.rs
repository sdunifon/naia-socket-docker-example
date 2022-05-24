use std::time::{Duration, Instant};

cfg_if! {
    if #[cfg(feature = "mquad")] {
        use miniquad::info;
    } else {
        use log::info;
    }
}

use naia_client_socket::{ PacketReceiver, PacketSender, ServerAddr, Socket};
use naia_socket_docker_example_shared::{get_shared_config, PING_MSG, PONG_MSG};


pub struct App {
    packet_sender: PacketSender,
    packet_receiver: PacketReceiver,
    message_count: u8,
    timer: Timer,
}

impl App {
    pub fn new() -> App {
        info!("Naia Client Socket Demo started");

        let shared_config = get_shared_config();

        let mut socket = Socket::new(&shared_config);
        socket.connect("http://192.168.0.107:14191");
        // socket.connect("http://127.0.0.1:14191");

        App {
            packet_sender: socket.packet_sender(),
            packet_receiver: socket.packet_receiver(),
            message_count: 0,
            timer: Timer::new(Duration::from_secs(1)),
        }
    }

     pub fn update(&mut self) {
        match self.packet_receiver.receive() {
            Ok(event) => match event {
                Some(packet) => {
                    let message_from_server = String::from_utf8_lossy(packet);

                    // let server_addr = match self.packet_receiver.server_addr() {
                    //     ServerAddr::Found(addr) => addr.to_string(),
                    //     _ => "".to_string(),
                    // };
                    info!("Client recv <- {}: {}", "server_addr", message_from_server);
                    //
                    // if message_from_server.eq(PONG_MSG) {
                    //     self.message_count += 1;
                    // }
                }
                None => {
                    if self.timer.ringing() {
                        self.timer.reset();
                        if self.message_count < 10 {
                            let message_to_server: String = PING_MSG.to_string();

                            let server_addr = match self.packet_receiver.server_addr() {
                                ServerAddr::Found(addr) => addr.to_string(),
                                _ => "".to_string(),
                            };
                            info!("Client send -> {}: {}", server_addr, message_to_server);

                            // self.packet_sender
                            //     .send(Packet::new(message_to_server.into_bytes()));
                            self.packet_sender.send(message_to_server.as_bytes());

                        }
                    }
                }
            },
            Err(err) => {
                info!("Client Error: {}", err);
            }
        }
    }
}

/// A Timer with a given duration after which it will enter into a "Ringing"
/// state. The Timer can be reset at an given time, or manually set to start
/// "Ringing" again.
pub struct Timer {
    duration: Duration,
    last: Instant,
}

impl Timer {
    /// Creates a new Timer with a given Duration
    pub fn new(duration: Duration) -> Self {
        Timer {
            last: Instant::now(),
            duration,
        }
    }

    /// Reset the Timer to stop ringing and wait till 'Duration' has elapsed
    /// again
    pub fn reset(&mut self) {
        self.last = Instant::now();
    }

    /// Gets whether or not the Timer is "Ringing" (i.e. the given Duration has
    /// elapsed since the last "reset")
    pub fn ringing(&self) -> bool {
        Instant::now().saturating_duration_since(self.last) > self.duration
    }

    /// Manually causes the Timer to enter into a "Ringing" state
    pub fn ring_manual(&mut self) {
        self.last -= self.duration;
    }
}
