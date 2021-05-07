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
//! Generated file from `google/type/calendar_period.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_23_0;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CalendarPeriod {
    CALENDAR_PERIOD_UNSPECIFIED = 0,
    DAY = 1,
    WEEK = 2,
    FORTNIGHT = 3,
    MONTH = 4,
    QUARTER = 5,
    HALF = 6,
    YEAR = 7,
}

impl ::protobuf::ProtobufEnum for CalendarPeriod {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CalendarPeriod> {
        match value {
            0 => ::std::option::Option::Some(CalendarPeriod::CALENDAR_PERIOD_UNSPECIFIED),
            1 => ::std::option::Option::Some(CalendarPeriod::DAY),
            2 => ::std::option::Option::Some(CalendarPeriod::WEEK),
            3 => ::std::option::Option::Some(CalendarPeriod::FORTNIGHT),
            4 => ::std::option::Option::Some(CalendarPeriod::MONTH),
            5 => ::std::option::Option::Some(CalendarPeriod::QUARTER),
            6 => ::std::option::Option::Some(CalendarPeriod::HALF),
            7 => ::std::option::Option::Some(CalendarPeriod::YEAR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CalendarPeriod] = &[
            CalendarPeriod::CALENDAR_PERIOD_UNSPECIFIED,
            CalendarPeriod::DAY,
            CalendarPeriod::WEEK,
            CalendarPeriod::FORTNIGHT,
            CalendarPeriod::MONTH,
            CalendarPeriod::QUARTER,
            CalendarPeriod::HALF,
            CalendarPeriod::YEAR,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<CalendarPeriod>("CalendarPeriod", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for CalendarPeriod {
}

impl ::std::default::Default for CalendarPeriod {
    fn default() -> Self {
        CalendarPeriod::CALENDAR_PERIOD_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for CalendarPeriod {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!google/type/calendar_period.proto\x12\x0bgoogle.type*\x7f\n\x0eCalend\
    arPeriod\x12\x1f\n\x1bCALENDAR_PERIOD_UNSPECIFIED\x10\0\x12\x07\n\x03DAY\
    \x10\x01\x12\x08\n\x04WEEK\x10\x02\x12\r\n\tFORTNIGHT\x10\x03\x12\t\n\
    \x05MONTH\x10\x04\x12\x0b\n\x07QUARTER\x10\x05\x12\x08\n\x04HALF\x10\x06\
    \x12\x08\n\x04YEAR\x10\x07Bx\n\x0fcom.google.typeB\x13CalendarPeriodProt\
    oP\x01ZHgoogle.golang.org/genproto/googleapis/type/calendarperiod;calend\
    arperiod\xa2\x02\x03GTPJ\xc6\r\n\x06\x12\x04\x0f\08\x01\n\xbe\x04\n\x01\
    \x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyright\x202019\x20Google\x20LLC.\n\
    \n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\
    \x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\
    \x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20ma\
    y\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\
    \x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20requ\
    ired\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\
    \x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20distri\
    buted\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\
    \x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x14\n\x08\n\x01\x08\x12\
    \x03\x13\0_\n\t\n\x02\x08\x0b\x12\x03\x13\0_\n\x08\n\x01\x08\x12\x03\x14\
    \0\"\n\t\n\x02\x08\n\x12\x03\x14\0\"\n\x08\n\x01\x08\x12\x03\x15\04\n\t\
    \n\x02\x08\x08\x12\x03\x15\04\n\x08\n\x01\x08\x12\x03\x16\0(\n\t\n\x02\
    \x08\x01\x12\x03\x16\0(\n\x08\n\x01\x08\x12\x03\x17\0!\n\t\n\x02\x08$\
    \x12\x03\x17\0!\n\xd6\x01\n\x02\x05\0\x12\x04\x1c\08\x01\x1a\xc9\x01\x20\
    A\x20`CalendarPeriod`\x20represents\x20the\x20abstract\x20concept\x20of\
    \x20a\x20time\x20period\x20that\x20has\n\x20a\x20canonical\x20start.\x20\
    Grammatically,\x20\"the\x20start\x20of\x20the\x20current\n\x20`CalendarP\
    eriod`.\"\x20All\x20calendar\x20times\x20begin\x20at\x20midnight\x20UTC.\
    \n\n\n\n\x03\x05\0\x01\x12\x03\x1c\x05\x13\n1\n\x04\x05\0\x02\0\x12\x03\
    \x1e\x02\"\x1a$\x20Undefined\x20period,\x20raises\x20an\x20error.\n\n\
    \x0c\n\x05\x05\0\x02\0\x01\x12\x03\x1e\x02\x1d\n\x0c\n\x05\x05\0\x02\0\
    \x02\x12\x03\x1e\x20!\n\x15\n\x04\x05\0\x02\x01\x12\x03!\x02\n\x1a\x08\
    \x20A\x20day.\n\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03!\x02\x05\n\x0c\n\
    \x05\x05\0\x02\x01\x02\x12\x03!\x08\t\nq\n\x04\x05\0\x02\x02\x12\x03%\
    \x02\x0b\x1ad\x20A\x20week.\x20Weeks\x20begin\x20on\x20Monday,\x20follow\
    ing\n\x20[ISO\x208601](https://en.wikipedia.org/wiki/ISO_week_date).\n\n\
    \x0c\n\x05\x05\0\x02\x02\x01\x12\x03%\x02\x06\n\x0c\n\x05\x05\0\x02\x02\
    \x02\x12\x03%\t\n\n\xab\x01\n\x04\x05\0\x02\x03\x12\x03*\x02\x10\x1a\x9d\
    \x01\x20A\x20fortnight.\x20The\x20first\x20calendar\x20fortnight\x20of\
    \x20the\x20year\x20begins\x20at\x20the\x20start\n\x20of\x20week\x201\x20\
    according\x20to\n\x20[ISO\x208601](https://en.wikipedia.org/wiki/ISO_wee\
    k_date).\n\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03*\x02\x0b\n\x0c\n\x05\
    \x05\0\x02\x03\x02\x12\x03*\x0e\x0f\n\x17\n\x04\x05\0\x02\x04\x12\x03-\
    \x02\x0c\x1a\n\x20A\x20month.\n\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03-\
    \x02\x07\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03-\n\x0b\n_\n\x04\x05\0\x02\
    \x05\x12\x031\x02\x0e\x1aR\x20A\x20quarter.\x20Quarters\x20start\x20on\
    \x20dates\x201-Jan,\x201-Apr,\x201-Jul,\x20and\x201-Oct\x20of\x20each\n\
    \x20year.\n\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x031\x02\t\n\x0c\n\x05\x05\
    \0\x02\x05\x02\x12\x031\x0c\r\nF\n\x04\x05\0\x02\x06\x12\x034\x02\x0b\
    \x1a9\x20A\x20half-year.\x20Half-years\x20start\x20on\x20dates\x201-Jan\
    \x20and\x201-Jul.\n\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x034\x02\x06\n\x0c\
    \n\x05\x05\0\x02\x06\x02\x12\x034\t\n\n\x16\n\x04\x05\0\x02\x07\x12\x037\
    \x02\x0b\x1a\t\x20A\x20year.\n\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x037\
    \x02\x06\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x037\t\nb\x06proto3\
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