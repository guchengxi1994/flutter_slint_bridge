// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.3.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'bridge_generated.io.dart' if (dart.library.html) 'bridge_generated.web.dart';
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

abstract class Fsb {
  Future<void> sayHello({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kSayHelloConstMeta;

  Future<void> createEventLoop({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kCreateEventLoopConstMeta;

  Future<void> createTrayEventLoop({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kCreateTrayEventLoopConstMeta;

  Future<void> showNotification({EventMessage? message, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kShowNotificationConstMeta;
}

enum DialogType {
  notification,
  confirmDialog,
  warningDialog,
}

class EventMessage {
  final (int, int) alignment;
  final String? title;
  final String? content;
  final DialogType dialogType;
  final int? durationInSec;

  const EventMessage({
    required this.alignment,
    this.title,
    this.content,
    required this.dialogType,
    this.durationInSec,
  });
}
