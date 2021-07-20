// This file is generated by rust-protobuf 2.22.1. Do not edit
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
//! Generated file from `app_create.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct CreateAppRequest {
    // message fields
    pub workspace_id: ::std::string::String,
    pub name: ::std::string::String,
    pub desc: ::std::string::String,
    pub color_style: ::protobuf::SingularPtrField<ColorStyle>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CreateAppRequest {
    fn default() -> &'a CreateAppRequest {
        <CreateAppRequest as ::protobuf::Message>::default_instance()
    }
}

impl CreateAppRequest {
    pub fn new() -> CreateAppRequest {
        ::std::default::Default::default()
    }

    // string workspace_id = 1;


    pub fn get_workspace_id(&self) -> &str {
        &self.workspace_id
    }
    pub fn clear_workspace_id(&mut self) {
        self.workspace_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_workspace_id(&mut self, v: ::std::string::String) {
        self.workspace_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_workspace_id(&mut self) -> &mut ::std::string::String {
        &mut self.workspace_id
    }

    // Take field
    pub fn take_workspace_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.workspace_id, ::std::string::String::new())
    }

    // string name = 2;


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

    // string desc = 3;


    pub fn get_desc(&self) -> &str {
        &self.desc
    }
    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::string::String) {
        self.desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc(&mut self) -> &mut ::std::string::String {
        &mut self.desc
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.desc, ::std::string::String::new())
    }

    // .ColorStyle color_style = 4;


    pub fn get_color_style(&self) -> &ColorStyle {
        self.color_style.as_ref().unwrap_or_else(|| <ColorStyle as ::protobuf::Message>::default_instance())
    }
    pub fn clear_color_style(&mut self) {
        self.color_style.clear();
    }

    pub fn has_color_style(&self) -> bool {
        self.color_style.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color_style(&mut self, v: ColorStyle) {
        self.color_style = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color_style(&mut self) -> &mut ColorStyle {
        if self.color_style.is_none() {
            self.color_style.set_default();
        }
        self.color_style.as_mut().unwrap()
    }

    // Take field
    pub fn take_color_style(&mut self) -> ColorStyle {
        self.color_style.take().unwrap_or_else(|| ColorStyle::new())
    }
}

impl ::protobuf::Message for CreateAppRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.color_style {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.workspace_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.desc)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.color_style)?;
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
        if !self.workspace_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.workspace_id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.desc.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.desc);
        }
        if let Some(ref v) = self.color_style.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.workspace_id.is_empty() {
            os.write_string(1, &self.workspace_id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.desc.is_empty() {
            os.write_string(3, &self.desc)?;
        }
        if let Some(ref v) = self.color_style.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> CreateAppRequest {
        CreateAppRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "workspace_id",
                |m: &CreateAppRequest| { &m.workspace_id },
                |m: &mut CreateAppRequest| { &mut m.workspace_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &CreateAppRequest| { &m.name },
                |m: &mut CreateAppRequest| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "desc",
                |m: &CreateAppRequest| { &m.desc },
                |m: &mut CreateAppRequest| { &mut m.desc },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ColorStyle>>(
                "color_style",
                |m: &CreateAppRequest| { &m.color_style },
                |m: &mut CreateAppRequest| { &mut m.color_style },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CreateAppRequest>(
                "CreateAppRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CreateAppRequest {
        static instance: ::protobuf::rt::LazyV2<CreateAppRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CreateAppRequest::new)
    }
}

impl ::protobuf::Clear for CreateAppRequest {
    fn clear(&mut self) {
        self.workspace_id.clear();
        self.name.clear();
        self.desc.clear();
        self.color_style.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateAppRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateAppRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ColorStyle {
    // message fields
    pub theme_color: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ColorStyle {
    fn default() -> &'a ColorStyle {
        <ColorStyle as ::protobuf::Message>::default_instance()
    }
}

impl ColorStyle {
    pub fn new() -> ColorStyle {
        ::std::default::Default::default()
    }

    // string theme_color = 1;


    pub fn get_theme_color(&self) -> &str {
        &self.theme_color
    }
    pub fn clear_theme_color(&mut self) {
        self.theme_color.clear();
    }

    // Param is passed by value, moved
    pub fn set_theme_color(&mut self, v: ::std::string::String) {
        self.theme_color = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_theme_color(&mut self) -> &mut ::std::string::String {
        &mut self.theme_color
    }

    // Take field
    pub fn take_theme_color(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.theme_color, ::std::string::String::new())
    }
}

impl ::protobuf::Message for ColorStyle {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.theme_color)?;
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
        if !self.theme_color.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.theme_color);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.theme_color.is_empty() {
            os.write_string(1, &self.theme_color)?;
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

    fn new() -> ColorStyle {
        ColorStyle::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "theme_color",
                |m: &ColorStyle| { &m.theme_color },
                |m: &mut ColorStyle| { &mut m.theme_color },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ColorStyle>(
                "ColorStyle",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ColorStyle {
        static instance: ::protobuf::rt::LazyV2<ColorStyle> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ColorStyle::new)
    }
}

impl ::protobuf::Clear for ColorStyle {
    fn clear(&mut self) {
        self.theme_color.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ColorStyle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ColorStyle {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct App {
    // message fields
    pub id: ::std::string::String,
    pub workspace_id: ::std::string::String,
    pub name: ::std::string::String,
    pub desc: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a App {
    fn default() -> &'a App {
        <App as ::protobuf::Message>::default_instance()
    }
}

impl App {
    pub fn new() -> App {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string workspace_id = 2;


    pub fn get_workspace_id(&self) -> &str {
        &self.workspace_id
    }
    pub fn clear_workspace_id(&mut self) {
        self.workspace_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_workspace_id(&mut self, v: ::std::string::String) {
        self.workspace_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_workspace_id(&mut self) -> &mut ::std::string::String {
        &mut self.workspace_id
    }

    // Take field
    pub fn take_workspace_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.workspace_id, ::std::string::String::new())
    }

    // string name = 3;


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

    // string desc = 4;


    pub fn get_desc(&self) -> &str {
        &self.desc
    }
    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::string::String) {
        self.desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc(&mut self) -> &mut ::std::string::String {
        &mut self.desc
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.desc, ::std::string::String::new())
    }
}

impl ::protobuf::Message for App {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.workspace_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.desc)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.workspace_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.workspace_id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        }
        if !self.desc.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.desc);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.workspace_id.is_empty() {
            os.write_string(2, &self.workspace_id)?;
        }
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
        }
        if !self.desc.is_empty() {
            os.write_string(4, &self.desc)?;
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

    fn new() -> App {
        App::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &App| { &m.id },
                |m: &mut App| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "workspace_id",
                |m: &App| { &m.workspace_id },
                |m: &mut App| { &mut m.workspace_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &App| { &m.name },
                |m: &mut App| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "desc",
                |m: &App| { &m.desc },
                |m: &mut App| { &mut m.desc },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<App>(
                "App",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static App {
        static instance: ::protobuf::rt::LazyV2<App> = ::protobuf::rt::LazyV2::INIT;
        instance.get(App::new)
    }
}

impl ::protobuf::Clear for App {
    fn clear(&mut self) {
        self.id.clear();
        self.workspace_id.clear();
        self.name.clear();
        self.desc.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for App {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for App {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10app_create.proto\"\x8b\x01\n\x10CreateAppRequest\x12!\n\x0cworkspa\
    ce_id\x18\x01\x20\x01(\tR\x0bworkspaceId\x12\x12\n\x04name\x18\x02\x20\
    \x01(\tR\x04name\x12\x12\n\x04desc\x18\x03\x20\x01(\tR\x04desc\x12,\n\
    \x0bcolor_style\x18\x04\x20\x01(\x0b2\x0b.ColorStyleR\ncolorStyle\"-\n\n\
    ColorStyle\x12\x1f\n\x0btheme_color\x18\x01\x20\x01(\tR\nthemeColor\"`\n\
    \x03App\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12!\n\x0cworkspace_id\
    \x18\x02\x20\x01(\tR\x0bworkspaceId\x12\x12\n\x04name\x18\x03\x20\x01(\t\
    R\x04name\x12\x12\n\x04desc\x18\x04\x20\x01(\tR\x04descJ\xc9\x04\n\x06\
    \x12\x04\0\0\x10\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\
    \x04\x02\0\x07\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x18\n\x0b\n\x04\
    \x04\0\x02\0\x12\x03\x03\x04\x1c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\
    \x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\x17\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\x03\x1a\x1b\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\
    \x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03\x04\x0b\x0f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \x04\x12\x13\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x04\x14\n\x0c\n\x05\
    \x04\0\x02\x02\x05\x12\x03\x05\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x03\x05\x0b\x0f\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05\x12\x13\n\x0b\
    \n\x04\x04\0\x02\x03\x12\x03\x06\x04\x1f\n\x0c\n\x05\x04\0\x02\x03\x06\
    \x12\x03\x06\x04\x0e\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x06\x0f\x1a\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x06\x1d\x1e\n\n\n\x02\x04\x01\x12\
    \x04\x08\0\n\x01\n\n\n\x03\x04\x01\x01\x12\x03\x08\x08\x12\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03\t\x04\x1b\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\t\
    \x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\t\x0b\x16\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\t\x19\x1a\n\n\n\x02\x04\x02\x12\x04\x0b\0\x10\x01\
    \n\n\n\x03\x04\x02\x01\x12\x03\x0b\x08\x0b\n\x0b\n\x04\x04\x02\x02\0\x12\
    \x03\x0c\x04\x12\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x0c\x04\n\n\x0c\n\
    \x05\x04\x02\x02\0\x01\x12\x03\x0c\x0b\r\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03\x0c\x10\x11\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\r\x04\x1c\n\x0c\
    \n\x05\x04\x02\x02\x01\x05\x12\x03\r\x04\n\n\x0c\n\x05\x04\x02\x02\x01\
    \x01\x12\x03\r\x0b\x17\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\r\x1a\x1b\
    \n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x0e\x04\x14\n\x0c\n\x05\x04\x02\x02\
    \x02\x05\x12\x03\x0e\x04\n\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x0e\
    \x0b\x0f\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x0e\x12\x13\n\x0b\n\x04\
    \x04\x02\x02\x03\x12\x03\x0f\x04\x14\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\
    \x03\x0f\x04\n\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03\x0f\x0b\x0f\n\x0c\
    \n\x05\x04\x02\x02\x03\x03\x12\x03\x0f\x12\x13b\x06proto3\
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
