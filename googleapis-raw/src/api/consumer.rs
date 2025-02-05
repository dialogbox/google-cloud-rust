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
//! Generated file from `google/api/consumer.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct ProjectProperties {
    // message fields
    pub properties: ::protobuf::RepeatedField<Property>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ProjectProperties {
    fn default() -> &'a ProjectProperties {
        <ProjectProperties as ::protobuf::Message>::default_instance()
    }
}

impl ProjectProperties {
    pub fn new() -> ProjectProperties {
        ::std::default::Default::default()
    }

    // repeated .google.api.Property properties = 1;


    pub fn get_properties(&self) -> &[Property] {
        &self.properties
    }
    pub fn clear_properties(&mut self) {
        self.properties.clear();
    }

    // Param is passed by value, moved
    pub fn set_properties(&mut self, v: ::protobuf::RepeatedField<Property>) {
        self.properties = v;
    }

    // Mutable pointer to the field.
    pub fn mut_properties(&mut self) -> &mut ::protobuf::RepeatedField<Property> {
        &mut self.properties
    }

    // Take field
    pub fn take_properties(&mut self) -> ::protobuf::RepeatedField<Property> {
        ::std::mem::replace(&mut self.properties, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ProjectProperties {
    fn is_initialized(&self) -> bool {
        for v in &self.properties {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.properties)?;
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
        for value in &self.properties {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.properties {
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

    fn new() -> ProjectProperties {
        ProjectProperties::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Property>>(
                "properties",
                |m: &ProjectProperties| { &m.properties },
                |m: &mut ProjectProperties| { &mut m.properties },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ProjectProperties>(
                "ProjectProperties",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ProjectProperties {
        static instance: ::protobuf::rt::LazyV2<ProjectProperties> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ProjectProperties::new)
    }
}

impl ::protobuf::Clear for ProjectProperties {
    fn clear(&mut self) {
        self.properties.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProjectProperties {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProjectProperties {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Property {
    // message fields
    pub name: ::std::string::String,
    pub field_type: Property_PropertyType,
    pub description: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Property {
    fn default() -> &'a Property {
        <Property as ::protobuf::Message>::default_instance()
    }
}

impl Property {
    pub fn new() -> Property {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
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

    // .google.api.Property.PropertyType type = 2;


    pub fn get_field_type(&self) -> Property_PropertyType {
        self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type = Property_PropertyType::UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Property_PropertyType) {
        self.field_type = v;
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
}

impl ::protobuf::Message for Property {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
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
        if self.field_type != Property_PropertyType::UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.field_type != Property_PropertyType::UNSPECIFIED {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.field_type))?;
        }
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
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

    fn new() -> Property {
        Property::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Property| { &m.name },
                |m: &mut Property| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Property_PropertyType>>(
                "type",
                |m: &Property| { &m.field_type },
                |m: &mut Property| { &mut m.field_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "description",
                |m: &Property| { &m.description },
                |m: &mut Property| { &mut m.description },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Property>(
                "Property",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Property {
        static instance: ::protobuf::rt::LazyV2<Property> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Property::new)
    }
}

impl ::protobuf::Clear for Property {
    fn clear(&mut self) {
        self.name.clear();
        self.field_type = Property_PropertyType::UNSPECIFIED;
        self.description.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Property {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Property {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Property_PropertyType {
    UNSPECIFIED = 0,
    INT64 = 1,
    BOOL = 2,
    STRING = 3,
    DOUBLE = 4,
}

impl ::protobuf::ProtobufEnum for Property_PropertyType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Property_PropertyType> {
        match value {
            0 => ::std::option::Option::Some(Property_PropertyType::UNSPECIFIED),
            1 => ::std::option::Option::Some(Property_PropertyType::INT64),
            2 => ::std::option::Option::Some(Property_PropertyType::BOOL),
            3 => ::std::option::Option::Some(Property_PropertyType::STRING),
            4 => ::std::option::Option::Some(Property_PropertyType::DOUBLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Property_PropertyType] = &[
            Property_PropertyType::UNSPECIFIED,
            Property_PropertyType::INT64,
            Property_PropertyType::BOOL,
            Property_PropertyType::STRING,
            Property_PropertyType::DOUBLE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Property_PropertyType>("Property.PropertyType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Property_PropertyType {
}

impl ::std::default::Default for Property_PropertyType {
    fn default() -> Self {
        Property_PropertyType::UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for Property_PropertyType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19google/api/consumer.proto\x12\ngoogle.api\"I\n\x11ProjectPropertie\
    s\x124\n\nproperties\x18\x01\x20\x03(\x0b2\x14.google.api.PropertyR\npro\
    perties\"\xc5\x01\n\x08Property\x12\x12\n\x04name\x18\x01\x20\x01(\tR\
    \x04name\x125\n\x04type\x18\x02\x20\x01(\x0e2!.google.api.Property.Prope\
    rtyTypeR\x04type\x12\x20\n\x0bdescription\x18\x03\x20\x01(\tR\x0bdescrip\
    tion\"L\n\x0cPropertyType\x12\x0f\n\x0bUNSPECIFIED\x10\0\x12\t\n\x05INT6\
    4\x10\x01\x12\x08\n\x04BOOL\x10\x02\x12\n\n\x06STRING\x10\x03\x12\n\n\
    \x06DOUBLE\x10\x04Bh\n\x0ecom.google.apiB\rConsumerProtoP\x01ZEgoogle.go\
    lang.org/genproto/googleapis/api/serviceconfig;serviceconfigJ\xbb\x15\n\
    \x06\x12\x04\x0e\0Q\x01\n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\
    \x20Copyright\x202016\x20Google\x20LLC\n\n\x20Licensed\x20under\x20the\
    \x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20y\
    ou\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\
    \x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\
    \x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/li\
    censes/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\
    \x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20distributed\
    \x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20\
    IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20A\
    NY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\x20Li\
    cense\x20for\x20the\x20specific\x20language\x20governing\x20permissions\
    \x20and\n\x20limitations\x20under\x20the\x20License.\n\n\x08\n\x01\x02\
    \x12\x03\x10\0\x13\n\x08\n\x01\x08\x12\x03\x12\0\\\n\t\n\x02\x08\x0b\x12\
    \x03\x12\0\\\n\x08\n\x01\x08\x12\x03\x13\0\"\n\t\n\x02\x08\n\x12\x03\x13\
    \0\"\n\x08\n\x01\x08\x12\x03\x14\0.\n\t\n\x02\x08\x08\x12\x03\x14\0.\n\
    \x08\n\x01\x08\x12\x03\x15\0'\n\t\n\x02\x08\x01\x12\x03\x15\0'\n\xbf\x05\
    \n\x02\x04\0\x12\x04'\0*\x01\x1a\xb2\x05\x20A\x20descriptor\x20for\x20de\
    fining\x20project\x20properties\x20for\x20a\x20service.\x20One\x20servic\
    e\x20may\n\x20have\x20many\x20consumer\x20projects,\x20and\x20the\x20ser\
    vice\x20may\x20want\x20to\x20behave\x20differently\n\x20depending\x20on\
    \x20some\x20properties\x20on\x20the\x20project.\x20For\x20example,\x20a\
    \x20project\x20may\x20be\n\x20associated\x20with\x20a\x20school,\x20or\
    \x20a\x20business,\x20or\x20a\x20government\x20agency,\x20a\x20business\
    \n\x20type\x20property\x20on\x20the\x20project\x20may\x20affect\x20how\
    \x20a\x20service\x20responds\x20to\x20the\x20client.\n\x20This\x20descri\
    ptor\x20defines\x20which\x20properties\x20are\x20allowed\x20to\x20be\x20\
    set\x20on\x20a\x20project.\n\n\x20Example:\n\n\x20\x20\x20\x20project_pr\
    operties:\n\x20\x20\x20\x20\x20\x20properties:\n\x20\x20\x20\x20\x20\x20\
    -\x20name:\x20NO_WATERMARK\n\x20\x20\x20\x20\x20\x20\x20\x20type:\x20BOO\
    L\n\x20\x20\x20\x20\x20\x20\x20\x20description:\x20Allows\x20usage\x20of\
    \x20the\x20API\x20without\x20watermarks.\n\x20\x20\x20\x20\x20\x20-\x20n\
    ame:\x20EXTENDED_TILE_CACHE_PERIOD\n\x20\x20\x20\x20\x20\x20\x20\x20type\
    :\x20INT64\n\n\n\n\x03\x04\0\x01\x12\x03'\x08\x19\n@\n\x04\x04\0\x02\0\
    \x12\x03)\x02#\x1a3\x20List\x20of\x20per\x20consumer\x20project-specific\
    \x20properties.\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03)\x02\n\n\x0c\n\x05\
    \x04\0\x02\0\x06\x12\x03)\x0b\x13\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03)\
    \x14\x1e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03)!\"\n\xe1\x03\n\x02\x04\x01\
    \x12\x046\0Q\x01\x1a\xd4\x03\x20Defines\x20project\x20properties.\n\n\
    \x20API\x20services\x20can\x20define\x20properties\x20that\x20can\x20be\
    \x20assigned\x20to\x20consumer\x20projects\n\x20so\x20that\x20backends\
    \x20can\x20perform\x20response\x20customization\x20without\x20having\x20\
    to\x20make\n\x20additional\x20calls\x20or\x20maintain\x20additional\x20s\
    torage.\x20For\x20example,\x20Maps\x20API\n\x20defines\x20properties\x20\
    that\x20controls\x20map\x20tile\x20cache\x20period,\x20or\x20whether\x20\
    to\x20embed\x20a\n\x20watermark\x20in\x20a\x20result.\n\n\x20These\x20va\
    lues\x20can\x20be\x20set\x20via\x20API\x20producer\x20console.\x20Only\
    \x20API\x20providers\x20can\n\x20define\x20and\x20set\x20these\x20proper\
    ties.\n\n\n\n\x03\x04\x01\x01\x12\x036\x08\x10\n:\n\x04\x04\x01\x04\0\
    \x12\x048\x02G\x03\x1a,\x20Supported\x20data\x20type\x20of\x20the\x20pro\
    perty\x20values\n\n\x0c\n\x05\x04\x01\x04\0\x01\x12\x038\x07\x13\nF\n\
    \x06\x04\x01\x04\0\x02\0\x12\x03:\x04\x14\x1a7\x20The\x20type\x20is\x20u\
    nspecified,\x20and\x20will\x20result\x20in\x20an\x20error.\n\n\x0e\n\x07\
    \x04\x01\x04\0\x02\0\x01\x12\x03:\x04\x0f\n\x0e\n\x07\x04\x01\x04\0\x02\
    \0\x02\x12\x03:\x12\x13\n%\n\x06\x04\x01\x04\0\x02\x01\x12\x03=\x04\x0e\
    \x1a\x16\x20The\x20type\x20is\x20`int64`.\n\n\x0e\n\x07\x04\x01\x04\0\
    \x02\x01\x01\x12\x03=\x04\t\n\x0e\n\x07\x04\x01\x04\0\x02\x01\x02\x12\
    \x03=\x0c\r\n$\n\x06\x04\x01\x04\0\x02\x02\x12\x03@\x04\r\x1a\x15\x20The\
    \x20type\x20is\x20`bool`.\n\n\x0e\n\x07\x04\x01\x04\0\x02\x02\x01\x12\
    \x03@\x04\x08\n\x0e\n\x07\x04\x01\x04\0\x02\x02\x02\x12\x03@\x0b\x0c\n&\
    \n\x06\x04\x01\x04\0\x02\x03\x12\x03C\x04\x0f\x1a\x17\x20The\x20type\x20\
    is\x20`string`.\n\n\x0e\n\x07\x04\x01\x04\0\x02\x03\x01\x12\x03C\x04\n\n\
    \x0e\n\x07\x04\x01\x04\0\x02\x03\x02\x12\x03C\r\x0e\n&\n\x06\x04\x01\x04\
    \0\x02\x04\x12\x03F\x04\x0f\x1a\x17\x20The\x20type\x20is\x20'double'.\n\
    \n\x0e\n\x07\x04\x01\x04\0\x02\x04\x01\x12\x03F\x04\n\n\x0e\n\x07\x04\
    \x01\x04\0\x02\x04\x02\x12\x03F\r\x0e\n4\n\x04\x04\x01\x02\0\x12\x03J\
    \x02\x12\x1a'\x20The\x20name\x20of\x20the\x20property\x20(a.k.a\x20key).\
    \n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03J\x02\x08\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03J\t\r\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03J\x10\x11\n\
    )\n\x04\x04\x01\x02\x01\x12\x03M\x02\x18\x1a\x1c\x20The\x20type\x20of\
    \x20this\x20property.\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03M\x02\x0e\
    \n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03M\x0f\x13\n\x0c\n\x05\x04\x01\
    \x02\x01\x03\x12\x03M\x16\x17\n.\n\x04\x04\x01\x02\x02\x12\x03P\x02\x19\
    \x1a!\x20The\x20description\x20of\x20the\x20property\n\n\x0c\n\x05\x04\
    \x01\x02\x02\x05\x12\x03P\x02\x08\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\
    \x03P\t\x14\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03P\x17\x18b\x06proto3\
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
