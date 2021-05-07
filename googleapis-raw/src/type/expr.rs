// This file is generated by rust-protobuf 2.23.0. Do not edit
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
//! Generated file from `google/type/expr.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_23_0;

#[derive(PartialEq,Clone,Default)]
pub struct Expr {
    // message fields
    pub expression: ::std::string::String,
    pub title: ::std::string::String,
    pub description: ::std::string::String,
    pub location: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Expr {
    fn default() -> &'a Expr {
        <Expr as ::protobuf::Message>::default_instance()
    }
}

impl Expr {
    pub fn new() -> Expr {
        ::std::default::Default::default()
    }

    // string expression = 1;


    pub fn get_expression(&self) -> &str {
        &self.expression
    }
    pub fn clear_expression(&mut self) {
        self.expression.clear();
    }

    // Param is passed by value, moved
    pub fn set_expression(&mut self, v: ::std::string::String) {
        self.expression = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_expression(&mut self) -> &mut ::std::string::String {
        &mut self.expression
    }

    // Take field
    pub fn take_expression(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.expression, ::std::string::String::new())
    }

    // string title = 2;


    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        &mut self.title
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.title, ::std::string::String::new())
    }

    // string description = 3;


    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    // string location = 4;


    pub fn get_location(&self) -> &str {
        &self.location
    }
    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::std::string::String) {
        self.location = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut ::std::string::String {
        &mut self.location
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.location, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Expr {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.expression)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.title)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.location)?;
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
        if !self.expression.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.expression);
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.title);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        if !self.location.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.location);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.expression.is_empty() {
            os.write_string(1, &self.expression)?;
        }
        if !self.title.is_empty() {
            os.write_string(2, &self.title)?;
        }
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
        }
        if !self.location.is_empty() {
            os.write_string(4, &self.location)?;
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

    fn new() -> Expr {
        Expr::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "expression",
                |m: &Expr| { &m.expression },
                |m: &mut Expr| { &mut m.expression },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "title",
                |m: &Expr| { &m.title },
                |m: &mut Expr| { &mut m.title },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "description",
                |m: &Expr| { &m.description },
                |m: &mut Expr| { &mut m.description },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "location",
                |m: &Expr| { &m.location },
                |m: &mut Expr| { &mut m.location },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Expr>(
                "Expr",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Expr {
        static instance: ::protobuf::rt::LazyV2<Expr> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Expr::new)
    }
}

impl ::protobuf::Clear for Expr {
    fn clear(&mut self) {
        self.expression.clear();
        self.title.clear();
        self.description.clear();
        self.location.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Expr {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16google/type/expr.proto\x12\x0bgoogle.type\"z\n\x04Expr\x12\x1e\n\n\
    expression\x18\x01\x20\x01(\tR\nexpression\x12\x14\n\x05title\x18\x02\
    \x20\x01(\tR\x05title\x12\x20\n\x0bdescription\x18\x03\x20\x01(\tR\x0bde\
    scription\x12\x1a\n\x08location\x18\x04\x20\x01(\tR\x08locationBZ\n\x0fc\
    om.google.typeB\tExprProtoP\x01Z4google.golang.org/genproto/googleapis/t\
    ype/expr;expr\xa2\x02\x03GTPJ\x9e\x0e\n\x06\x12\x04\x0f\02\x01\n\xbe\x04\
    \n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyright\x202019\x20Google\x20\
    LLC.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\
    \x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\
    \x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20Y\
    ou\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\
    \x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\
    \x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20w\
    riting,\x20software\n\x20distributed\x20under\x20the\x20License\x20is\
    \x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WA\
    RRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\
    \x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x14\n\x08\n\x01\
    \x08\x12\x03\x13\0K\n\t\n\x02\x08\x0b\x12\x03\x13\0K\n\x08\n\x01\x08\x12\
    \x03\x14\0\"\n\t\n\x02\x08\n\x12\x03\x14\0\"\n\x08\n\x01\x08\x12\x03\x15\
    \0*\n\t\n\x02\x08\x08\x12\x03\x15\0*\n\x08\n\x01\x08\x12\x03\x16\0(\n\t\
    \n\x02\x08\x01\x12\x03\x16\0(\n\x08\n\x01\x08\x12\x03\x17\0!\n\t\n\x02\
    \x08$\x12\x03\x17\0!\n\xcb\x01\n\x02\x04\0\x12\x04\x1e\02\x01\x1a\xbe\
    \x01\x20Represents\x20an\x20expression\x20text.\x20Example:\n\n\x20\x20\
    \x20\x20\x20title:\x20\"User\x20account\x20presence\"\n\x20\x20\x20\x20\
    \x20description:\x20\"Determines\x20whether\x20the\x20request\x20has\x20\
    a\x20user\x20account\"\n\x20\x20\x20\x20\x20expression:\x20\"size(reques\
    t.user)\x20>\x200\"\n\n\n\n\x03\x04\0\x01\x12\x03\x1e\x08\x0c\n\xd0\x01\
    \n\x04\x04\0\x02\0\x12\x03$\x02\x18\x1a\xc2\x01\x20Textual\x20representa\
    tion\x20of\x20an\x20expression\x20in\n\x20Common\x20Expression\x20Langua\
    ge\x20syntax.\n\n\x20The\x20application\x20context\x20of\x20the\x20conta\
    ining\x20message\x20determines\x20which\n\x20well-known\x20feature\x20se\
    t\x20of\x20CEL\x20is\x20supported.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04$\
    \x02\x1e\x0e\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03$\x02\x08\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03$\t\x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03$\x16\
    \x17\n\xa5\x01\n\x04\x04\0\x02\x01\x12\x03)\x02\x13\x1a\x97\x01\x20An\
    \x20optional\x20title\x20for\x20the\x20expression,\x20i.e.\x20a\x20short\
    \x20string\x20describing\n\x20its\x20purpose.\x20This\x20can\x20be\x20us\
    ed\x20e.g.\x20in\x20UIs\x20which\x20allow\x20to\x20enter\x20the\n\x20exp\
    ression.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04)\x02$\x18\n\x0c\n\x05\x04\
    \0\x02\x01\x05\x12\x03)\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03)\t\
    \x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03)\x11\x12\n\x94\x01\n\x04\x04\
    \0\x02\x02\x12\x03-\x02\x19\x1a\x86\x01\x20An\x20optional\x20description\
    \x20of\x20the\x20expression.\x20This\x20is\x20a\x20longer\x20text\x20whi\
    ch\n\x20describes\x20the\x20expression,\x20e.g.\x20when\x20hovered\x20ov\
    er\x20it\x20in\x20a\x20UI.\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04-\x02)\
    \x13\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03-\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03-\t\x14\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03-\x17\
    \x18\n\x8f\x01\n\x04\x04\0\x02\x03\x12\x031\x02\x16\x1a\x81\x01\x20An\
    \x20optional\x20string\x20indicating\x20the\x20location\x20of\x20the\x20\
    expression\x20for\x20error\n\x20reporting,\x20e.g.\x20a\x20file\x20name\
    \x20and\x20a\x20position\x20in\x20the\x20file.\n\n\r\n\x05\x04\0\x02\x03\
    \x04\x12\x041\x02-\x19\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x031\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x03\x01\x12\x031\t\x11\n\x0c\n\x05\x04\0\x02\x03\
    \x03\x12\x031\x14\x15b\x06proto3\
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