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

class FsbPlatform extends FlutterRustBridgeBase<FsbWire> with FlutterRustBridgeSetupMixin {
  FsbPlatform(FutureOr<WasmModule> dylib) : super(FsbWire(dylib)) {
    setupMixinConstructor();
  }
  Future<void> setup() => inner.init;

// Section: api2wire

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  List<dynamic> api2wire___record__i8_i8((int, int) raw) {
    return [api2wire_i8(raw.$1), api2wire_i8(raw.$2)];
  }

  @protected
  List<dynamic> api2wire_box_autoadd_event_message(EventMessage raw) {
    return api2wire_event_message(raw);
  }

  @protected
  int api2wire_box_autoadd_u8(int raw) {
    return api2wire_u8(raw);
  }

  @protected
  List<dynamic> api2wire_event_message(EventMessage raw) {
    return [
      api2wire___record__i8_i8(raw.alignment),
      api2wire_opt_String(raw.title),
      api2wire_opt_String(raw.content),
      api2wire_dialog_type(raw.dialogType),
      api2wire_opt_box_autoadd_u8(raw.durationInSec)
    ];
  }

  @protected
  String? api2wire_opt_String(String? raw) {
    return raw == null ? null : api2wire_String(raw);
  }

  @protected
  List<dynamic>? api2wire_opt_box_autoadd_event_message(EventMessage? raw) {
    return raw == null ? null : api2wire_box_autoadd_event_message(raw);
  }

  @protected
  int? api2wire_opt_box_autoadd_u8(int? raw) {
    return raw == null ? null : api2wire_box_autoadd_u8(raw);
  }

  @protected
  Uint8List api2wire_uint_8_list(Uint8List raw) {
    return raw;
  }
// Section: finalizer
}

// Section: WASM wire module

@JS('wasm_bindgen')
external FsbWasmModule get wasmModule;

@JS()
@anonymous
class FsbWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);
  external FsbWasmModule bind(dynamic thisArg, String moduleName);
  external dynamic /* void */ wire_say_hello(NativePortType port_);

  external dynamic /* void */ wire_create_event_loop(NativePortType port_);

  external dynamic /* void */ wire_show_notification(NativePortType port_, List<dynamic>? message);
}

// Section: WASM wire connector

class FsbWire extends FlutterRustBridgeWasmWireBase<FsbWasmModule> {
  FsbWire(FutureOr<WasmModule> module) : super(WasmModule.cast<FsbWasmModule>(module));

  void wire_say_hello(NativePortType port_) => wasmModule.wire_say_hello(port_);

  void wire_create_event_loop(NativePortType port_) => wasmModule.wire_create_event_loop(port_);

  void wire_show_notification(NativePortType port_, List<dynamic>? message) =>
      wasmModule.wire_show_notification(port_, message);
}
