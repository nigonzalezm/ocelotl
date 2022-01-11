mod connect;
mod server;

use connect::Connect;
use server::server_param::ServerParam;

fn main() {
    let connect = Connect::connect();
    let server_param_message = connect.receive();
    let server_param = ServerParam::build(server_param_message);
}
