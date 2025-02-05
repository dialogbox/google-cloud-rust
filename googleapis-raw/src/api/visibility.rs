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
//! Generated file from `google/api/visibility.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct Visibility {
    // message fields
    pub rules: ::protobuf::RepeatedField<VisibilityRule>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Visibility {
    fn default() -> &'a Visibility {
        <Visibility as ::protobuf::Message>::default_instance()
    }
}

impl Visibility {
    pub fn new() -> Visibility {
        ::std::default::Default::default()
    }

    // repeated .google.api.VisibilityRule rules = 1;


    pub fn get_rules(&self) -> &[VisibilityRule] {
        &self.rules
    }
    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<VisibilityRule>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<VisibilityRule> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<VisibilityRule> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Visibility {
    fn is_initialized(&self) -> bool {
        for v in &self.rules {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rules)?;
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
        for value in &self.rules {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.rules {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> Visibility {
        Visibility::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VisibilityRule>>(
                "rules",
                |m: &Visibility| { &m.rules },
                |m: &mut Visibility| { &mut m.rules },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Visibility>(
                "Visibility",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Visibility {
        static instance: ::protobuf::rt::LazyV2<Visibility> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Visibility::new)
    }
}

impl ::protobuf::Clear for Visibility {
    fn clear(&mut self) {
        self.rules.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Visibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Visibility {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VisibilityRule {
    // message fields
    pub selector: ::std::string::String,
    pub restriction: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a VisibilityRule {
    fn default() -> &'a VisibilityRule {
        <VisibilityRule as ::protobuf::Message>::default_instance()
    }
}

impl VisibilityRule {
    pub fn new() -> VisibilityRule {
        ::std::default::Default::default()
    }

    // string selector = 1;


    pub fn get_selector(&self) -> &str {
        &self.selector
    }
    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: ::std::string::String) {
        self.selector = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selector(&mut self) -> &mut ::std::string::String {
        &mut self.selector
    }

    // Take field
    pub fn take_selector(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.selector, ::std::string::String::new())
    }

    // string restriction = 2;


    pub fn get_restriction(&self) -> &str {
        &self.restriction
    }
    pub fn clear_restriction(&mut self) {
        self.restriction.clear();
    }

    // Param is passed by value, moved
    pub fn set_restriction(&mut self, v: ::std::string::String) {
        self.restriction = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_restriction(&mut self) -> &mut ::std::string::String {
        &mut self.restriction
    }

    // Take field
    pub fn take_restriction(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.restriction, ::std::string::String::new())
    }
}

impl ::protobuf::Message for VisibilityRule {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.selector)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.restriction)?;
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
        if !self.selector.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.selector);
        }
        if !self.restriction.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.restriction);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.selector.is_empty() {
            os.write_string(1, &self.selector)?;
        }
        if !self.restriction.is_empty() {
            os.write_string(2, &self.restriction)?;
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

    fn new() -> VisibilityRule {
        VisibilityRule::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "selector",
                |m: &VisibilityRule| { &m.selector },
                |m: &mut VisibilityRule| { &mut m.selector },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "restriction",
                |m: &VisibilityRule| { &m.restriction },
                |m: &mut VisibilityRule| { &mut m.restriction },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<VisibilityRule>(
                "VisibilityRule",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static VisibilityRule {
        static instance: ::protobuf::rt::LazyV2<VisibilityRule> = ::protobuf::rt::LazyV2::INIT;
        instance.get(VisibilityRule::new)
    }
}

impl ::protobuf::Clear for VisibilityRule {
    fn clear(&mut self) {
        self.selector.clear();
        self.restriction.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VisibilityRule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VisibilityRule {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

/// Extension fields
pub mod exts {

    pub const enum_visibility: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, ::protobuf::types::ProtobufTypeMessage<super::VisibilityRule>> = ::protobuf::ext::ExtFieldOptional { field_number: 72295727, phantom: ::std::marker::PhantomData };

    pub const value_visibility: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumValueOptions, ::protobuf::types::ProtobufTypeMessage<super::VisibilityRule>> = ::protobuf::ext::ExtFieldOptional { field_number: 72295727, phantom: ::std::marker::PhantomData };

    pub const field_visibility: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeMessage<super::VisibilityRule>> = ::protobuf::ext::ExtFieldOptional { field_number: 72295727, phantom: ::std::marker::PhantomData };

    pub const message_visibility: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeMessage<super::VisibilityRule>> = ::protobuf::ext::ExtFieldOptional { field_number: 72295727, phantom: ::std::marker::PhantomData };

    pub const method_visibility: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MethodOptions, ::protobuf::types::ProtobufTypeMessage<super::VisibilityRule>> = ::protobuf::ext::ExtFieldOptional { field_number: 72295727, phantom: ::std::marker::PhantomData };

    pub const api_visibility: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::ServiceOptions, ::protobuf::types::ProtobufTypeMessage<super::VisibilityRule>> = ::protobuf::ext::ExtFieldOptional { field_number: 72295727, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgoogle/api/visibility.proto\x12\ngoogle.api\x1a\x20google/protobuf\
    /descriptor.proto\">\n\nVisibility\x120\n\x05rules\x18\x01\x20\x03(\x0b2\
    \x1a.google.api.VisibilityRuleR\x05rules\"N\n\x0eVisibilityRule\x12\x1a\
    \n\x08selector\x18\x01\x20\x01(\tR\x08selector\x12\x20\n\x0brestriction\
    \x18\x02\x20\x01(\tR\x0brestriction:d\n\x0fenum_visibility\x18\xaf\xca\
    \xbc\"\x20\x01(\x0b2\x1a.google.api.VisibilityRule\x12\x1c.google.protob\
    uf.EnumOptionsR\x0eenumVisibility:k\n\x10value_visibility\x18\xaf\xca\
    \xbc\"\x20\x01(\x0b2\x1a.google.api.VisibilityRule\x12!.google.protobuf.\
    EnumValueOptionsR\x0fvalueVisibility:g\n\x10field_visibility\x18\xaf\xca\
    \xbc\"\x20\x01(\x0b2\x1a.google.api.VisibilityRule\x12\x1d.google.protob\
    uf.FieldOptionsR\x0ffieldVisibility:m\n\x12message_visibility\x18\xaf\
    \xca\xbc\"\x20\x01(\x0b2\x1a.google.api.VisibilityRule\x12\x1f.google.pr\
    otobuf.MessageOptionsR\x11messageVisibility:j\n\x11method_visibility\x18\
    \xaf\xca\xbc\"\x20\x01(\x0b2\x1a.google.api.VisibilityRule\x12\x1e.googl\
    e.protobuf.MethodOptionsR\x10methodVisibility:e\n\x0eapi_visibility\x18\
    \xaf\xca\xbc\"\x20\x01(\x0b2\x1a.google.api.VisibilityRule\x12\x1f.googl\
    e.protobuf.ServiceOptionsR\rapiVisibilityBn\n\x0ecom.google.apiB\x0fVisi\
    bilityProtoP\x01Z?google.golang.org/genproto/googleapis/api/visibility;v\
    isibility\xf8\x01\x01\xa2\x02\x04GAPIJ\xa6\x19\n\x06\x12\x04\x0e\0n\x01\
    \n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202021\x20\
    Google\x20LLC\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20V\
    ersion\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20\
    this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\
    \x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\
    \x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Un\
    less\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\
    \x20writing,\x20software\n\x20distributed\x20under\x20the\x20License\x20\
    is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20\
    WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20expres\
    s\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0\x13\n\t\n\x02\x03\0\
    \x12\x03\x12\0*\n\x08\n\x01\x08\x12\x03\x14\0\x1f\n\t\n\x02\x08\x1f\x12\
    \x03\x14\0\x1f\n\x08\n\x01\x08\x12\x03\x15\0V\n\t\n\x02\x08\x0b\x12\x03\
    \x15\0V\n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\x08\n\x12\x03\x16\0\"\
    \n\x08\n\x01\x08\x12\x03\x17\00\n\t\n\x02\x08\x08\x12\x03\x17\00\n\x08\n\
    \x01\x08\x12\x03\x18\0'\n\t\n\x02\x08\x01\x12\x03\x18\0'\n\x08\n\x01\x08\
    \x12\x03\x19\0\"\n\t\n\x02\x08$\x12\x03\x19\0\"\n\t\n\x01\x07\x12\x04\
    \x1b\0\x1e\x01\n\"\n\x02\x07\0\x12\x03\x1d\x027\x1a\x17\x20See\x20`Visib\
    ilityRule`.\n\n\n\n\x03\x07\0\x02\x12\x03\x1b\x07\"\n\n\n\x03\x07\0\x06\
    \x12\x03\x1d\x02\x1b\n\n\n\x03\x07\0\x01\x12\x03\x1d\x1c+\n\n\n\x03\x07\
    \0\x03\x12\x03\x1d.6\n\t\n\x01\x07\x12\x04\x20\0#\x01\n\"\n\x02\x07\x01\
    \x12\x03\"\x028\x1a\x17\x20See\x20`VisibilityRule`.\n\n\n\n\x03\x07\x01\
    \x02\x12\x03\x20\x07'\n\n\n\x03\x07\x01\x06\x12\x03\"\x02\x1b\n\n\n\x03\
    \x07\x01\x01\x12\x03\"\x1c,\n\n\n\x03\x07\x01\x03\x12\x03\"/7\n\t\n\x01\
    \x07\x12\x04%\0(\x01\n\"\n\x02\x07\x02\x12\x03'\x028\x1a\x17\x20See\x20`\
    VisibilityRule`.\n\n\n\n\x03\x07\x02\x02\x12\x03%\x07#\n\n\n\x03\x07\x02\
    \x06\x12\x03'\x02\x1b\n\n\n\x03\x07\x02\x01\x12\x03'\x1c,\n\n\n\x03\x07\
    \x02\x03\x12\x03'/7\n\t\n\x01\x07\x12\x04*\0-\x01\n\"\n\x02\x07\x03\x12\
    \x03,\x02:\x1a\x17\x20See\x20`VisibilityRule`.\n\n\n\n\x03\x07\x03\x02\
    \x12\x03*\x07%\n\n\n\x03\x07\x03\x06\x12\x03,\x02\x1b\n\n\n\x03\x07\x03\
    \x01\x12\x03,\x1c.\n\n\n\x03\x07\x03\x03\x12\x03,19\n\t\n\x01\x07\x12\
    \x04/\02\x01\n\"\n\x02\x07\x04\x12\x031\x029\x1a\x17\x20See\x20`Visibili\
    tyRule`.\n\n\n\n\x03\x07\x04\x02\x12\x03/\x07$\n\n\n\x03\x07\x04\x06\x12\
    \x031\x02\x1b\n\n\n\x03\x07\x04\x01\x12\x031\x1c-\n\n\n\x03\x07\x04\x03\
    \x12\x03108\n\t\n\x01\x07\x12\x044\07\x01\n\"\n\x02\x07\x05\x12\x036\x02\
    6\x1a\x17\x20See\x20`VisibilityRule`.\n\n\n\n\x03\x07\x05\x02\x12\x034\
    \x07%\n\n\n\x03\x07\x05\x06\x12\x036\x02\x1b\n\n\n\x03\x07\x05\x01\x12\
    \x036\x1c*\n\n\n\x03\x07\x05\x03\x12\x036-5\n\xac\x06\n\x02\x04\0\x12\
    \x04O\0T\x01\x1a\x9f\x06\x20`Visibility`\x20defines\x20restrictions\x20f\
    or\x20the\x20visibility\x20of\x20service\n\x20elements.\x20\x20Restricti\
    ons\x20are\x20specified\x20using\x20visibility\x20labels\n\x20(e.g.,\x20\
    PREVIEW)\x20that\x20are\x20elsewhere\x20linked\x20to\x20users\x20and\x20\
    projects.\n\n\x20Users\x20and\x20projects\x20can\x20have\x20access\x20to\
    \x20more\x20than\x20one\x20visibility\x20label.\x20The\n\x20effective\
    \x20visibility\x20for\x20multiple\x20labels\x20is\x20the\x20union\x20of\
    \x20each\x20label's\n\x20elements,\x20plus\x20any\x20unrestricted\x20ele\
    ments.\n\n\x20If\x20an\x20element\x20and\x20its\x20parents\x20have\x20no\
    \x20restrictions,\x20visibility\x20is\n\x20unconditionally\x20granted.\n\
    \n\x20Example:\n\n\x20\x20\x20\x20\x20visibility:\n\x20\x20\x20\x20\x20\
    \x20\x20rules:\n\x20\x20\x20\x20\x20\x20\x20-\x20selector:\x20google.cal\
    endar.Calendar.EnhancedSearch\n\x20\x20\x20\x20\x20\x20\x20\x20\x20restr\
    iction:\x20PREVIEW\n\x20\x20\x20\x20\x20\x20\x20-\x20selector:\x20google\
    .calendar.Calendar.Delegate\n\x20\x20\x20\x20\x20\x20\x20\x20\x20restric\
    tion:\x20INTERNAL\n\n\x20Here,\x20all\x20methods\x20are\x20publicly\x20v\
    isible\x20except\x20for\x20the\x20restricted\x20methods\n\x20EnhancedSea\
    rch\x20and\x20Delegate.\n\n\n\n\x03\x04\0\x01\x12\x03O\x08\x12\n\x9b\x01\
    \n\x04\x04\0\x02\0\x12\x03S\x02$\x1a\x8d\x01\x20A\x20list\x20of\x20visib\
    ility\x20rules\x20that\x20apply\x20to\x20individual\x20API\x20elements.\
    \n\n\x20**NOTE:**\x20All\x20service\x20configuration\x20rules\x20follow\
    \x20\"last\x20one\x20wins\"\x20order.\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\
    \x03S\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03S\x0b\x19\n\x0c\n\x05\x04\
    \0\x02\0\x01\x12\x03S\x1a\x1f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03S\"#\na\
    \n\x02\x04\x01\x12\x04X\0n\x01\x1aU\x20A\x20visibility\x20rule\x20provid\
    es\x20visibility\x20configuration\x20for\x20an\x20individual\x20API\n\
    \x20element.\n\n\n\n\x03\x04\x01\x01\x12\x03X\x08\x16\n\xab\x01\n\x04\
    \x04\x01\x02\0\x12\x03\\\x02\x16\x1a\x9d\x01\x20Selects\x20methods,\x20m\
    essages,\x20fields,\x20enums,\x20etc.\x20to\x20which\x20this\x20rule\x20\
    applies.\n\n\x20Refer\x20to\x20[selector][google.api.DocumentationRule.s\
    elector]\x20for\x20syntax\x20details.\n\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03\\\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\\\t\x11\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\\\x14\x15\n\x8d\x04\n\x04\x04\x01\x02\x01\
    \x12\x03m\x02\x19\x1a\xff\x03\x20A\x20comma-separated\x20list\x20of\x20v\
    isibility\x20labels\x20that\x20apply\x20to\x20the\x20`selector`.\n\x20An\
    y\x20of\x20the\x20listed\x20labels\x20can\x20be\x20used\x20to\x20grant\
    \x20the\x20visibility.\n\n\x20If\x20a\x20rule\x20has\x20multiple\x20labe\
    ls,\x20removing\x20one\x20of\x20the\x20labels\x20but\x20not\x20all\x20of\
    \n\x20them\x20can\x20break\x20clients.\n\n\x20Example:\n\n\x20\x20\x20\
    \x20\x20visibility:\n\x20\x20\x20\x20\x20\x20\x20rules:\n\x20\x20\x20\
    \x20\x20\x20\x20-\x20selector:\x20google.calendar.Calendar.EnhancedSearc\
    h\n\x20\x20\x20\x20\x20\x20\x20\x20\x20restriction:\x20INTERNAL,\x20PREV\
    IEW\n\n\x20Removing\x20INTERNAL\x20from\x20this\x20restriction\x20will\
    \x20break\x20clients\x20that\x20rely\x20on\n\x20this\x20method\x20and\
    \x20only\x20had\x20access\x20to\x20it\x20through\x20INTERNAL.\n\n\x0c\n\
    \x05\x04\x01\x02\x01\x05\x12\x03m\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03m\t\x14\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03m\x17\x18b\
    \x06proto3\
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
