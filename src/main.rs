use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};

mod methods;

fn main() {
    let mut io = IoHandler::default();
    io.add_method("say_hello", methods::say_hello);
    io.add_method("say_bye", methods::say_bye);

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}
