extern crate futures;
extern crate grpcio;
extern crate grpc_rs;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::Future;
use futures::sync::oneshot;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use grpc_rs::user::{UserRequest, UserResponse};
use grpc_rs::user_grpc::{self, UserLogin};

#[derive(Clone)]
struct UserServiceImpl;
impl   UserLogin for UserServiceImpl {
       fn login(&self, ctx: ::grpcio::RpcContext, req: UserRequest, sink: ::grpcio::UnarySink<UserResponse>){
            let msg = format!("Hello from dalongrong {}", req.get_name());
        let mut resp = UserResponse::new();
        resp.set_message(msg);
        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
       }
}
fn main(){
    let env = Arc::new(Environment::new(1));
    let service = user_grpc::create_user_login(UserServiceImpl);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
