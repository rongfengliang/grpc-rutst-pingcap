// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('grpc');
var proto_user_pb = require('../proto/user_pb.js');

function serialize_UserRequest(arg) {
  if (!(arg instanceof proto_user_pb.UserRequest)) {
    throw new Error('Expected argument of type UserRequest');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_UserRequest(buffer_arg) {
  return proto_user_pb.UserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_UserResponse(arg) {
  if (!(arg instanceof proto_user_pb.UserResponse)) {
    throw new Error('Expected argument of type UserResponse');
  }
  return new Buffer(arg.serializeBinary());
}

function deserialize_UserResponse(buffer_arg) {
  return proto_user_pb.UserResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var UserLoginService = exports.UserLoginService = {
  login: {
    path: '/UserLogin/Login',
    requestStream: false,
    responseStream: false,
    requestType: proto_user_pb.UserRequest,
    responseType: proto_user_pb.UserResponse,
    requestSerialize: serialize_UserRequest,
    requestDeserialize: deserialize_UserRequest,
    responseSerialize: serialize_UserResponse,
    responseDeserialize: deserialize_UserResponse,
  },
};

exports.UserLoginClient = grpc.makeGenericClientConstructor(UserLoginService);
