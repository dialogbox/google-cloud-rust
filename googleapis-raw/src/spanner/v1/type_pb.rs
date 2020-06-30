// This file is generated by rust-protobuf 2.15.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

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
//! Generated file from `google/spanner/v1/type.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_15_1;

#[derive(PartialEq,Clone,Default)]
pub struct Type {
    // message fields
    pub code: TypeCode,
    pub array_element_type: ::protobuf::SingularPtrField<Type>,
    pub struct_type: ::protobuf::SingularPtrField<StructType>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Type {
    fn default() -> &'a Type {
        <Type as ::protobuf::Message>::default_instance()
    }
}

impl Type {
    pub fn new() -> Type {
        ::std::default::Default::default()
    }

    // .google.spanner.v1.TypeCode code = 1;


    pub fn get_code(&self) -> TypeCode {
        self.code
    }
    pub fn clear_code(&mut self) {
        self.code = TypeCode::TYPE_CODE_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: TypeCode) {
        self.code = v;
    }

    // .google.spanner.v1.Type array_element_type = 2;


    pub fn get_array_element_type(&self) -> &Type {
        self.array_element_type.as_ref().unwrap_or_else(|| Type::default_instance())
    }
    pub fn clear_array_element_type(&mut self) {
        self.array_element_type.clear();
    }

    pub fn has_array_element_type(&self) -> bool {
        self.array_element_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_array_element_type(&mut self, v: Type) {
        self.array_element_type = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_array_element_type(&mut self) -> &mut Type {
        if self.array_element_type.is_none() {
            self.array_element_type.set_default();
        }
        self.array_element_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_array_element_type(&mut self) -> Type {
        self.array_element_type.take().unwrap_or_else(|| Type::new())
    }

    // .google.spanner.v1.StructType struct_type = 3;


    pub fn get_struct_type(&self) -> &StructType {
        self.struct_type.as_ref().unwrap_or_else(|| StructType::default_instance())
    }
    pub fn clear_struct_type(&mut self) {
        self.struct_type.clear();
    }

    pub fn has_struct_type(&self) -> bool {
        self.struct_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_struct_type(&mut self, v: StructType) {
        self.struct_type = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_struct_type(&mut self) -> &mut StructType {
        if self.struct_type.is_none() {
            self.struct_type.set_default();
        }
        self.struct_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_struct_type(&mut self) -> StructType {
        self.struct_type.take().unwrap_or_else(|| StructType::new())
    }
}

impl ::protobuf::Message for Type {
    fn is_initialized(&self) -> bool {
        for v in &self.array_element_type {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.struct_type {
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.code, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.array_element_type)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.struct_type)?;
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
        if self.code != TypeCode::TYPE_CODE_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        if let Some(ref v) = self.array_element_type.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.struct_type.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.code != TypeCode::TYPE_CODE_UNSPECIFIED {
            os.write_enum(1, self.code.value())?;
        }
        if let Some(ref v) = self.array_element_type.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.struct_type.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> Type {
        Type::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TypeCode>>(
                    "code",
                    |m: &Type| { &m.code },
                    |m: &mut Type| { &mut m.code },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Type>>(
                    "array_element_type",
                    |m: &Type| { &m.array_element_type },
                    |m: &mut Type| { &mut m.array_element_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StructType>>(
                    "struct_type",
                    |m: &Type| { &m.struct_type },
                    |m: &mut Type| { &mut m.struct_type },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<Type>(
                    "Type",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Type {
        static mut instance: ::protobuf::lazy::Lazy<Type> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Type::new)
        }
    }
}

impl ::protobuf::Clear for Type {
    fn clear(&mut self) {
        self.code = TypeCode::TYPE_CODE_UNSPECIFIED;
        self.array_element_type.clear();
        self.struct_type.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Type {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StructType {
    // message fields
    pub fields: ::protobuf::RepeatedField<StructType_Field>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StructType {
    fn default() -> &'a StructType {
        <StructType as ::protobuf::Message>::default_instance()
    }
}

impl StructType {
    pub fn new() -> StructType {
        ::std::default::Default::default()
    }

    // repeated .google.spanner.v1.StructType.Field fields = 1;


    pub fn get_fields(&self) -> &[StructType_Field] {
        &self.fields
    }
    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: ::protobuf::RepeatedField<StructType_Field>) {
        self.fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields(&mut self) -> &mut ::protobuf::RepeatedField<StructType_Field> {
        &mut self.fields
    }

    // Take field
    pub fn take_fields(&mut self) -> ::protobuf::RepeatedField<StructType_Field> {
        ::std::mem::replace(&mut self.fields, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for StructType {
    fn is_initialized(&self) -> bool {
        for v in &self.fields {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fields)?;
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
        for value in &self.fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.fields {
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

    fn new() -> StructType {
        StructType::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StructType_Field>>(
                    "fields",
                    |m: &StructType| { &m.fields },
                    |m: &mut StructType| { &mut m.fields },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<StructType>(
                    "StructType",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static StructType {
        static mut instance: ::protobuf::lazy::Lazy<StructType> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(StructType::new)
        }
    }
}

impl ::protobuf::Clear for StructType {
    fn clear(&mut self) {
        self.fields.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StructType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StructType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StructType_Field {
    // message fields
    pub name: ::std::string::String,
    pub field_type: ::protobuf::SingularPtrField<Type>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StructType_Field {
    fn default() -> &'a StructType_Field {
        <StructType_Field as ::protobuf::Message>::default_instance()
    }
}

impl StructType_Field {
    pub fn new() -> StructType_Field {
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

    // .google.spanner.v1.Type type = 2;


    pub fn get_field_type(&self) -> &Type {
        self.field_type.as_ref().unwrap_or_else(|| Type::default_instance())
    }
    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Type) {
        self.field_type = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut Type {
        if self.field_type.is_none() {
            self.field_type.set_default();
        }
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> Type {
        self.field_type.take().unwrap_or_else(|| Type::new())
    }
}

impl ::protobuf::Message for StructType_Field {
    fn is_initialized(&self) -> bool {
        for v in &self.field_type {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field_type)?;
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
        if let Some(ref v) = self.field_type.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.field_type.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> StructType_Field {
        StructType_Field::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    |m: &StructType_Field| { &m.name },
                    |m: &mut StructType_Field| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Type>>(
                    "type",
                    |m: &StructType_Field| { &m.field_type },
                    |m: &mut StructType_Field| { &mut m.field_type },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<StructType_Field>(
                    "StructType.Field",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static StructType_Field {
        static mut instance: ::protobuf::lazy::Lazy<StructType_Field> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(StructType_Field::new)
        }
    }
}

impl ::protobuf::Clear for StructType_Field {
    fn clear(&mut self) {
        self.name.clear();
        self.field_type.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StructType_Field {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StructType_Field {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum TypeCode {
    TYPE_CODE_UNSPECIFIED = 0,
    BOOL = 1,
    INT64 = 2,
    FLOAT64 = 3,
    TIMESTAMP = 4,
    DATE = 5,
    STRING = 6,
    BYTES = 7,
    ARRAY = 8,
    STRUCT = 9,
}

impl ::protobuf::ProtobufEnum for TypeCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TypeCode> {
        match value {
            0 => ::std::option::Option::Some(TypeCode::TYPE_CODE_UNSPECIFIED),
            1 => ::std::option::Option::Some(TypeCode::BOOL),
            2 => ::std::option::Option::Some(TypeCode::INT64),
            3 => ::std::option::Option::Some(TypeCode::FLOAT64),
            4 => ::std::option::Option::Some(TypeCode::TIMESTAMP),
            5 => ::std::option::Option::Some(TypeCode::DATE),
            6 => ::std::option::Option::Some(TypeCode::STRING),
            7 => ::std::option::Option::Some(TypeCode::BYTES),
            8 => ::std::option::Option::Some(TypeCode::ARRAY),
            9 => ::std::option::Option::Some(TypeCode::STRUCT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [TypeCode] = &[
            TypeCode::TYPE_CODE_UNSPECIFIED,
            TypeCode::BOOL,
            TypeCode::INT64,
            TypeCode::FLOAT64,
            TypeCode::TIMESTAMP,
            TypeCode::DATE,
            TypeCode::STRING,
            TypeCode::BYTES,
            TypeCode::ARRAY,
            TypeCode::STRUCT,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new_pb_name::<TypeCode>("TypeCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for TypeCode {
}

impl ::std::default::Default for TypeCode {
    fn default() -> Self {
        TypeCode::TYPE_CODE_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for TypeCode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cgoogle/spanner/v1/type.proto\x12\x11google.spanner.v1\x1a\x1cgoogl\
    e/api/annotations.proto\"\xbe\x01\n\x04Type\x12/\n\x04code\x18\x01\x20\
    \x01(\x0e2\x1b.google.spanner.v1.TypeCodeR\x04code\x12E\n\x12array_eleme\
    nt_type\x18\x02\x20\x01(\x0b2\x17.google.spanner.v1.TypeR\x10arrayElemen\
    tType\x12>\n\x0bstruct_type\x18\x03\x20\x01(\x0b2\x1d.google.spanner.v1.\
    StructTypeR\nstructType\"\x93\x01\n\nStructType\x12;\n\x06fields\x18\x01\
    \x20\x03(\x0b2#.google.spanner.v1.StructType.FieldR\x06fields\x1aH\n\x05\
    Field\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12+\n\x04type\x18\
    \x02\x20\x01(\x0b2\x17.google.spanner.v1.TypeR\x04type*\x8e\x01\n\x08Typ\
    eCode\x12\x19\n\x15TYPE_CODE_UNSPECIFIED\x10\0\x12\x08\n\x04BOOL\x10\x01\
    \x12\t\n\x05INT64\x10\x02\x12\x0b\n\x07FLOAT64\x10\x03\x12\r\n\tTIMESTAM\
    P\x10\x04\x12\x08\n\x04DATE\x10\x05\x12\n\n\x06STRING\x10\x06\x12\t\n\
    \x05BYTES\x10\x07\x12\t\n\x05ARRAY\x10\x08\x12\n\n\x06STRUCT\x10\tB\x92\
    \x01\n\x15com.google.spanner.v1B\tTypeProtoP\x01Z8google.golang.org/genp\
    roto/googleapis/spanner/v1;spanner\xaa\x02\x17Google.Cloud.Spanner.V1\
    \xca\x02\x17Google\\Cloud\\Spanner\\V1J\xf9!\n\x06\x12\x04\x0e\0u\x01\n\
    \xbc\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202018\x20Go\
    ogle\x20LLC\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Ver\
    sion\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20th\
    is\x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\
    \x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\
    \x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Un\
    less\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\
    \x20writing,\x20software\n\x20distributed\x20under\x20the\x20License\x20\
    is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20\
    WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20expres\
    s\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0\x1a\n\t\n\x02\x03\0\
    \x12\x03\x12\0&\n\x08\n\x01\x08\x12\x03\x14\04\n\t\n\x02\x08%\x12\x03\
    \x14\04\n\x08\n\x01\x08\x12\x03\x15\0O\n\t\n\x02\x08\x0b\x12\x03\x15\0O\
    \n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\x08\n\x12\x03\x16\0\"\n\x08\n\
    \x01\x08\x12\x03\x17\0*\n\t\n\x02\x08\x08\x12\x03\x17\0*\n\x08\n\x01\x08\
    \x12\x03\x18\0.\n\t\n\x02\x08\x01\x12\x03\x18\0.\n\x08\n\x01\x08\x12\x03\
    \x19\04\n\t\n\x02\x08)\x12\x03\x19\04\n\x84\x01\n\x02\x04\0\x12\x04\x1e\
    \0)\x01\x1ax\x20`Type`\x20indicates\x20the\x20type\x20of\x20a\x20Cloud\
    \x20Spanner\x20value,\x20as\x20might\x20be\x20stored\x20in\x20a\n\x20tab\
    le\x20cell\x20or\x20returned\x20from\x20an\x20SQL\x20query.\n\n\n\n\x03\
    \x04\0\x01\x12\x03\x1e\x08\x0c\nR\n\x04\x04\0\x02\0\x12\x03\x20\x02\x14\
    \x1aE\x20Required.\x20The\x20[TypeCode][google.spanner.v1.TypeCode]\x20f\
    or\x20this\x20type.\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x20\x02\n\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x20\x0b\x0f\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x20\x12\x13\n\xa2\x01\n\x04\x04\0\x02\x01\x12\x03$\x02\x1e\
    \x1a\x94\x01\x20If\x20[code][google.spanner.v1.Type.code]\x20==\x20[ARRA\
    Y][google.spanner.v1.TypeCode.ARRAY],\x20then\x20`array_element_type`\n\
    \x20is\x20the\x20type\x20of\x20the\x20array\x20elements.\n\n\x0c\n\x05\
    \x04\0\x02\x01\x06\x12\x03$\x02\x06\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03$\x07\x19\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03$\x1c\x1d\n\xad\x01\n\
    \x04\x04\0\x02\x02\x12\x03(\x02\x1d\x1a\x9f\x01\x20If\x20[code][google.s\
    panner.v1.Type.code]\x20==\x20[STRUCT][google.spanner.v1.TypeCode.STRUCT\
    ],\x20then\x20`struct_type`\n\x20provides\x20type\x20information\x20for\
    \x20the\x20struct's\x20fields.\n\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03(\
    \x02\x0c\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03(\r\x18\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x03(\x1b\x1c\nd\n\x02\x04\x01\x12\x04,\0C\x01\x1aX\x20`\
    StructType`\x20defines\x20the\x20fields\x20of\x20a\x20[STRUCT][google.sp\
    anner.v1.TypeCode.STRUCT]\x20type.\n\n\n\n\x03\x04\x01\x01\x12\x03,\x08\
    \x12\n@\n\x04\x04\x01\x03\0\x12\x04.\x02:\x03\x1a2\x20Message\x20represe\
    nting\x20a\x20single\x20field\x20of\x20a\x20struct.\n\n\x0c\n\x05\x04\
    \x01\x03\0\x01\x12\x03.\n\x0f\n\x99\x03\n\x06\x04\x01\x03\0\x02\0\x12\
    \x036\x04\x14\x1a\x89\x03\x20The\x20name\x20of\x20the\x20field.\x20For\
    \x20reads,\x20this\x20is\x20the\x20column\x20name.\x20For\n\x20SQL\x20qu\
    eries,\x20it\x20is\x20the\x20column\x20alias\x20(e.g.,\x20`\"Word\"`\x20\
    in\x20the\n\x20query\x20`\"SELECT\x20'hello'\x20AS\x20Word\"`),\x20or\
    \x20the\x20column\x20name\x20(e.g.,\n\x20`\"ColName\"`\x20in\x20the\x20q\
    uery\x20`\"SELECT\x20ColName\x20FROM\x20Table\"`).\x20Some\n\x20columns\
    \x20might\x20have\x20an\x20empty\x20name\x20(e.g.,\x20!\"SELECT\n\x20UPP\
    ER(ColName)\"`).\x20Note\x20that\x20a\x20query\x20result\x20can\x20conta\
    in\n\x20multiple\x20fields\x20with\x20the\x20same\x20name.\n\n\x0e\n\x07\
    \x04\x01\x03\0\x02\0\x05\x12\x036\x04\n\n\x0e\n\x07\x04\x01\x03\0\x02\0\
    \x01\x12\x036\x0b\x0f\n\x0e\n\x07\x04\x01\x03\0\x02\0\x03\x12\x036\x12\
    \x13\n'\n\x06\x04\x01\x03\0\x02\x01\x12\x039\x04\x12\x1a\x18\x20The\x20t\
    ype\x20of\x20the\x20field.\n\n\x0e\n\x07\x04\x01\x03\0\x02\x01\x06\x12\
    \x039\x04\x08\n\x0e\n\x07\x04\x01\x03\0\x02\x01\x01\x12\x039\t\r\n\x0e\n\
    \x07\x04\x01\x03\0\x02\x01\x03\x12\x039\x10\x11\n\x8a\x03\n\x04\x04\x01\
    \x02\0\x12\x03B\x02\x1c\x1a\xfc\x02\x20The\x20list\x20of\x20fields\x20th\
    at\x20make\x20up\x20this\x20struct.\x20Order\x20is\n\x20significant,\x20\
    because\x20values\x20of\x20this\x20struct\x20type\x20are\x20represented\
    \x20as\n\x20lists,\x20where\x20the\x20order\x20of\x20field\x20values\x20\
    matches\x20the\x20order\x20of\n\x20fields\x20in\x20the\x20[StructType][g\
    oogle.spanner.v1.StructType].\x20In\x20turn,\x20the\x20order\x20of\x20fi\
    elds\n\x20matches\x20the\x20order\x20of\x20columns\x20in\x20a\x20read\
    \x20request,\x20or\x20the\x20order\x20of\n\x20fields\x20in\x20the\x20`SE\
    LECT`\x20clause\x20of\x20a\x20query.\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\
    \x03B\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03B\x0b\x10\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03B\x11\x17\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03B\x1a\x1b\n\xd9\x02\n\x02\x05\0\x12\x04L\0u\x01\x1a\xcc\x02\x20`Type\
    Code`\x20is\x20used\x20as\x20part\x20of\x20[Type][google.spanner.v1.Type\
    ]\x20to\n\x20indicate\x20the\x20type\x20of\x20a\x20Cloud\x20Spanner\x20v\
    alue.\n\n\x20Each\x20legal\x20value\x20of\x20a\x20type\x20can\x20be\x20e\
    ncoded\x20to\x20or\x20decoded\x20from\x20a\x20JSON\n\x20value,\x20using\
    \x20the\x20encodings\x20described\x20below.\x20All\x20Cloud\x20Spanner\
    \x20values\x20can\n\x20be\x20`null`,\x20regardless\x20of\x20type;\x20`nu\
    ll`s\x20are\x20always\x20encoded\x20as\x20a\x20JSON\n\x20`null`.\n\n\n\n\
    \x03\x05\0\x01\x12\x03L\x05\r\n\x1d\n\x04\x05\0\x02\0\x12\x03N\x02\x1c\
    \x1a\x10\x20Not\x20specified.\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03N\x02\
    \x17\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03N\x1a\x1b\n1\n\x04\x05\0\x02\x01\
    \x12\x03Q\x02\x0b\x1a$\x20Encoded\x20as\x20JSON\x20`true`\x20or\x20`fals\
    e`.\n\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03Q\x02\x06\n\x0c\n\x05\x05\0\
    \x02\x01\x02\x12\x03Q\t\n\n6\n\x04\x05\0\x02\x02\x12\x03T\x02\x0c\x1a)\
    \x20Encoded\x20as\x20`string`,\x20in\x20decimal\x20format.\n\n\x0c\n\x05\
    \x05\0\x02\x02\x01\x12\x03T\x02\x07\n\x0c\n\x05\x05\0\x02\x02\x02\x12\
    \x03T\n\x0b\n\\\n\x04\x05\0\x02\x03\x12\x03X\x02\x0e\x1aO\x20Encoded\x20\
    as\x20`number`,\x20or\x20the\x20strings\x20`\"NaN\"`,\x20`\"Infinity\"`,\
    \x20or\n\x20`\"-Infinity\"`.\n\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03X\
    \x02\t\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03X\x0c\r\n\xdd\x02\n\x04\x05\
    \0\x02\x04\x12\x03b\x02\x10\x1a\xcf\x02\x20Encoded\x20as\x20`string`\x20\
    in\x20RFC\x203339\x20timestamp\x20format.\x20The\x20time\x20zone\n\x20mu\
    st\x20be\x20present,\x20and\x20must\x20be\x20`\"Z\"`.\n\n\x20If\x20the\
    \x20schema\x20has\x20the\x20column\x20option\n\x20`allow_commit_timestam\
    p=true`,\x20the\x20placeholder\x20string\n\x20`\"spanner.commit_timestam\
    p()\"`\x20can\x20be\x20used\x20to\x20instruct\x20the\x20system\n\x20to\
    \x20insert\x20the\x20commit\x20timestamp\x20associated\x20with\x20the\
    \x20transaction\n\x20commit.\n\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03b\
    \x02\x0b\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03b\x0e\x0f\n;\n\x04\x05\0\
    \x02\x05\x12\x03e\x02\x0b\x1a.\x20Encoded\x20as\x20`string`\x20in\x20RFC\
    \x203339\x20date\x20format.\n\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03e\x02\
    \x06\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03e\t\n\n#\n\x04\x05\0\x02\x06\
    \x12\x03h\x02\r\x1a\x16\x20Encoded\x20as\x20`string`.\n\n\x0c\n\x05\x05\
    \0\x02\x06\x01\x12\x03h\x02\x08\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03h\
    \x0b\x0c\nZ\n\x04\x05\0\x02\x07\x12\x03l\x02\x0c\x1aM\x20Encoded\x20as\
    \x20a\x20base64-encoded\x20`string`,\x20as\x20described\x20in\x20RFC\x20\
    4648,\n\x20section\x204.\n\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03l\x02\
    \x07\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03l\n\x0b\n\x99\x01\n\x04\x05\0\
    \x02\x08\x12\x03p\x02\x0c\x1a\x8b\x01\x20Encoded\x20as\x20`list`,\x20whe\
    re\x20the\x20list\x20elements\x20are\x20represented\n\x20according\x20to\
    \x20[array_element_type][google.spanner.v1.Type.array_element_type].\n\n\
    \x0c\n\x05\x05\0\x02\x08\x01\x12\x03p\x02\x07\n\x0c\n\x05\x05\0\x02\x08\
    \x02\x12\x03p\n\x0b\n\x94\x01\n\x04\x05\0\x02\t\x12\x03t\x02\r\x1a\x86\
    \x01\x20Encoded\x20as\x20`list`,\x20where\x20list\x20element\x20`i`\x20i\
    s\x20represented\x20according\n\x20to\x20[struct_type.fields[i]][google.\
    spanner.v1.StructType.fields].\n\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03t\
    \x02\x08\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03t\x0b\x0cb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

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
