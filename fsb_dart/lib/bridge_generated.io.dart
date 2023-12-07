// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.3.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';
import 'dart:ffi' as ffi;

class FsbPlatform extends FlutterRustBridgeBase<FsbWire> {
  FsbPlatform(ffi.DynamicLibrary dylib) : super(FsbWire(dylib));

// Section: api2wire

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_EventMessage> api2wire_box_autoadd_event_message(EventMessage raw) {
    final ptr = inner.new_box_autoadd_event_message_0();
    _api_fill_to_wire_event_message(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<ffi.Uint8> api2wire_box_autoadd_u8(int raw) {
    return inner.new_box_autoadd_u8_0(api2wire_u8(raw));
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_opt_String(String? raw) {
    return raw == null ? ffi.nullptr : api2wire_String(raw);
  }

  @protected
  ffi.Pointer<wire_EventMessage> api2wire_opt_box_autoadd_event_message(EventMessage? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_event_message(raw);
  }

  @protected
  ffi.Pointer<ffi.Uint8> api2wire_opt_box_autoadd_u8(int? raw) {
    return raw == null ? ffi.nullptr : api2wire_box_autoadd_u8(raw);
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: finalizer

// Section: api_fill_to_wire

  void _api_fill_to_wire___record__i8_i8((int, int) apiObj, wire___record__i8_i8 wireObj) {
    wireObj.field0 = api2wire_i8(apiObj.$1);
    wireObj.field1 = api2wire_i8(apiObj.$2);
  }

  void _api_fill_to_wire_box_autoadd_event_message(EventMessage apiObj, ffi.Pointer<wire_EventMessage> wireObj) {
    _api_fill_to_wire_event_message(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_event_message(EventMessage apiObj, wire_EventMessage wireObj) {
    _api_fill_to_wire___record__i8_i8(apiObj.alignment, wireObj.alignment);
    wireObj.title = api2wire_opt_String(apiObj.title);
    wireObj.content = api2wire_opt_String(apiObj.content);
    wireObj.dialog_type = api2wire_dialog_type(apiObj.dialogType);
    wireObj.duration_in_sec = api2wire_opt_box_autoadd_u8(apiObj.durationInSec);
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class FsbWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  FsbWire(ffi.DynamicLibrary dynamicLibrary) : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  FsbWire.fromLookup(ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) lookup) : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>('store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr.asFunction<void Function(DartPostCObjectFnType)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr = _lookup<ffi.NativeFunction<ffi.Handle Function(ffi.UintPtr)>>('get_dart_object');
  late final _get_dart_object = _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.UintPtr)>>('drop_dart_object');
  late final _drop_dart_object = _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr = _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>('new_dart_opaque');
  late final _new_dart_opaque = _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>('init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr.asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_say_hello(
    int port_,
  ) {
    return _wire_say_hello(
      port_,
    );
  }

  late final _wire_say_helloPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_say_hello');
  late final _wire_say_hello = _wire_say_helloPtr.asFunction<void Function(int)>();

  void wire_create_event_loop(
    int port_,
  ) {
    return _wire_create_event_loop(
      port_,
    );
  }

  late final _wire_create_event_loopPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_create_event_loop');
  late final _wire_create_event_loop = _wire_create_event_loopPtr.asFunction<void Function(int)>();

  void wire_create_tray_event_loop(
    int port_,
  ) {
    return _wire_create_tray_event_loop(
      port_,
    );
  }

  late final _wire_create_tray_event_loopPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_create_tray_event_loop');
  late final _wire_create_tray_event_loop = _wire_create_tray_event_loopPtr.asFunction<void Function(int)>();

  void wire_show_notification(
    int port_,
    ffi.Pointer<wire_EventMessage> message,
  ) {
    return _wire_show_notification(
      port_,
      message,
    );
  }

  late final _wire_show_notificationPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_EventMessage>)>>(
          'wire_show_notification');
  late final _wire_show_notification =
      _wire_show_notificationPtr.asFunction<void Function(int, ffi.Pointer<wire_EventMessage>)>();

  ffi.Pointer<wire_EventMessage> new_box_autoadd_event_message_0() {
    return _new_box_autoadd_event_message_0();
  }

  late final _new_box_autoadd_event_message_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_EventMessage> Function()>>('new_box_autoadd_event_message_0');
  late final _new_box_autoadd_event_message_0 =
      _new_box_autoadd_event_message_0Ptr.asFunction<ffi.Pointer<wire_EventMessage> Function()>();

  ffi.Pointer<ffi.Uint8> new_box_autoadd_u8_0(
    int value,
  ) {
    return _new_box_autoadd_u8_0(
      value,
    );
  }

  late final _new_box_autoadd_u8_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<ffi.Uint8> Function(ffi.Uint8)>>('new_box_autoadd_u8_0');
  late final _new_box_autoadd_u8_0 = _new_box_autoadd_u8_0Ptr.asFunction<ffi.Pointer<ffi.Uint8> Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_uint_8_list> Function(ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr.asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>('free_WireSyncReturn');
  late final _free_WireSyncReturn = _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

final class _Dart_Handle extends ffi.Opaque {}

final class wire___record__i8_i8 extends ffi.Struct {
  @ffi.Int8()
  external int field0;

  @ffi.Int8()
  external int field1;
}

final class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

final class wire_EventMessage extends ffi.Struct {
  external wire___record__i8_i8 alignment;

  external ffi.Pointer<wire_uint_8_list> title;

  external ffi.Pointer<wire_uint_8_list> content;

  @ffi.Int32()
  external int dialog_type;

  external ffi.Pointer<ffi.Uint8> duration_in_sec;
}

typedef DartPostCObjectFnType
    = ffi.Pointer<ffi.NativeFunction<ffi.Bool Function(DartPort port_id, ffi.Pointer<ffi.Void> message)>>;
typedef DartPort = ffi.Int64;
