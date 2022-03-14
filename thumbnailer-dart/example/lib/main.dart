import 'package:thumbnailer/thumbnailer.dart' as thumbnailer;

void main() {
  final output_path = "assets/assets_bbb-vp9-opus.png";
  final video_path = "assets/assets_bbb-vp9-opus.webm";
  thumbnailer.thumbnail_file(video_path, output_path);
}
