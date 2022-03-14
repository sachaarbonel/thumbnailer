use std::collections::HashMap;
use std::fs::File;

use av_codec as codec;
use av_data as data;
use av_format as format;
use av_vorbis::decoder::VORBIS_DESCR;
use codec::common::CodecList;
use codec::decoder::Codecs as DecCodecs;
use codec::decoder::Context as DecContext;
use data::frame::FrameBufferConv;
use data::frame::{ArcFrame, MediaKind};
use data::params;
use format::buffer::AccReader;
use format::demuxer::{Context, Event};
use image::ImageError;
use image::Rgba;
use image::RgbaImage;
use libopus::decoder::OPUS_DESCR;
use libvpx::decoder::VP9_DESCR;
use matroska::demuxer::MkvDemuxer;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThumbnailerError {
    #[error("format error: {0}")]
    Format(#[from] format::error::Error),
    #[error("codec error: {0}")]
    Codec(#[from] codec::error::Error),
    #[error("frame error: {0}")]
    Frame(#[from] data::frame::FrameError),
    #[error("image error: {0}")]
    Image(#[from] ImageError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct Thumbnailer {
    decoders: HashMap<isize, DecContext>,
    demuxer: Context,
    video: Option<params::VideoInfo>,
}

impl Thumbnailer {
    ///```rust
    ///     let output_path = "assets/assets_bbb-vp9-opus.png";
    ///     let mut th = Thumbnailer::from_path("assets/assets_bbb-vp9-opus.webm").unwrap();
    ///     th.save_image(output_path);
    ///```
    pub fn from_path<P: ?Sized>(path: &P) -> Result<Self, ThumbnailerError>
    where
        P: AsRef<Path>,
    {
        let r = File::open(path)?;
        let ar = AccReader::with_capacity(4 * 1024, r);

        let mut c = Context::new(Box::new(MkvDemuxer::new()), Box::new(ar));

        c.read_headers()?;

        let decoders = DecCodecs::from_list(&[VP9_DESCR, OPUS_DESCR, VORBIS_DESCR]);

        let mut video_info = None;
        let mut decs: HashMap<isize, DecContext> = HashMap::with_capacity(2);
        for st in &c.info.streams {
            if let Some(ref codec_id) = st.params.codec_id {
                if let Some(mut ctx) = DecContext::by_name(&decoders, codec_id) {
                    if let Some(ref extradata) = st.params.extradata {
                        ctx.set_extradata(extradata);
                    }
                    ctx.configure()?;
                    decs.insert(st.index as isize, ctx);
                    match st.params.kind {
                        Some(params::MediaKind::Video(ref info)) => {
                            let width = info.width as u32;
                            let height = info.height as u32;
                            println!("{}x{}", width, height);
                            video_info = Some(info.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(Self {
            decoders: decs,
            demuxer: c,
            video: video_info,
        })
    }

    pub fn save_image(&mut self, output_path: &str) {
        let mut frame_idx = 0;
        while let Ok(data) = self.decode_one() {
            if let Some(frame) = data {
                match frame.kind {
                    MediaKind::Video(_) => {
                        println!("{:#?}", frame.t);

                        if frame.t.pts == Some(960) {
                            self.frame_to_image(frame, self.width(), self.height(), output_path);
                        }
                    }
                    _ => {}
                }
                frame_idx += 1;
            }
        }
    }
    // This function decodes a single frame using the most appropriate decoder
    fn decode_one(&mut self) -> Result<Option<ArcFrame>, String> {
        // The demuxer reads which event has occurred
        match self.demuxer.read_event() {
            // If a new packet has been found, decode it
            Ok(event) => match event {
                Event::NewPacket(pkt) => {
                    // Choose the right decoder for the packet
                    let pkt_index = &pkt.stream_index;
                    if let Some(decoder) = self.decoders.get_mut(pkt_index) {
                        decoder.send_packet(&pkt);
                        Ok(decoder.receive_frame().ok())
                    } else {
                        // If a packet cannot be decoded, it will be skipped
                        println!("Skipping packet at index {}", pkt.stream_index);
                        Ok(None)
                    }
                }
                // When the EOF is reached, the decoding process is stopped
                Event::Eof => {
                    println!("EOF reached.");
                    Err("EOF reached".to_owned())
                }
                _ => {
                    // If an unsupported event occurs,
                    // the decoding process is stopped
                    println!("Unsupported event {:?}", event);
                    Err("Unsupported event".to_owned())
                }
            },
            Err(err) => {
                // If there are no more events, the decoding process is stopped
                println!("No more events {:?}", err);
                Err("No more events".to_owned())
            }
        }
    }

    fn frame_to_image(
        &self,
        frame: std::sync::Arc<data::frame::Frame>,
        width: usize,
        height: usize,
        output_path: &str,
    ) -> Result<(), ThumbnailerError> {
        let y_plane: &[u8] = frame.buf.as_slice(0)?;
        let y_stride = frame.buf.linesize(0)? as usize;
        let u_plane: &[u8] = frame.buf.as_slice(1)?;
        let v_plane: &[u8] = frame.buf.as_slice(2)?;
        let img = RgbaImage::from_fn(width as u32, height as u32, |x, y| {
            let (cx, cy) = (x as usize, y as usize);
            let y = y_plane[cy * y_stride + cx] as f64;
            let u = u_plane[cy / 2 * width / 2 + cx / 2] as f64;
            let v = v_plane[cy / 2 * width / 2 + cx / 2] as f64;
            let r = 1.164 * (y - 16.0) + 1.596 * (v - 128.0);
            let g = 1.164 * (y - 16.0) - 0.391 * (u - 128.0) - 0.813 * (v - 128.0);
            let b = 1.164 * (y - 16.0) + 2.018 * (u - 128.0);
            Rgba([clamp(r), clamp(g), clamp(b), 255])
        });
        img.save(output_path)?;
        Ok(())
    }

    fn width(&self) -> usize {
        self.video.as_ref().unwrap().width
    }
    fn height(&self) -> usize {
        self.video.as_ref().unwrap().height
    }
}

fn clamp(value: f64) -> u8 {
    if value <= 0.0 {
        return 0;
    }
    if value >= 255.0 {
        return 255;
    }
    value as u8
}
