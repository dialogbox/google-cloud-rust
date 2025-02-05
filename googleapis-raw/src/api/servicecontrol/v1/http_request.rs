// This file is generated by rust-protobuf 2.25.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/api/servicecontrol/v1/http_request.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct HttpRequest {
    // message fields
    pub request_method: ::std::string::String,
    pub request_url: ::std::string::String,
    pub request_size: i64,
    pub status: i32,
    pub response_size: i64,
    pub user_agent: ::std::string::String,
    pub remote_ip: ::std::string::String,
    pub server_ip: ::std::string::String,
    pub referer: ::std::string::String,
    pub latency: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub cache_lookup: bool,
    pub cache_hit: bool,
    pub cache_validated_with_origin_server: bool,
    pub cache_fill_bytes: i64,
    pub protocol: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HttpRequest {
    fn default() -> &'a HttpRequest {
        <HttpRequest as ::protobuf::Message>::default_instance()
    }
}

impl HttpRequest {
    pub fn new() -> HttpRequest {
        ::std::default::Default::default()
    }

    // string request_method = 1;


    pub fn get_request_method(&self) -> &str {
        &self.request_method
    }
    pub fn clear_request_method(&mut self) {
        self.request_method.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_method(&mut self, v: ::std::string::String) {
        self.request_method = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_method(&mut self) -> &mut ::std::string::String {
        &mut self.request_method
    }

    // Take field
    pub fn take_request_method(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.request_method, ::std::string::String::new())
    }

    // string request_url = 2;


    pub fn get_request_url(&self) -> &str {
        &self.request_url
    }
    pub fn clear_request_url(&mut self) {
        self.request_url.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_url(&mut self, v: ::std::string::String) {
        self.request_url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_url(&mut self) -> &mut ::std::string::String {
        &mut self.request_url
    }

    // Take field
    pub fn take_request_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.request_url, ::std::string::String::new())
    }

    // int64 request_size = 3;


    pub fn get_request_size(&self) -> i64 {
        self.request_size
    }
    pub fn clear_request_size(&mut self) {
        self.request_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_request_size(&mut self, v: i64) {
        self.request_size = v;
    }

    // int32 status = 4;


    pub fn get_status(&self) -> i32 {
        self.status
    }
    pub fn clear_status(&mut self) {
        self.status = 0;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: i32) {
        self.status = v;
    }

    // int64 response_size = 5;


    pub fn get_response_size(&self) -> i64 {
        self.response_size
    }
    pub fn clear_response_size(&mut self) {
        self.response_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_response_size(&mut self, v: i64) {
        self.response_size = v;
    }

    // string user_agent = 6;


    pub fn get_user_agent(&self) -> &str {
        &self.user_agent
    }
    pub fn clear_user_agent(&mut self) {
        self.user_agent.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_agent(&mut self, v: ::std::string::String) {
        self.user_agent = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_agent(&mut self) -> &mut ::std::string::String {
        &mut self.user_agent
    }

    // Take field
    pub fn take_user_agent(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_agent, ::std::string::String::new())
    }

    // string remote_ip = 7;


    pub fn get_remote_ip(&self) -> &str {
        &self.remote_ip
    }
    pub fn clear_remote_ip(&mut self) {
        self.remote_ip.clear();
    }

    // Param is passed by value, moved
    pub fn set_remote_ip(&mut self, v: ::std::string::String) {
        self.remote_ip = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remote_ip(&mut self) -> &mut ::std::string::String {
        &mut self.remote_ip
    }

    // Take field
    pub fn take_remote_ip(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.remote_ip, ::std::string::String::new())
    }

    // string server_ip = 13;


    pub fn get_server_ip(&self) -> &str {
        &self.server_ip
    }
    pub fn clear_server_ip(&mut self) {
        self.server_ip.clear();
    }

    // Param is passed by value, moved
    pub fn set_server_ip(&mut self, v: ::std::string::String) {
        self.server_ip = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_ip(&mut self) -> &mut ::std::string::String {
        &mut self.server_ip
    }

    // Take field
    pub fn take_server_ip(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.server_ip, ::std::string::String::new())
    }

    // string referer = 8;


    pub fn get_referer(&self) -> &str {
        &self.referer
    }
    pub fn clear_referer(&mut self) {
        self.referer.clear();
    }

    // Param is passed by value, moved
    pub fn set_referer(&mut self, v: ::std::string::String) {
        self.referer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_referer(&mut self) -> &mut ::std::string::String {
        &mut self.referer
    }

    // Take field
    pub fn take_referer(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.referer, ::std::string::String::new())
    }

    // .google.protobuf.Duration latency = 14;


    pub fn get_latency(&self) -> &::protobuf::well_known_types::Duration {
        self.latency.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance())
    }
    pub fn clear_latency(&mut self) {
        self.latency.clear();
    }

    pub fn has_latency(&self) -> bool {
        self.latency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latency(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.latency = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_latency(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.latency.is_none() {
            self.latency.set_default();
        }
        self.latency.as_mut().unwrap()
    }

    // Take field
    pub fn take_latency(&mut self) -> ::protobuf::well_known_types::Duration {
        self.latency.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    // bool cache_lookup = 11;


    pub fn get_cache_lookup(&self) -> bool {
        self.cache_lookup
    }
    pub fn clear_cache_lookup(&mut self) {
        self.cache_lookup = false;
    }

    // Param is passed by value, moved
    pub fn set_cache_lookup(&mut self, v: bool) {
        self.cache_lookup = v;
    }

    // bool cache_hit = 9;


    pub fn get_cache_hit(&self) -> bool {
        self.cache_hit
    }
    pub fn clear_cache_hit(&mut self) {
        self.cache_hit = false;
    }

    // Param is passed by value, moved
    pub fn set_cache_hit(&mut self, v: bool) {
        self.cache_hit = v;
    }

    // bool cache_validated_with_origin_server = 10;


    pub fn get_cache_validated_with_origin_server(&self) -> bool {
        self.cache_validated_with_origin_server
    }
    pub fn clear_cache_validated_with_origin_server(&mut self) {
        self.cache_validated_with_origin_server = false;
    }

    // Param is passed by value, moved
    pub fn set_cache_validated_with_origin_server(&mut self, v: bool) {
        self.cache_validated_with_origin_server = v;
    }

    // int64 cache_fill_bytes = 12;


    pub fn get_cache_fill_bytes(&self) -> i64 {
        self.cache_fill_bytes
    }
    pub fn clear_cache_fill_bytes(&mut self) {
        self.cache_fill_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_cache_fill_bytes(&mut self, v: i64) {
        self.cache_fill_bytes = v;
    }

    // string protocol = 15;


    pub fn get_protocol(&self) -> &str {
        &self.protocol
    }
    pub fn clear_protocol(&mut self) {
        self.protocol.clear();
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: ::std::string::String) {
        self.protocol = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_protocol(&mut self) -> &mut ::std::string::String {
        &mut self.protocol
    }

    // Take field
    pub fn take_protocol(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.protocol, ::std::string::String::new())
    }
}

impl ::protobuf::Message for HttpRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.latency {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.request_method)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.request_url)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.request_size = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.status = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.response_size = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_agent)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.remote_ip)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.server_ip)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.referer)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.latency)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.cache_lookup = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.cache_hit = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.cache_validated_with_origin_server = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.cache_fill_bytes = tmp;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.protocol)?;
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
        if !self.request_method.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.request_method);
        }
        if !self.request_url.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.request_url);
        }
        if self.request_size != 0 {
            my_size += ::protobuf::rt::value_size(3, self.request_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.status != 0 {
            my_size += ::protobuf::rt::value_size(4, self.status, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.response_size != 0 {
            my_size += ::protobuf::rt::value_size(5, self.response_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.user_agent.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.user_agent);
        }
        if !self.remote_ip.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.remote_ip);
        }
        if !self.server_ip.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.server_ip);
        }
        if !self.referer.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.referer);
        }
        if let Some(ref v) = self.latency.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.cache_lookup != false {
            my_size += 2;
        }
        if self.cache_hit != false {
            my_size += 2;
        }
        if self.cache_validated_with_origin_server != false {
            my_size += 2;
        }
        if self.cache_fill_bytes != 0 {
            my_size += ::protobuf::rt::value_size(12, self.cache_fill_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.protocol.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.protocol);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.request_method.is_empty() {
            os.write_string(1, &self.request_method)?;
        }
        if !self.request_url.is_empty() {
            os.write_string(2, &self.request_url)?;
        }
        if self.request_size != 0 {
            os.write_int64(3, self.request_size)?;
        }
        if self.status != 0 {
            os.write_int32(4, self.status)?;
        }
        if self.response_size != 0 {
            os.write_int64(5, self.response_size)?;
        }
        if !self.user_agent.is_empty() {
            os.write_string(6, &self.user_agent)?;
        }
        if !self.remote_ip.is_empty() {
            os.write_string(7, &self.remote_ip)?;
        }
        if !self.server_ip.is_empty() {
            os.write_string(13, &self.server_ip)?;
        }
        if !self.referer.is_empty() {
            os.write_string(8, &self.referer)?;
        }
        if let Some(ref v) = self.latency.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.cache_lookup != false {
            os.write_bool(11, self.cache_lookup)?;
        }
        if self.cache_hit != false {
            os.write_bool(9, self.cache_hit)?;
        }
        if self.cache_validated_with_origin_server != false {
            os.write_bool(10, self.cache_validated_with_origin_server)?;
        }
        if self.cache_fill_bytes != 0 {
            os.write_int64(12, self.cache_fill_bytes)?;
        }
        if !self.protocol.is_empty() {
            os.write_string(15, &self.protocol)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> HttpRequest {
        HttpRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "request_method",
                |m: &HttpRequest| { &m.request_method },
                |m: &mut HttpRequest| { &mut m.request_method },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "request_url",
                |m: &HttpRequest| { &m.request_url },
                |m: &mut HttpRequest| { &mut m.request_url },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "request_size",
                |m: &HttpRequest| { &m.request_size },
                |m: &mut HttpRequest| { &mut m.request_size },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "status",
                |m: &HttpRequest| { &m.status },
                |m: &mut HttpRequest| { &mut m.status },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "response_size",
                |m: &HttpRequest| { &m.response_size },
                |m: &mut HttpRequest| { &mut m.response_size },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "user_agent",
                |m: &HttpRequest| { &m.user_agent },
                |m: &mut HttpRequest| { &mut m.user_agent },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "remote_ip",
                |m: &HttpRequest| { &m.remote_ip },
                |m: &mut HttpRequest| { &mut m.remote_ip },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "server_ip",
                |m: &HttpRequest| { &m.server_ip },
                |m: &mut HttpRequest| { &mut m.server_ip },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "referer",
                |m: &HttpRequest| { &m.referer },
                |m: &mut HttpRequest| { &mut m.referer },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                "latency",
                |m: &HttpRequest| { &m.latency },
                |m: &mut HttpRequest| { &mut m.latency },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "cache_lookup",
                |m: &HttpRequest| { &m.cache_lookup },
                |m: &mut HttpRequest| { &mut m.cache_lookup },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "cache_hit",
                |m: &HttpRequest| { &m.cache_hit },
                |m: &mut HttpRequest| { &mut m.cache_hit },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "cache_validated_with_origin_server",
                |m: &HttpRequest| { &m.cache_validated_with_origin_server },
                |m: &mut HttpRequest| { &mut m.cache_validated_with_origin_server },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "cache_fill_bytes",
                |m: &HttpRequest| { &m.cache_fill_bytes },
                |m: &mut HttpRequest| { &mut m.cache_fill_bytes },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "protocol",
                |m: &HttpRequest| { &m.protocol },
                |m: &mut HttpRequest| { &mut m.protocol },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<HttpRequest>(
                "HttpRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static HttpRequest {
        static instance: ::protobuf::rt::LazyV2<HttpRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(HttpRequest::new)
    }
}

impl ::protobuf::Clear for HttpRequest {
    fn clear(&mut self) {
        self.request_method.clear();
        self.request_url.clear();
        self.request_size = 0;
        self.status = 0;
        self.response_size = 0;
        self.user_agent.clear();
        self.remote_ip.clear();
        self.server_ip.clear();
        self.referer.clear();
        self.latency.clear();
        self.cache_lookup = false;
        self.cache_hit = false;
        self.cache_validated_with_origin_server = false;
        self.cache_fill_bytes = 0;
        self.protocol.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HttpRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HttpRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n/google/api/servicecontrol/v1/http_request.proto\x12\x1cgoogle.api.ser\
    vicecontrol.v1\x1a\x1egoogle/protobuf/duration.proto\"\xaf\x04\n\x0bHttp\
    Request\x12%\n\x0erequest_method\x18\x01\x20\x01(\tR\rrequestMethod\x12\
    \x1f\n\x0brequest_url\x18\x02\x20\x01(\tR\nrequestUrl\x12!\n\x0crequest_\
    size\x18\x03\x20\x01(\x03R\x0brequestSize\x12\x16\n\x06status\x18\x04\
    \x20\x01(\x05R\x06status\x12#\n\rresponse_size\x18\x05\x20\x01(\x03R\x0c\
    responseSize\x12\x1d\n\nuser_agent\x18\x06\x20\x01(\tR\tuserAgent\x12\
    \x1b\n\tremote_ip\x18\x07\x20\x01(\tR\x08remoteIp\x12\x1b\n\tserver_ip\
    \x18\r\x20\x01(\tR\x08serverIp\x12\x18\n\x07referer\x18\x08\x20\x01(\tR\
    \x07referer\x123\n\x07latency\x18\x0e\x20\x01(\x0b2\x19.google.protobuf.\
    DurationR\x07latency\x12!\n\x0ccache_lookup\x18\x0b\x20\x01(\x08R\x0bcac\
    heLookup\x12\x1b\n\tcache_hit\x18\t\x20\x01(\x08R\x08cacheHit\x12J\n\"ca\
    che_validated_with_origin_server\x18\n\x20\x01(\x08R\x1ecacheValidatedWi\
    thOriginServer\x12(\n\x10cache_fill_bytes\x18\x0c\x20\x01(\x03R\x0ecache\
    FillBytes\x12\x1a\n\x08protocol\x18\x0f\x20\x01(\tR\x08protocolB\xe8\x01\
    \n\x20com.google.api.servicecontrol.v1B\x10HttpRequestProtoP\x01ZJgoogle\
    .golang.org/genproto/googleapis/api/servicecontrol/v1;servicecontrol\xaa\
    \x02\x1eGoogle.Cloud.ServiceControl.V1\xca\x02\x1eGoogle\\Cloud\\Service\
    Control\\V1\xea\x02!Google::Cloud::ServiceControl::V1J\xdc\x1a\n\x06\x12\
    \x04\x0e\0\\\x01\n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyr\
    ight\x202021\x20Google\x20LLC\n\n\x20Licensed\x20under\x20the\x20Apache\
    \x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\
    \x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20\
    the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20L\
    icense\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICEN\
    SE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agr\
    eed\x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0%\n\t\
    \n\x02\x03\0\x12\x03\x12\0(\n\x08\n\x01\x08\x12\x03\x14\0;\n\t\n\x02\x08\
    %\x12\x03\x14\0;\n\x08\n\x01\x08\x12\x03\x15\0a\n\t\n\x02\x08\x0b\x12\
    \x03\x15\0a\n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\x08\n\x12\x03\x16\
    \0\"\n\x08\n\x01\x08\x12\x03\x17\01\n\t\n\x02\x08\x08\x12\x03\x17\01\n\
    \x08\n\x01\x08\x12\x03\x18\09\n\t\n\x02\x08\x01\x12\x03\x18\09\n\x08\n\
    \x01\x08\x12\x03\x19\0;\n\t\n\x02\x08)\x12\x03\x19\0;\n\x08\n\x01\x08\
    \x12\x03\x1a\0:\n\t\n\x02\x08-\x12\x03\x1a\0:\n\xc1\x01\n\x02\x04\0\x12\
    \x04\x1f\0\\\x01\x1a\xb4\x01\x20A\x20common\x20proto\x20for\x20logging\
    \x20HTTP\x20requests.\x20Only\x20contains\x20semantics\n\x20defined\x20b\
    y\x20the\x20HTTP\x20specification.\x20Product-specific\x20logging\n\x20i\
    nformation\x20MUST\x20be\x20defined\x20in\x20a\x20separate\x20message.\n\
    \n\n\n\x03\x04\0\x01\x12\x03\x1f\x08\x13\nR\n\x04\x04\0\x02\0\x12\x03!\
    \x02\x1c\x1aE\x20The\x20request\x20method.\x20Examples:\x20`\"GET\"`,\
    \x20`\"HEAD\"`,\x20`\"PUT\"`,\x20`\"POST\"`.\n\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03!\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03!\t\x17\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03!\x1a\x1b\n\xae\x01\n\x04\x04\0\x02\x01\x12\
    \x03&\x02\x19\x1a\xa0\x01\x20The\x20scheme\x20(http,\x20https),\x20the\
    \x20host\x20name,\x20the\x20path,\x20and\x20the\x20query\n\x20portion\
    \x20of\x20the\x20URL\x20that\x20was\x20requested.\n\x20Example:\x20`\"ht\
    tp://example.com/some/info?color=red\"`.\n\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03&\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03&\t\x14\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03&\x17\x18\nr\n\x04\x04\0\x02\x02\x12\x03*\
    \x02\x19\x1ae\x20The\x20size\x20of\x20the\x20HTTP\x20request\x20message\
    \x20in\x20bytes,\x20including\x20the\x20request\n\x20headers\x20and\x20t\
    he\x20request\x20body.\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03*\x02\x07\
    \n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03*\x08\x14\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03*\x17\x18\n\\\n\x04\x04\0\x02\x03\x12\x03.\x02\x13\x1aO\
    \x20The\x20response\x20code\x20indicating\x20the\x20status\x20of\x20the\
    \x20response.\n\x20Examples:\x20200,\x20404.\n\n\x0c\n\x05\x04\0\x02\x03\
    \x05\x12\x03.\x02\x07\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03.\x08\x0e\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03.\x11\x12\n\x8f\x01\n\x04\x04\0\x02\
    \x04\x12\x032\x02\x1a\x1a\x81\x01\x20The\x20size\x20of\x20the\x20HTTP\
    \x20response\x20message\x20sent\x20back\x20to\x20the\x20client,\x20in\
    \x20bytes,\n\x20including\x20the\x20response\x20headers\x20and\x20the\
    \x20response\x20body.\n\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x032\x02\x07\n\
    \x0c\n\x05\x04\0\x02\x04\x01\x12\x032\x08\x15\n\x0c\n\x05\x04\0\x02\x04\
    \x03\x12\x032\x18\x19\n\x8c\x01\n\x04\x04\0\x02\x05\x12\x037\x02\x18\x1a\
    \x7f\x20The\x20user\x20agent\x20sent\x20by\x20the\x20client.\x20Example:\
    \n\x20`\"Mozilla/4.0\x20(compatible;\x20MSIE\x206.0;\x20Windows\x2098;\
    \x20Q312461;\x20.NET\n\x20CLR\x201.0.3705)\"`.\n\n\x0c\n\x05\x04\0\x02\
    \x05\x05\x12\x037\x02\x08\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x037\t\x13\n\
    \x0c\n\x05\x04\0\x02\x05\x03\x12\x037\x16\x17\n\x94\x01\n\x04\x04\0\x02\
    \x06\x12\x03;\x02\x17\x1a\x86\x01\x20The\x20IP\x20address\x20(IPv4\x20or\
    \x20IPv6)\x20of\x20the\x20client\x20that\x20issued\x20the\x20HTTP\n\x20r\
    equest.\x20Examples:\x20`\"192.168.1.1\"`,\x20`\"FE80::0202:B3FF:FE1E:83\
    29\"`.\n\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03;\x02\x08\n\x0c\n\x05\x04\
    \0\x02\x06\x01\x12\x03;\t\x12\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03;\x15\
    \x16\n`\n\x04\x04\0\x02\x07\x12\x03?\x02\x18\x1aS\x20The\x20IP\x20addres\
    s\x20(IPv4\x20or\x20IPv6)\x20of\x20the\x20origin\x20server\x20that\x20th\
    e\x20request\x20was\n\x20sent\x20to.\n\n\x0c\n\x05\x04\0\x02\x07\x05\x12\
    \x03?\x02\x08\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03?\t\x12\n\x0c\n\x05\
    \x04\0\x02\x07\x03\x12\x03?\x15\x17\n\x9c\x01\n\x04\x04\0\x02\x08\x12\
    \x03D\x02\x15\x1a\x8e\x01\x20The\x20referer\x20URL\x20of\x20the\x20reque\
    st,\x20as\x20defined\x20in\n\x20[HTTP/1.1\x20Header\x20Field\n\x20Defini\
    tions](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html).\n\n\x0c\
    \n\x05\x04\0\x02\x08\x05\x12\x03D\x02\x08\n\x0c\n\x05\x04\0\x02\x08\x01\
    \x12\x03D\t\x10\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03D\x13\x14\n\x81\x01\
    \n\x04\x04\0\x02\t\x12\x03H\x02(\x1at\x20The\x20request\x20processing\
    \x20latency\x20on\x20the\x20server,\x20from\x20the\x20time\x20the\x20req\
    uest\x20was\n\x20received\x20until\x20the\x20response\x20was\x20sent.\n\
    \n\x0c\n\x05\x04\0\x02\t\x06\x12\x03H\x02\x1a\n\x0c\n\x05\x04\0\x02\t\
    \x01\x12\x03H\x1b\"\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03H%'\n;\n\x04\x04\
    \0\x02\n\x12\x03K\x02\x19\x1a.\x20Whether\x20or\x20not\x20a\x20cache\x20\
    lookup\x20was\x20attempted.\n\n\x0c\n\x05\x04\0\x02\n\x05\x12\x03K\x02\
    \x06\n\x0c\n\x05\x04\0\x02\n\x01\x12\x03K\x07\x13\n\x0c\n\x05\x04\0\x02\
    \n\x03\x12\x03K\x16\x18\n\\\n\x04\x04\0\x02\x0b\x12\x03O\x02\x15\x1aO\
    \x20Whether\x20or\x20not\x20an\x20entity\x20was\x20served\x20from\x20cac\
    he\n\x20(with\x20or\x20without\x20validation).\n\n\x0c\n\x05\x04\0\x02\
    \x0b\x05\x12\x03O\x02\x06\n\x0c\n\x05\x04\0\x02\x0b\x01\x12\x03O\x07\x10\
    \n\x0c\n\x05\x04\0\x02\x0b\x03\x12\x03O\x13\x14\n\xa8\x01\n\x04\x04\0\
    \x02\x0c\x12\x03T\x02/\x1a\x9a\x01\x20Whether\x20or\x20not\x20the\x20res\
    ponse\x20was\x20validated\x20with\x20the\x20origin\x20server\x20before\n\
    \x20being\x20served\x20from\x20cache.\x20This\x20field\x20is\x20only\x20\
    meaningful\x20if\x20`cache_hit`\x20is\n\x20True.\n\n\x0c\n\x05\x04\0\x02\
    \x0c\x05\x12\x03T\x02\x06\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\x03T\x07)\n\
    \x0c\n\x05\x04\0\x02\x0c\x03\x12\x03T,.\np\n\x04\x04\0\x02\r\x12\x03X\
    \x02\x1e\x1ac\x20The\x20number\x20of\x20HTTP\x20response\x20bytes\x20ins\
    erted\x20into\x20cache.\x20Set\x20only\x20when\x20a\n\x20cache\x20fill\
    \x20was\x20attempted.\n\n\x0c\n\x05\x04\0\x02\r\x05\x12\x03X\x02\x07\n\
    \x0c\n\x05\x04\0\x02\r\x01\x12\x03X\x08\x18\n\x0c\n\x05\x04\0\x02\r\x03\
    \x12\x03X\x1b\x1d\nY\n\x04\x04\0\x02\x0e\x12\x03[\x02\x17\x1aL\x20Protoc\
    ol\x20used\x20for\x20the\x20request.\x20Examples:\x20\"HTTP/1.1\",\x20\"\
    HTTP/2\",\x20\"websocket\"\n\n\x0c\n\x05\x04\0\x02\x0e\x05\x12\x03[\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x0e\x01\x12\x03[\t\x11\n\x0c\n\x05\x04\0\x02\
    \x0e\x03\x12\x03[\x14\x16b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
