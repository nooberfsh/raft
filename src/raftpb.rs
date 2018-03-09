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
pub struct Entry {
    // message fields
    pub term: u64,
    pub index: u64,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Entry {}

impl Entry {
    pub fn new() -> Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Entry {
        static mut instance: ::protobuf::lazy::Lazy<Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Entry,
        };
        unsafe {
            instance.get(Entry::new)
        }
    }

    // uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }

    // uint64 index = 2;

    pub fn clear_index(&mut self) {
        self.index = 0;
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = v;
    }

    pub fn get_index(&self) -> u64 {
        self.index
    }

    fn get_index_for_reflect(&self) -> &u64 {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut u64 {
        &mut self.index
    }

    // bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for Entry {
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
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.index = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(1, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.index != 0 {
            my_size += ::protobuf::rt::value_size(2, self.index, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.term != 0 {
            os.write_uint64(1, self.term)?;
        }
        if self.index != 0 {
            os.write_uint64(2, self.index)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
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

impl ::protobuf::MessageStatic for Entry {
    fn new() -> Entry {
        Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    Entry::get_term_for_reflect,
                    Entry::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    Entry::get_index_for_reflect,
                    Entry::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Entry::get_data_for_reflect,
                    Entry::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Entry>(
                    "Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Entry {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_index();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AppendEntriesRequest {
    // message fields
    pub term: u64,
    pub leader_id: ::std::string::String,
    pub prev_log_index: u64,
    pub prev_log_term: u64,
    pub entries: ::protobuf::RepeatedField<Entry>,
    pub leader_commit: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppendEntriesRequest {}

impl AppendEntriesRequest {
    pub fn new() -> AppendEntriesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendEntriesRequest {
        static mut instance: ::protobuf::lazy::Lazy<AppendEntriesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendEntriesRequest,
        };
        unsafe {
            instance.get(AppendEntriesRequest::new)
        }
    }

    // uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }

    // string leader_id = 2;

    pub fn clear_leader_id(&mut self) {
        self.leader_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_leader_id(&mut self, v: ::std::string::String) {
        self.leader_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leader_id(&mut self) -> &mut ::std::string::String {
        &mut self.leader_id
    }

    // Take field
    pub fn take_leader_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.leader_id, ::std::string::String::new())
    }

    pub fn get_leader_id(&self) -> &str {
        &self.leader_id
    }

    fn get_leader_id_for_reflect(&self) -> &::std::string::String {
        &self.leader_id
    }

    fn mut_leader_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.leader_id
    }

    // uint64 prev_log_index = 3;

    pub fn clear_prev_log_index(&mut self) {
        self.prev_log_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_prev_log_index(&mut self, v: u64) {
        self.prev_log_index = v;
    }

    pub fn get_prev_log_index(&self) -> u64 {
        self.prev_log_index
    }

    fn get_prev_log_index_for_reflect(&self) -> &u64 {
        &self.prev_log_index
    }

    fn mut_prev_log_index_for_reflect(&mut self) -> &mut u64 {
        &mut self.prev_log_index
    }

    // uint64 prev_log_term = 4;

    pub fn clear_prev_log_term(&mut self) {
        self.prev_log_term = 0;
    }

    // Param is passed by value, moved
    pub fn set_prev_log_term(&mut self, v: u64) {
        self.prev_log_term = v;
    }

    pub fn get_prev_log_term(&self) -> u64 {
        self.prev_log_term
    }

    fn get_prev_log_term_for_reflect(&self) -> &u64 {
        &self.prev_log_term
    }

    fn mut_prev_log_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.prev_log_term
    }

    // repeated .raftpb.Entry entries = 5;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<Entry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<Entry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<Entry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[Entry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<Entry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Entry> {
        &mut self.entries
    }

    // uint64 leader_commit = 6;

    pub fn clear_leader_commit(&mut self) {
        self.leader_commit = 0;
    }

    // Param is passed by value, moved
    pub fn set_leader_commit(&mut self, v: u64) {
        self.leader_commit = v;
    }

    pub fn get_leader_commit(&self) -> u64 {
        self.leader_commit
    }

    fn get_leader_commit_for_reflect(&self) -> &u64 {
        &self.leader_commit
    }

    fn mut_leader_commit_for_reflect(&mut self) -> &mut u64 {
        &mut self.leader_commit
    }
}

impl ::protobuf::Message for AppendEntriesRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.leader_id)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.prev_log_index = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.prev_log_term = tmp;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.leader_commit = tmp;
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
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(1, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.leader_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.leader_id);
        }
        if self.prev_log_index != 0 {
            my_size += ::protobuf::rt::value_size(3, self.prev_log_index, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.prev_log_term != 0 {
            my_size += ::protobuf::rt::value_size(4, self.prev_log_term, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.leader_commit != 0 {
            my_size += ::protobuf::rt::value_size(6, self.leader_commit, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.term != 0 {
            os.write_uint64(1, self.term)?;
        }
        if !self.leader_id.is_empty() {
            os.write_string(2, &self.leader_id)?;
        }
        if self.prev_log_index != 0 {
            os.write_uint64(3, self.prev_log_index)?;
        }
        if self.prev_log_term != 0 {
            os.write_uint64(4, self.prev_log_term)?;
        }
        for v in &self.entries {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.leader_commit != 0 {
            os.write_uint64(6, self.leader_commit)?;
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

impl ::protobuf::MessageStatic for AppendEntriesRequest {
    fn new() -> AppendEntriesRequest {
        AppendEntriesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendEntriesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    AppendEntriesRequest::get_term_for_reflect,
                    AppendEntriesRequest::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "leader_id",
                    AppendEntriesRequest::get_leader_id_for_reflect,
                    AppendEntriesRequest::mut_leader_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "prev_log_index",
                    AppendEntriesRequest::get_prev_log_index_for_reflect,
                    AppendEntriesRequest::mut_prev_log_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "prev_log_term",
                    AppendEntriesRequest::get_prev_log_term_for_reflect,
                    AppendEntriesRequest::mut_prev_log_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Entry>>(
                    "entries",
                    AppendEntriesRequest::get_entries_for_reflect,
                    AppendEntriesRequest::mut_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "leader_commit",
                    AppendEntriesRequest::get_leader_commit_for_reflect,
                    AppendEntriesRequest::mut_leader_commit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendEntriesRequest>(
                    "AppendEntriesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendEntriesRequest {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_leader_id();
        self.clear_prev_log_index();
        self.clear_prev_log_term();
        self.clear_entries();
        self.clear_leader_commit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AppendEntriesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AppendEntriesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AppendEntriesResponse {
    // message fields
    pub term: u64,
    pub success: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppendEntriesResponse {}

impl AppendEntriesResponse {
    pub fn new() -> AppendEntriesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendEntriesResponse {
        static mut instance: ::protobuf::lazy::Lazy<AppendEntriesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendEntriesResponse,
        };
        unsafe {
            instance.get(AppendEntriesResponse::new)
        }
    }

    // uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }

    // bool success = 2;

    pub fn clear_success(&mut self) {
        self.success = false;
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = v;
    }

    pub fn get_success(&self) -> bool {
        self.success
    }

    fn get_success_for_reflect(&self) -> &bool {
        &self.success
    }

    fn mut_success_for_reflect(&mut self) -> &mut bool {
        &mut self.success
    }
}

impl ::protobuf::Message for AppendEntriesResponse {
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
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.success = tmp;
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
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(1, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.success != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.term != 0 {
            os.write_uint64(1, self.term)?;
        }
        if self.success != false {
            os.write_bool(2, self.success)?;
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

impl ::protobuf::MessageStatic for AppendEntriesResponse {
    fn new() -> AppendEntriesResponse {
        AppendEntriesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendEntriesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    AppendEntriesResponse::get_term_for_reflect,
                    AppendEntriesResponse::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "success",
                    AppendEntriesResponse::get_success_for_reflect,
                    AppendEntriesResponse::mut_success_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendEntriesResponse>(
                    "AppendEntriesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendEntriesResponse {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AppendEntriesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AppendEntriesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestVoteRequest {
    // message fields
    pub term: u64,
    pub candidate_id: u64,
    pub last_log_index: u64,
    pub last_log_term: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestVoteRequest {}

impl RequestVoteRequest {
    pub fn new() -> RequestVoteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestVoteRequest {
        static mut instance: ::protobuf::lazy::Lazy<RequestVoteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestVoteRequest,
        };
        unsafe {
            instance.get(RequestVoteRequest::new)
        }
    }

    // uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }

    // uint64 candidate_id = 2;

    pub fn clear_candidate_id(&mut self) {
        self.candidate_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_candidate_id(&mut self, v: u64) {
        self.candidate_id = v;
    }

    pub fn get_candidate_id(&self) -> u64 {
        self.candidate_id
    }

    fn get_candidate_id_for_reflect(&self) -> &u64 {
        &self.candidate_id
    }

    fn mut_candidate_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.candidate_id
    }

    // uint64 last_log_index = 3;

    pub fn clear_last_log_index(&mut self) {
        self.last_log_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_last_log_index(&mut self, v: u64) {
        self.last_log_index = v;
    }

    pub fn get_last_log_index(&self) -> u64 {
        self.last_log_index
    }

    fn get_last_log_index_for_reflect(&self) -> &u64 {
        &self.last_log_index
    }

    fn mut_last_log_index_for_reflect(&mut self) -> &mut u64 {
        &mut self.last_log_index
    }

    // uint64 last_log_term = 4;

    pub fn clear_last_log_term(&mut self) {
        self.last_log_term = 0;
    }

    // Param is passed by value, moved
    pub fn set_last_log_term(&mut self, v: u64) {
        self.last_log_term = v;
    }

    pub fn get_last_log_term(&self) -> u64 {
        self.last_log_term
    }

    fn get_last_log_term_for_reflect(&self) -> &u64 {
        &self.last_log_term
    }

    fn mut_last_log_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.last_log_term
    }
}

impl ::protobuf::Message for RequestVoteRequest {
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
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.candidate_id = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.last_log_index = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.last_log_term = tmp;
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
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(1, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.candidate_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.candidate_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.last_log_index != 0 {
            my_size += ::protobuf::rt::value_size(3, self.last_log_index, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.last_log_term != 0 {
            my_size += ::protobuf::rt::value_size(4, self.last_log_term, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.term != 0 {
            os.write_uint64(1, self.term)?;
        }
        if self.candidate_id != 0 {
            os.write_uint64(2, self.candidate_id)?;
        }
        if self.last_log_index != 0 {
            os.write_uint64(3, self.last_log_index)?;
        }
        if self.last_log_term != 0 {
            os.write_uint64(4, self.last_log_term)?;
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

impl ::protobuf::MessageStatic for RequestVoteRequest {
    fn new() -> RequestVoteRequest {
        RequestVoteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestVoteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    RequestVoteRequest::get_term_for_reflect,
                    RequestVoteRequest::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "candidate_id",
                    RequestVoteRequest::get_candidate_id_for_reflect,
                    RequestVoteRequest::mut_candidate_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "last_log_index",
                    RequestVoteRequest::get_last_log_index_for_reflect,
                    RequestVoteRequest::mut_last_log_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "last_log_term",
                    RequestVoteRequest::get_last_log_term_for_reflect,
                    RequestVoteRequest::mut_last_log_term_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestVoteRequest>(
                    "RequestVoteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestVoteRequest {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_candidate_id();
        self.clear_last_log_index();
        self.clear_last_log_term();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestVoteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestVoteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestVoteResponse {
    // message fields
    pub term: u64,
    pub vote_granted: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestVoteResponse {}

impl RequestVoteResponse {
    pub fn new() -> RequestVoteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestVoteResponse {
        static mut instance: ::protobuf::lazy::Lazy<RequestVoteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestVoteResponse,
        };
        unsafe {
            instance.get(RequestVoteResponse::new)
        }
    }

    // uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }

    // bool vote_granted = 2;

    pub fn clear_vote_granted(&mut self) {
        self.vote_granted = false;
    }

    // Param is passed by value, moved
    pub fn set_vote_granted(&mut self, v: bool) {
        self.vote_granted = v;
    }

    pub fn get_vote_granted(&self) -> bool {
        self.vote_granted
    }

    fn get_vote_granted_for_reflect(&self) -> &bool {
        &self.vote_granted
    }

    fn mut_vote_granted_for_reflect(&mut self) -> &mut bool {
        &mut self.vote_granted
    }
}

impl ::protobuf::Message for RequestVoteResponse {
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
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.vote_granted = tmp;
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
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(1, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.vote_granted != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.term != 0 {
            os.write_uint64(1, self.term)?;
        }
        if self.vote_granted != false {
            os.write_bool(2, self.vote_granted)?;
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

impl ::protobuf::MessageStatic for RequestVoteResponse {
    fn new() -> RequestVoteResponse {
        RequestVoteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestVoteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    RequestVoteResponse::get_term_for_reflect,
                    RequestVoteResponse::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "vote_granted",
                    RequestVoteResponse::get_vote_granted_for_reflect,
                    RequestVoteResponse::mut_vote_granted_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestVoteResponse>(
                    "RequestVoteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestVoteResponse {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_vote_granted();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestVoteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestVoteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12proto/raftpb.proto\x12\x06raftpb\"E\n\x05Entry\x12\x12\n\x04term\
    \x18\x01\x20\x01(\x04R\x04term\x12\x14\n\x05index\x18\x02\x20\x01(\x04R\
    \x05index\x12\x12\n\x04data\x18\x03\x20\x01(\x0cR\x04data\"\xdf\x01\n\
    \x14AppendEntriesRequest\x12\x12\n\x04term\x18\x01\x20\x01(\x04R\x04term\
    \x12\x1b\n\tleader_id\x18\x02\x20\x01(\tR\x08leaderId\x12$\n\x0eprev_log\
    _index\x18\x03\x20\x01(\x04R\x0cprevLogIndex\x12\"\n\rprev_log_term\x18\
    \x04\x20\x01(\x04R\x0bprevLogTerm\x12'\n\x07entries\x18\x05\x20\x03(\x0b\
    2\r.raftpb.EntryR\x07entries\x12#\n\rleader_commit\x18\x06\x20\x01(\x04R\
    \x0cleaderCommit\"E\n\x15AppendEntriesResponse\x12\x12\n\x04term\x18\x01\
    \x20\x01(\x04R\x04term\x12\x18\n\x07success\x18\x02\x20\x01(\x08R\x07suc\
    cess\"\x95\x01\n\x12RequestVoteRequest\x12\x12\n\x04term\x18\x01\x20\x01\
    (\x04R\x04term\x12!\n\x0ccandidate_id\x18\x02\x20\x01(\x04R\x0bcandidate\
    Id\x12$\n\x0elast_log_index\x18\x03\x20\x01(\x04R\x0clastLogIndex\x12\"\
    \n\rlast_log_term\x18\x04\x20\x01(\x04R\x0blastLogTerm\"L\n\x13RequestVo\
    teResponse\x12\x12\n\x04term\x18\x01\x20\x01(\x04R\x04term\x12!\n\x0cvot\
    e_granted\x18\x02\x20\x01(\x08R\x0bvoteGranted2\xa0\x01\n\x04Raft\x12N\n\
    \rAppendEntries\x12\x1c.raftpb.AppendEntriesRequest\x1a\x1d.raftpb.Appen\
    dEntriesResponse\"\0\x12H\n\x0bRequestVote\x12\x1a.raftpb.RequestVoteReq\
    uest\x1a\x1b.raftpb.RequestVoteResponse\"\0J\xbf\x0b\n\x06\x12\x04\0\0'\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0e\
    \n\n\n\x02\x04\0\x12\x04\x04\0\x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\
    \x08\r\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x08\x18\n\r\n\x05\x04\0\x02\0\
    \x04\x12\x04\x05\x08\x04\x0f\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x05\x08\
    \x0e\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\x0f\x13\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\x05\x16\x17\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x06\x08\
    \x19\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x06\x08\x05\x18\n\x0c\n\x05\x04\
    \0\x02\x01\x05\x12\x03\x06\x08\x0e\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x06\x0f\x14\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x06\x17\x18\n\x0b\n\
    \x04\x04\0\x02\x02\x12\x03\x07\x08\x17\n\r\n\x05\x04\0\x02\x02\x04\x12\
    \x04\x07\x08\x06\x19\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x07\x08\r\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x07\x0e\x12\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x07\x15\x16\n\n\n\x02\x04\x01\x12\x04\n\0\x11\x01\n\n\n\
    \x03\x04\x01\x01\x12\x03\n\x08\x1c\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\
    \x08\x18\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x0b\x08\n\x1e\n\x0c\n\x05\
    \x04\x01\x02\0\x05\x12\x03\x0b\x08\x0e\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03\x0b\x0f\x13\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0b\x16\x17\n\x0b\
    \n\x04\x04\x01\x02\x01\x12\x03\x0c\x08\x1d\n\r\n\x05\x04\x01\x02\x01\x04\
    \x12\x04\x0c\x08\x0b\x18\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x0c\x08\
    \x0e\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0c\x0f\x18\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03\x0c\x1b\x1c\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\
    \r\x08\"\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\r\x08\x0c\x1d\n\x0c\n\x05\
    \x04\x01\x02\x02\x05\x12\x03\r\x08\x0e\n\x0c\n\x05\x04\x01\x02\x02\x01\
    \x12\x03\r\x0f\x1d\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\r\x20!\n\x0b\
    \n\x04\x04\x01\x02\x03\x12\x03\x0e\x08!\n\r\n\x05\x04\x01\x02\x03\x04\
    \x12\x04\x0e\x08\r\"\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x0e\x08\x0e\
    \n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x0e\x0f\x1c\n\x0c\n\x05\x04\x01\
    \x02\x03\x03\x12\x03\x0e\x1f\x20\n\x0b\n\x04\x04\x01\x02\x04\x12\x03\x0f\
    \x08#\n\x0c\n\x05\x04\x01\x02\x04\x04\x12\x03\x0f\x08\x10\n\x0c\n\x05\
    \x04\x01\x02\x04\x06\x12\x03\x0f\x11\x16\n\x0c\n\x05\x04\x01\x02\x04\x01\
    \x12\x03\x0f\x17\x1e\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x0f!\"\n\
    \x0b\n\x04\x04\x01\x02\x05\x12\x03\x10\x08!\n\r\n\x05\x04\x01\x02\x05\
    \x04\x12\x04\x10\x08\x0f#\n\x0c\n\x05\x04\x01\x02\x05\x05\x12\x03\x10\
    \x08\x0e\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03\x10\x0f\x1c\n\x0c\n\x05\
    \x04\x01\x02\x05\x03\x12\x03\x10\x1f\x20\n\n\n\x02\x04\x02\x12\x04\x13\0\
    \x16\x01\n\n\n\x03\x04\x02\x01\x12\x03\x13\x08\x1d\n\x0b\n\x04\x04\x02\
    \x02\0\x12\x03\x14\x08\x18\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x14\x08\
    \x13\x1f\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x14\x08\x0e\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03\x14\x0f\x13\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x03\x14\x16\x17\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x15\x08\x19\n\r\n\
    \x05\x04\x02\x02\x01\x04\x12\x04\x15\x08\x14\x18\n\x0c\n\x05\x04\x02\x02\
    \x01\x05\x12\x03\x15\x08\x0c\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x15\
    \r\x14\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x15\x17\x18\n\n\n\x02\x04\
    \x03\x12\x04\x18\0\x1d\x01\n\n\n\x03\x04\x03\x01\x12\x03\x18\x08\x1a\n\
    \x0b\n\x04\x04\x03\x02\0\x12\x03\x19\x08\x18\n\r\n\x05\x04\x03\x02\0\x04\
    \x12\x04\x19\x08\x18\x1c\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x19\x08\
    \x0e\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x19\x0f\x13\n\x0c\n\x05\x04\
    \x03\x02\0\x03\x12\x03\x19\x16\x17\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\
    \x1a\x08\x20\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04\x1a\x08\x19\x18\n\x0c\
    \n\x05\x04\x03\x02\x01\x05\x12\x03\x1a\x08\x0e\n\x0c\n\x05\x04\x03\x02\
    \x01\x01\x12\x03\x1a\x0f\x1b\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\x1a\
    \x1e\x1f\n\x0b\n\x04\x04\x03\x02\x02\x12\x03\x1b\x08\"\n\r\n\x05\x04\x03\
    \x02\x02\x04\x12\x04\x1b\x08\x1a\x20\n\x0c\n\x05\x04\x03\x02\x02\x05\x12\
    \x03\x1b\x08\x0e\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03\x1b\x0f\x1d\n\
    \x0c\n\x05\x04\x03\x02\x02\x03\x12\x03\x1b\x20!\n\x0b\n\x04\x04\x03\x02\
    \x03\x12\x03\x1c\x08!\n\r\n\x05\x04\x03\x02\x03\x04\x12\x04\x1c\x08\x1b\
    \"\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03\x1c\x08\x0e\n\x0c\n\x05\x04\
    \x03\x02\x03\x01\x12\x03\x1c\x0f\x1c\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\
    \x03\x1c\x1f\x20\n\n\n\x02\x04\x04\x12\x04\x1f\0\"\x01\n\n\n\x03\x04\x04\
    \x01\x12\x03\x1f\x08\x1b\n\x0b\n\x04\x04\x04\x02\0\x12\x03\x20\x08\x18\n\
    \r\n\x05\x04\x04\x02\0\x04\x12\x04\x20\x08\x1f\x1d\n\x0c\n\x05\x04\x04\
    \x02\0\x05\x12\x03\x20\x08\x0e\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x20\
    \x0f\x13\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03\x20\x16\x17\n\x0b\n\x04\
    \x04\x04\x02\x01\x12\x03!\x08\x1e\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04!\
    \x08\x20\x18\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03!\x08\x0c\n\x0c\n\
    \x05\x04\x04\x02\x01\x01\x12\x03!\r\x19\n\x0c\n\x05\x04\x04\x02\x01\x03\
    \x12\x03!\x1c\x1d\n\n\n\x02\x06\0\x12\x04$\0'\x01\n\n\n\x03\x06\0\x01\
    \x12\x03$\x08\x0c\n\x0b\n\x04\x06\0\x02\0\x12\x03%\x08R\n\x0c\n\x05\x06\
    \0\x02\0\x01\x12\x03%\x0c\x19\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03%\x1a.\
    \n\x0c\n\x05\x06\0\x02\0\x03\x12\x03%9N\n\x0b\n\x04\x06\0\x02\x01\x12\
    \x03&\x08M\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03&\x0c\x17\n\x0c\n\x05\
    \x06\0\x02\x01\x02\x12\x03&\x18*\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03&5\
    Hb\x06proto3\
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
