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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct UserRequest {
    // message fields
    pub name: ::std::string::String,
    pub age: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserRequest {}

impl UserRequest {
    pub fn new() -> UserRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserRequest {
        static mut instance: ::protobuf::lazy::Lazy<UserRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserRequest,
        };
        unsafe {
            instance.get(UserRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string age = 2;

    pub fn clear_age(&mut self) {
        self.age.clear();
    }

    // Param is passed by value, moved
    pub fn set_age(&mut self, v: ::std::string::String) {
        self.age = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_age(&mut self) -> &mut ::std::string::String {
        &mut self.age
    }

    // Take field
    pub fn take_age(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.age, ::std::string::String::new())
    }

    pub fn get_age(&self) -> &str {
        &self.age
    }

    fn get_age_for_reflect(&self) -> &::std::string::String {
        &self.age
    }

    fn mut_age_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.age
    }
}

impl ::protobuf::Message for UserRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.age)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.age.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.age);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.age.is_empty() {
            os.write_string(2, &self.age)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UserRequest {
    fn new() -> UserRequest {
        UserRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    UserRequest::get_name_for_reflect,
                    UserRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "age",
                    UserRequest::get_age_for_reflect,
                    UserRequest::mut_age_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserRequest>(
                    "UserRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_age();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserResponse {
    // message fields
    pub code: i32,
    pub message: ::std::string::String,
    pub result: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserResponse {}

impl UserResponse {
    pub fn new() -> UserResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserResponse {
        static mut instance: ::protobuf::lazy::Lazy<UserResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserResponse,
        };
        unsafe {
            instance.get(UserResponse::new)
        }
    }

    // int32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: i32) {
        self.code = v;
    }

    pub fn get_code(&self) -> i32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &i32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut i32 {
        &mut self.code
    }

    // string message = 2;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // string result = 3;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ::std::string::String) {
        self.result = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result(&mut self) -> &mut ::std::string::String {
        &mut self.result
    }

    // Take field
    pub fn take_result(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.result, ::std::string::String::new())
    }

    pub fn get_result(&self) -> &str {
        &self.result
    }

    fn get_result_for_reflect(&self) -> &::std::string::String {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.result
    }
}

impl ::protobuf::Message for UserResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.result)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        if !self.result.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.result);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_int32(1, self.code)?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
        }
        if !self.result.is_empty() {
            os.write_string(3, &self.result)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UserResponse {
    fn new() -> UserResponse {
        UserResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "code",
                    UserResponse::get_code_for_reflect,
                    UserResponse::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    UserResponse::get_message_for_reflect,
                    UserResponse::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "result",
                    UserResponse::get_result_for_reflect,
                    UserResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserResponse>(
                    "UserResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserResponse {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_message();
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10proto/user.proto\"3\n\x0bUserRequest\x12\x12\n\x04name\x18\x01\x20\
    \x01(\tR\x04name\x12\x10\n\x03age\x18\x02\x20\x01(\tR\x03age\"T\n\x0cUse\
    rResponse\x12\x12\n\x04code\x18\x01\x20\x01(\x05R\x04code\x12\x18\n\x07m\
    essage\x18\x02\x20\x01(\tR\x07message\x12\x16\n\x06result\x18\x03\x20\
    \x01(\tR\x06result23\n\tUserLogin\x12&\n\x05Login\x12\x0c.UserRequest\
    \x1a\r.UserResponse\"\0J\xef\x03\n\x06\x12\x04\0\0\x0c\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x10\n\n\n\x02\x06\0\x12\x04\x01\0\x03\x01\n\n\n\x03\x06\
    \0\x01\x12\x03\x01\x08\x11\n\x0b\n\x04\x06\0\x02\0\x12\x03\x02\x042\n\
    \x0c\n\x05\x06\0\x02\0\x01\x12\x03\x02\x08\r\n\x0c\n\x05\x06\0\x02\0\x02\
    \x12\x03\x02\x0e\x19\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x02#/\n\n\n\x02\
    \x04\0\x12\x04\x04\0\x07\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x13\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\x05\x04\x12\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x04\x05\x04\x04\x14\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x05\x04\n\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x05\x0b\x0f\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x05\x10\x11\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x06\x04\x11\n\r\n\
    \x05\x04\0\x02\x01\x04\x12\x04\x06\x04\x05\x12\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03\x06\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x06\x0b\x0e\
    \n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x06\x0f\x10\n\n\n\x02\x04\x01\x12\
    \x04\x08\0\x0c\x01\n\n\n\x03\x04\x01\x01\x12\x03\x08\x08\x14\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03\t\x04\x12\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\t\
    \x04\x08\x15\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\t\x04\t\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\t\n\x0e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\
    \t\x10\x11\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\n\x04\x15\n\r\n\x05\x04\
    \x01\x02\x01\x04\x12\x04\n\x04\t\x12\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\
    \x03\n\x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\n\x0b\x12\n\x0c\n\
    \x05\x04\x01\x02\x01\x03\x12\x03\n\x13\x14\n\x0b\n\x04\x04\x01\x02\x02\
    \x12\x03\x0b\x04\x14\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x0b\x04\n\x15\
    \n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x0b\x04\n\n\x0c\n\x05\x04\x01\
    \x02\x02\x01\x12\x03\x0b\x0b\x11\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\
    \x0b\x12\x13b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
