extern crate grpcio;
extern crate grpc_rs;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use grpc_rs::user::UserRequest;
use grpc_rs::user_grpc::UserLoginClient;

fn main(){
   let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("rpcserver:50051");
    let client = UserLoginClient::new(ch);
    let mut req = UserRequest::new();
    req.set_name("world".to_owned());
    let reply = client.login(&req).expect("rpc");
    println!("userlogin send: {}", reply.get_message());
}