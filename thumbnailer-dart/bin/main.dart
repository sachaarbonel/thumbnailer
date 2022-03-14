import 'package:thumbnailer/thumbnailer.dart' as thumbnailer;

void main() {
  final output_path = "assets/vp9-opus.png";
  final video_path = "assets/vp9-opus.webm";
  thumbnailer.thumbnail_file(
      video_path: video_path, thumbnail_path: output_path);
}
