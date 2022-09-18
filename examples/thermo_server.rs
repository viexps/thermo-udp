use std::{
    io,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct SmartThermometer {
    temp: Arc<Mutex<Option<f32>>>,
}

impl SmartThermometer {
    fn new() -> Self {
        Self {
            temp: Arc::new(Mutex::new(None)),
        }
    }

    fn set_temp(&mut self, temp: f32) {
        let mut t = self.temp.lock().unwrap();
        *t = Some(temp)
    }
}

fn main() -> Result<(), io::Error> {
    let bind_addr = "127.0.0.1:34255";
    let socket = UdpSocket::bind(bind_addr)?;
    let mut thermo = SmartThermometer::new();

    let temp = Arc::clone(&thermo.temp);

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        let t = temp.lock().unwrap();
        let temp_str: String = t.map_or_else(|| "undefined".to_string(), |x| format!("{} C.", x));
        println!("current temp: {}", temp_str);
    });

    let mut buf = [0u8; 4];
    while let Ok(_) = socket.recv(&mut buf) {
        let temp = f32::from_be_bytes(buf);
        thermo.set_temp(temp);
    }

    Ok(())
}
