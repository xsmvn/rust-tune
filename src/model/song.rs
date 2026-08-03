use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs::File;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, StandardTagKey, StandardVisualKey};
use symphonia::core::probe::Hint;
use symphonia::core::formats::FormatOptions;
use symphonia::default::get_probe;
use iced::widget::image::Handle;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    pub id: u32,
    pub title: String,
    pub artiste: String,
    pub album: Option<String>,
    pub genre: String,
    pub release_year: Option<u32>,
    pub duration_secs: Option<f64>,
    pub file_path: String,

    /// Données brutes de la jacquette
    #[serde(skip)]
    pub cover: Option<Vec<u8>>,

    /// Handle iced créé une seule fois
    #[serde(skip)]
    pub cover_handle: Option<Handle>,
}

impl Song {
    pub fn from_path(path: &Path, id: u32) -> Self {
        let file_stem = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".into());

        let mut title = file_stem;
        let mut artiste = "Unknown Artist".to_string();
        let mut album = None;
        let mut genre = "Unknown".to_string();
        let mut release_year = None;
        let mut duration_secs = None;

        if let Ok(file) = File::open(path) {
            let mss = MediaSourceStream::new(Box::new(file), Default::default());

            let mut hint = Hint::new();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                hint.with_extension(&ext.to_lowercase());
            }

            if let Ok(mut probed) = get_probe().format(
                &hint,
                mss,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            ) {
                {
                    let mut metadata = probed.format.metadata();
                    if let Some(rev) = metadata.current() {
                        for tag in rev.tags() {
                            apply_tag(tag, &mut title, &mut artiste, &mut album, &mut genre, &mut release_year);
                        }
                    }
                }

                if let Some(metadata) = probed.metadata.get() {
                    if let Some(rev) = metadata.current() {
                        for tag in rev.tags() {
                            apply_tag(tag, &mut title, &mut artiste, &mut album, &mut genre, &mut release_year);
                        }
                    }
                }

                if let Some(track) = probed.format.default_track() {
                    let p = &track.codec_params;
                    if let (Some(frames), Some(sr)) = (p.n_frames, p.sample_rate) {
                        duration_secs = Some(frames as f64 / sr as f64);
                    } else if let (Some(tb), Some(frames)) = (p.time_base, p.n_frames) {
                        let t = tb.calc_time(frames);
                        duration_secs = Some(t.seconds as f64 + t.frac);
                    }
                }
            }
        }

        Song {
            id,
            title,
            artiste,
            album,
            genre,
            release_year,
            duration_secs,
            file_path: path.to_string_lossy().into(),
            cover: None,
            cover_handle: None,
        }
    }

    pub fn duration_formatted(&self) -> String {
        match self.duration_secs {
            Some(secs) => {
                let total = secs as u64;
                format!("{:02}:{:02}", total / 60, total % 60)
            }
            None => "--:--".into(),
        }
    }
}

fn apply_tag(
    tag: &symphonia::core::meta::Tag,
    title: &mut String,
    artiste: &mut String,
    album: &mut Option<String>,
    genre: &mut String,
    release_year: &mut Option<u32>,
) {
    if let Some(key) = tag.std_key {
        match key {
            StandardTagKey::TrackTitle => *title = tag.value.to_string(),
            StandardTagKey::Artist
            | StandardTagKey::AlbumArtist
            | StandardTagKey::Performer => *artiste = tag.value.to_string(),
            StandardTagKey::Album => *album = Some(tag.value.to_string()),
            StandardTagKey::Genre => *genre = tag.value.to_string(),
            StandardTagKey::Date
            | StandardTagKey::ReleaseDate
            | StandardTagKey::OriginalDate => {
                if let Some(y) = tag.value.to_string().get(0..4) {
                    if let Ok(year) = y.parse() {
                        *release_year = Some(year);
                    }
                }
            }
            _ => {}
        }
    }
}

/// Extraction de la jacquette (utilisée depuis home.rs)
pub fn extract_cover_from_path(path: &Path) -> Option<Vec<u8>> {
    let file = File::open(path).ok()?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(&ext.to_lowercase());
    }

    let mut probed = get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .ok()?;

    // 1. Format metadata
    {
        let mut metadata = probed.format.metadata();
        if let Some(rev) = metadata.current() {
            if let Some(data) = pick_cover(rev.visuals()) {
                return Some(data);
            }
        }
    }

    // 2. Probe metadata (ID3…)
    if let Some(metadata) = probed.metadata.get() {
        if let Some(rev) = metadata.current() {
            if let Some(data) = pick_cover(rev.visuals()) {
                return Some(data);
            }
        }
    }

    None
}

fn pick_cover(visuals: &[symphonia::core::meta::Visual]) -> Option<Vec<u8>> {
    if let Some(v) = visuals
        .iter()
        .find(|v| v.usage == Some(StandardVisualKey::FrontCover))
    {
        return Some(v.data.to_vec());
    }
    visuals.first().map(|v| v.data.to_vec())
}