
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';

import 'package:thumbnailer/thumbnailer_bindings.dart';

// import 'thumbnailer_bindings_generated.dart';

void thumbnail_file(String video_path, String thumbnail_path) => _bindings.save_image(video_path, thumbnail_path);


const String _libName = 'thumbnailer';

/// The dynamic library in which the symbols for [ThumbnailerBindings] can be found.
final DynamicLibrary _dylib = () {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.open('$_libName.framework/$_libName');
  }
  if (Platform.isAndroid || Platform.isLinux) {
    return DynamicLibrary.open('lib$_libName.so');
  }
  if (Platform.isWindows) {
    return DynamicLibrary.open('$_libName.dll');
  }
  throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
}();

/// The bindings to the native functions in [_dylib].
final ThumbnailerBindings _bindings = ThumbnailerBindings(_dylib);
