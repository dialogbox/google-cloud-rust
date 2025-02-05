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
//! Generated file from `google/type/month.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Month {
    MONTH_UNSPECIFIED = 0,
    JANUARY = 1,
    FEBRUARY = 2,
    MARCH = 3,
    APRIL = 4,
    MAY = 5,
    JUNE = 6,
    JULY = 7,
    AUGUST = 8,
    SEPTEMBER = 9,
    OCTOBER = 10,
    NOVEMBER = 11,
    DECEMBER = 12,
}

impl ::protobuf::ProtobufEnum for Month {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Month> {
        match value {
            0 => ::std::option::Option::Some(Month::MONTH_UNSPECIFIED),
            1 => ::std::option::Option::Some(Month::JANUARY),
            2 => ::std::option::Option::Some(Month::FEBRUARY),
            3 => ::std::option::Option::Some(Month::MARCH),
            4 => ::std::option::Option::Some(Month::APRIL),
            5 => ::std::option::Option::Some(Month::MAY),
            6 => ::std::option::Option::Some(Month::JUNE),
            7 => ::std::option::Option::Some(Month::JULY),
            8 => ::std::option::Option::Some(Month::AUGUST),
            9 => ::std::option::Option::Some(Month::SEPTEMBER),
            10 => ::std::option::Option::Some(Month::OCTOBER),
            11 => ::std::option::Option::Some(Month::NOVEMBER),
            12 => ::std::option::Option::Some(Month::DECEMBER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Month] = &[
            Month::MONTH_UNSPECIFIED,
            Month::JANUARY,
            Month::FEBRUARY,
            Month::MARCH,
            Month::APRIL,
            Month::MAY,
            Month::JUNE,
            Month::JULY,
            Month::AUGUST,
            Month::SEPTEMBER,
            Month::OCTOBER,
            Month::NOVEMBER,
            Month::DECEMBER,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Month>("Month", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Month {
}

impl ::std::default::Default for Month {
    fn default() -> Self {
        Month::MONTH_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for Month {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17google/type/month.proto\x12\x0bgoogle.type*\xb0\x01\n\x05Month\x12\
    \x15\n\x11MONTH_UNSPECIFIED\x10\0\x12\x0b\n\x07JANUARY\x10\x01\x12\x0c\n\
    \x08FEBRUARY\x10\x02\x12\t\n\x05MARCH\x10\x03\x12\t\n\x05APRIL\x10\x04\
    \x12\x07\n\x03MAY\x10\x05\x12\x08\n\x04JUNE\x10\x06\x12\x08\n\x04JULY\
    \x10\x07\x12\n\n\x06AUGUST\x10\x08\x12\r\n\tSEPTEMBER\x10\t\x12\x0b\n\
    \x07OCTOBER\x10\n\x12\x0c\n\x08NOVEMBER\x10\x0b\x12\x0c\n\x08DECEMBER\
    \x10\x0cB]\n\x0fcom.google.typeB\nMonthProtoP\x01Z6google.golang.org/gen\
    proto/googleapis/type/month;month\xa2\x02\x03GTPJ\xd4\x0c\n\x06\x12\x04\
    \x0e\0@\x01\n\xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\
    \x202021\x20Google\x20LLC\n\n\x20Licensed\x20under\x20the\x20Apache\x20L\
    icense,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20no\
    t\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\
    \x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20Lice\
    nse\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-\
    2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\
    \x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0\x14\
    \n\x08\n\x01\x08\x12\x03\x12\0M\n\t\n\x02\x08\x0b\x12\x03\x12\0M\n\x08\n\
    \x01\x08\x12\x03\x13\0\"\n\t\n\x02\x08\n\x12\x03\x13\0\"\n\x08\n\x01\x08\
    \x12\x03\x14\0+\n\t\n\x02\x08\x08\x12\x03\x14\0+\n\x08\n\x01\x08\x12\x03\
    \x15\0(\n\t\n\x02\x08\x01\x12\x03\x15\0(\n\x08\n\x01\x08\x12\x03\x16\0!\
    \n\t\n\x02\x08$\x12\x03\x16\0!\n;\n\x02\x05\0\x12\x04\x19\0@\x01\x1a/\
    \x20Represents\x20a\x20month\x20in\x20the\x20Gregorian\x20calendar.\n\n\
    \n\n\x03\x05\0\x01\x12\x03\x19\x05\n\n%\n\x04\x05\0\x02\0\x12\x03\x1b\
    \x02\x18\x1a\x18\x20The\x20unspecified\x20month.\n\n\x0c\n\x05\x05\0\x02\
    \0\x01\x12\x03\x1b\x02\x13\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x1b\x16\
    \x17\n$\n\x04\x05\0\x02\x01\x12\x03\x1e\x02\x0e\x1a\x17\x20The\x20month\
    \x20of\x20January.\n\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x1e\x02\t\n\
    \x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x1e\x0c\r\n%\n\x04\x05\0\x02\x02\
    \x12\x03!\x02\x0f\x1a\x18\x20The\x20month\x20of\x20February.\n\n\x0c\n\
    \x05\x05\0\x02\x02\x01\x12\x03!\x02\n\n\x0c\n\x05\x05\0\x02\x02\x02\x12\
    \x03!\r\x0e\n\"\n\x04\x05\0\x02\x03\x12\x03$\x02\x0c\x1a\x15\x20The\x20m\
    onth\x20of\x20March.\n\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03$\x02\x07\n\
    \x0c\n\x05\x05\0\x02\x03\x02\x12\x03$\n\x0b\n\"\n\x04\x05\0\x02\x04\x12\
    \x03'\x02\x0c\x1a\x15\x20The\x20month\x20of\x20April.\n\n\x0c\n\x05\x05\
    \0\x02\x04\x01\x12\x03'\x02\x07\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03'\n\
    \x0b\n\x20\n\x04\x05\0\x02\x05\x12\x03*\x02\n\x1a\x13\x20The\x20month\
    \x20of\x20May.\n\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03*\x02\x05\n\x0c\n\
    \x05\x05\0\x02\x05\x02\x12\x03*\x08\t\n!\n\x04\x05\0\x02\x06\x12\x03-\
    \x02\x0b\x1a\x14\x20The\x20month\x20of\x20June.\n\n\x0c\n\x05\x05\0\x02\
    \x06\x01\x12\x03-\x02\x06\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03-\t\n\n!\
    \n\x04\x05\0\x02\x07\x12\x030\x02\x0b\x1a\x14\x20The\x20month\x20of\x20J\
    uly.\n\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x030\x02\x06\n\x0c\n\x05\x05\0\
    \x02\x07\x02\x12\x030\t\n\n#\n\x04\x05\0\x02\x08\x12\x033\x02\r\x1a\x16\
    \x20The\x20month\x20of\x20August.\n\n\x0c\n\x05\x05\0\x02\x08\x01\x12\
    \x033\x02\x08\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x033\x0b\x0c\n&\n\x04\
    \x05\0\x02\t\x12\x036\x02\x10\x1a\x19\x20The\x20month\x20of\x20September\
    .\n\n\x0c\n\x05\x05\0\x02\t\x01\x12\x036\x02\x0b\n\x0c\n\x05\x05\0\x02\t\
    \x02\x12\x036\x0e\x0f\n$\n\x04\x05\0\x02\n\x12\x039\x02\x0f\x1a\x17\x20T\
    he\x20month\x20of\x20October.\n\n\x0c\n\x05\x05\0\x02\n\x01\x12\x039\x02\
    \t\n\x0c\n\x05\x05\0\x02\n\x02\x12\x039\x0c\x0e\n%\n\x04\x05\0\x02\x0b\
    \x12\x03<\x02\x10\x1a\x18\x20The\x20month\x20of\x20November.\n\n\x0c\n\
    \x05\x05\0\x02\x0b\x01\x12\x03<\x02\n\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\
    \x03<\r\x0f\n%\n\x04\x05\0\x02\x0c\x12\x03?\x02\x10\x1a\x18\x20The\x20mo\
    nth\x20of\x20December.\n\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\x03?\x02\n\n\
    \x0c\n\x05\x05\0\x02\x0c\x02\x12\x03?\r\x0fb\x06proto3\
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
