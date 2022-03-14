// Import dart:ffi.
import 'dart:ffi' as ffi;
import 'package:ffi/ffi.dart';

class ThumbnailerBindings {
  final ffi.DynamicLibrary _dylib;

  ThumbnailerBindings(this._dylib);

  void save_image(String video_path, String thumbnail_path) {
    // Get a reference to the C function, and put it into a variable. This code uses the typedefs defined in steps 2 and 3, along with the dynamic library variable from step 4.
    final ThumbnailerC thumbnailer_c = _dylib
        .lookup<ffi.NativeFunction<thumbnailer_c_func>>('thumbnailer')
        .asFunction();

    // Convert a Dart [String] to a Utf8-encoded null-terminated C string.
    final ffi.Pointer<Utf8> video_path_c_str = Utf8.toUtf8(video_path).cast();
    final ffi.Pointer<Utf8> thumbnail_path_c_str =
        Utf8.toUtf8(thumbnail_path).cast();

    // Call the C function.
    thumbnailer_c(video_path_c_str, thumbnail_path_c_str);
  }
}

// Create a typedef with the FFI type signature of the C function.
// Commonly used types defined by dart:ffi library include Double, Int32, NativeFunction, Pointer, Struct, Uint8, and Void.
typedef thumbnailer_c_func = ffi.Void Function(
    ffi.Pointer<Utf8>, ffi.Pointer<Utf8>);

// Create a typedef for the variable that you’ll use when calling the C function.
typedef ThumbnailerC = void Function(ffi.Pointer<Utf8>, ffi.Pointer<Utf8>);
