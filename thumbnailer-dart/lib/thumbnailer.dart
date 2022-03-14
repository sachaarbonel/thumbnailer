import 'dart:ffi';
import 'dart:io';

import 'package:thumbnailer/thumbnailer_bindings.dart';

// import 'thumbnailer_bindings_generated.dart';

void thumbnail_file(
        {required String video_path, required String thumbnail_path}) =>
    _bindings.thumbnail_file(
        video_path: video_path,
        thumbnail_path: thumbnail_path); //:thumbnail_path

const String _architecture = 'x86_64-apple-darwin';
const String _libPath =
    '/Users/sachaarbonel/rust-dev/thumbnailer/target/$_architecture/debug';
const String _libName = 'thumbnail_file';

/// The dynamic library in which the symbols for [ThumbnailerBindings] can be found.
final DynamicLibrary _dylib = () {
  if (Platform.isMacOS || Platform.isIOS) {
    const lib = '$_libPath/lib$_libName.dylib';
    print(lib);
    return DynamicLibrary.open(lib);
  }
  if (Platform.isAndroid || Platform.isLinux) {
    return DynamicLibrary.open('$_libPath/lib$_libPath$_libName.so');
  }
  if (Platform.isWindows) {
    return DynamicLibrary.open('$_libPath//lib$_libPath$_libName.dll');
  }
  throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
}();

/// The bindings to the native functions in [_dylib].
final ThumbnailerBindings _bindings = ThumbnailerBindings(_dylib);
