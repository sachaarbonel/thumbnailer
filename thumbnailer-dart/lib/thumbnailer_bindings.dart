// Import dart:ffi.
import 'dart:ffi' as ffi;
import 'package:ffi/ffi.dart';

class ThumbnailerBindings {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  ThumbnailerBindings(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  ThumbnailerBindings.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void thumbnail_file({
    required String video_path,
    required String thumbnail_path,
  }) {
    final video_path_ptr = video_path.toNativeUtf8();
    final thumbnail_path_ptr = thumbnail_path.toNativeUtf8();
    _thumbnail_file(video_path_ptr, thumbnail_path_ptr);
  }

  late final _thumbnailer_c_ptr =
      _lookup<ffi.NativeFunction<_c_thumbnailer_c>>('thumbnail_file');
  late final _dart_thumbnailer_c _thumbnail_file =
      _thumbnailer_c_ptr.asFunction<_dart_thumbnailer_c>();
}

typedef _c_thumbnailer_c = ffi.Void Function(
  ffi.Pointer<Utf8> video_path,
  ffi.Pointer<Utf8> thumbnail_path,
);

typedef _dart_thumbnailer_c = void Function(
  ffi.Pointer<Utf8> video_path,
  ffi.Pointer<Utf8> thumbnail_path,
);
