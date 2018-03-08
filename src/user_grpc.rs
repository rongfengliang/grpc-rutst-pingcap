// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_USER_LOGIN_LOGIN: ::grpcio::Method<super::user::UserRequest, super::user::UserResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/UserLogin/Login",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct UserLoginClient {
    client: ::grpcio::Client,
}

impl UserLoginClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        UserLoginClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn login_opt(&self, req: &super::user::UserRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::user::UserResponse> {
        self.client.unary_call(&METHOD_USER_LOGIN_LOGIN, req, opt)
    }

    pub fn login(&self, req: &super::user::UserRequest) -> ::grpcio::Result<super::user::UserResponse> {
        self.login_opt(req, ::grpcio::CallOption::default())
    }

    pub fn login_async_opt(&self, req: &super::user::UserRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::user::UserResponse>> {
        self.client.unary_call_async(&METHOD_USER_LOGIN_LOGIN, req, opt)
    }

    pub fn login_async(&self, req: &super::user::UserRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::user::UserResponse>> {
        self.login_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait UserLogin {
    fn login(&self, ctx: ::grpcio::RpcContext, req: super::user::UserRequest, sink: ::grpcio::UnarySink<super::user::UserResponse>);
}

pub fn create_user_login<S: UserLogin + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_USER_LOGIN_LOGIN, move |ctx, req, resp| {
        instance.login(ctx, req, resp)
    });
    builder.build()
}
