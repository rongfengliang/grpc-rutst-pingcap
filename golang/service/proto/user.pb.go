// Code generated by protoc-gen-go. DO NOT EDIT.
// source: proto/user.proto

/*
Package service is a generated protocol buffer package.

It is generated from these files:
	proto/user.proto

It has these top-level messages:
	UserRequest
	UserResponse
*/
package service

import proto "github.com/golang/protobuf/proto"
import fmt "fmt"
import math "math"

import (
	context "golang.org/x/net/context"
	grpc "google.golang.org/grpc"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion2 // please upgrade the proto package

type UserRequest struct {
	Name string `protobuf:"bytes,1,opt,name=name" json:"name,omitempty"`
	Age  string `protobuf:"bytes,2,opt,name=age" json:"age,omitempty"`
}

func (m *UserRequest) Reset()                    { *m = UserRequest{} }
func (m *UserRequest) String() string            { return proto.CompactTextString(m) }
func (*UserRequest) ProtoMessage()               {}
func (*UserRequest) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{0} }

func (m *UserRequest) GetName() string {
	if m != nil {
		return m.Name
	}
	return ""
}

func (m *UserRequest) GetAge() string {
	if m != nil {
		return m.Age
	}
	return ""
}

type UserResponse struct {
	Code    int32  `protobuf:"varint,1,opt,name=code" json:"code,omitempty"`
	Message string `protobuf:"bytes,2,opt,name=message" json:"message,omitempty"`
	Result  string `protobuf:"bytes,3,opt,name=result" json:"result,omitempty"`
}

func (m *UserResponse) Reset()                    { *m = UserResponse{} }
func (m *UserResponse) String() string            { return proto.CompactTextString(m) }
func (*UserResponse) ProtoMessage()               {}
func (*UserResponse) Descriptor() ([]byte, []int) { return fileDescriptor0, []int{1} }

func (m *UserResponse) GetCode() int32 {
	if m != nil {
		return m.Code
	}
	return 0
}

func (m *UserResponse) GetMessage() string {
	if m != nil {
		return m.Message
	}
	return ""
}

func (m *UserResponse) GetResult() string {
	if m != nil {
		return m.Result
	}
	return ""
}

func init() {
	proto.RegisterType((*UserRequest)(nil), "UserRequest")
	proto.RegisterType((*UserResponse)(nil), "UserResponse")
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// Client API for UserLogin service

type UserLoginClient interface {
	Login(ctx context.Context, in *UserRequest, opts ...grpc.CallOption) (*UserResponse, error)
}

type userLoginClient struct {
	cc *grpc.ClientConn
}

func NewUserLoginClient(cc *grpc.ClientConn) UserLoginClient {
	return &userLoginClient{cc}
}

func (c *userLoginClient) Login(ctx context.Context, in *UserRequest, opts ...grpc.CallOption) (*UserResponse, error) {
	out := new(UserResponse)
	err := grpc.Invoke(ctx, "/UserLogin/Login", in, out, c.cc, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// Server API for UserLogin service

type UserLoginServer interface {
	Login(context.Context, *UserRequest) (*UserResponse, error)
}

func RegisterUserLoginServer(s *grpc.Server, srv UserLoginServer) {
	s.RegisterService(&_UserLogin_serviceDesc, srv)
}

func _UserLogin_Login_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UserRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(UserLoginServer).Login(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/UserLogin/Login",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(UserLoginServer).Login(ctx, req.(*UserRequest))
	}
	return interceptor(ctx, in, info, handler)
}

var _UserLogin_serviceDesc = grpc.ServiceDesc{
	ServiceName: "UserLogin",
	HandlerType: (*UserLoginServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Login",
			Handler:    _UserLogin_Login_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "proto/user.proto",
}

func init() { proto.RegisterFile("proto/user.proto", fileDescriptor0) }

var fileDescriptor0 = []byte{
	// 177 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x4c, 0x8f, 0xcf, 0x0a, 0x82, 0x40,
	0x10, 0x87, 0x33, 0x53, 0x71, 0x32, 0x90, 0x39, 0xc4, 0xd2, 0x29, 0x3c, 0x44, 0x27, 0x83, 0x7c,
	0x83, 0xce, 0x9d, 0xa4, 0x2e, 0xdd, 0xcc, 0x06, 0x11, 0xd2, 0xb5, 0x1d, 0xed, 0xf9, 0xc3, 0xd9,
	0x22, 0x6f, 0xdf, 0x6f, 0x86, 0x6f, 0xfe, 0x40, 0xdc, 0x19, 0xdd, 0xeb, 0xc3, 0xc0, 0x64, 0x52,
	0xc1, 0x24, 0x83, 0xe5, 0x95, 0xc9, 0xe4, 0xf4, 0x1a, 0x88, 0x7b, 0x44, 0x58, 0xb4, 0x45, 0x43,
	0xca, 0xd9, 0x3a, 0xfb, 0x30, 0x17, 0xc6, 0x18, 0xdc, 0xa2, 0x22, 0x35, 0x97, 0xd2, 0x88, 0xc9,
	0x05, 0x22, 0x2b, 0x71, 0xa7, 0x5b, 0xa6, 0xd1, 0x2a, 0xf5, 0xc3, 0x5a, 0x5e, 0x2e, 0x8c, 0x0a,
	0x82, 0x86, 0x98, 0xff, 0xe6, 0x2f, 0xe2, 0x1a, 0x7c, 0x43, 0x3c, 0x3c, 0x7b, 0xe5, 0x4a, 0xe3,
	0x9b, 0x8e, 0x19, 0x84, 0xe3, 0xd4, 0xb3, 0xae, 0xea, 0x16, 0x77, 0xe0, 0x59, 0x88, 0xd2, 0xc9,
	0x7d, 0x9b, 0x55, 0x3a, 0x5d, 0x9c, 0xcc, 0x4e, 0xe1, 0x2d, 0x60, 0x32, 0xef, 0xba, 0xa4, 0xbb,
	0x2f, 0x1f, 0x65, 0x9f, 0x00, 0x00, 0x00, 0xff, 0xff, 0xed, 0x00, 0xb2, 0xc5, 0xe5, 0x00, 0x00,
	0x00,
}
