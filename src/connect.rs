use std::net::UdpSocket;
use std::str;

pub struct Connect {
    socket: UdpSocket,
    port: u16
}

fn receive_from(socket: &UdpSocket) -> (String, u16) {
    let mut buf = [0; 4096];
    let (bytes, addr) = socket.recv_from(&mut buf).unwrap();
    let message = str::from_utf8(&buf[..(bytes - 1)]).unwrap();
    (message.to_string(), addr.port())
}

impl Connect {
    pub fn connect() -> Connect {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.send_to("(init ocelotl (version 15.1))\0".as_bytes(), "127.0.0.1:6000").unwrap();
        let (message, port) = receive_from(&socket);
        println!("{}", message);
        Connect { socket, port }
    }
    pub fn receive(&self) -> String {
        let (message, _) = receive_from(&self.socket);
        message
    }
    pub fn send(self, message: String) {
        self.socket.send_to(format!("{}\0", message).as_bytes(), format!("127.0.0.1:{}", self.port)).unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_connect() {
        let connect = Connect::connect();
        let message = connect.receive();
        println!("{}", message);
        assert_eq!(1, 1);
    }

}