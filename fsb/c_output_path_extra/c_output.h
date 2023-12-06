#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire___record__i8_i8 {
  int8_t field0;
  int8_t field1;
} wire___record__i8_i8;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_EventMessage {
  struct wire___record__i8_i8 alignment;
  struct wire_uint_8_list *title;
  struct wire_uint_8_list *content;
  int32_t dialog_type;
} wire_EventMessage;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_say_hello(int64_t port_);

void wire_create_event_loop(int64_t port_);

void wire_show_auto_close_dialog(int64_t port_, struct wire_EventMessage *message);

struct wire_EventMessage *new_box_autoadd_event_message_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_say_hello);
    dummy_var ^= ((int64_t) (void*) wire_create_event_loop);
    dummy_var ^= ((int64_t) (void*) wire_show_auto_close_dialog);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_event_message_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
