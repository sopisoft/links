use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::images::CachedImage;
use crate::links::LinkId;

const FAVICON_TTL: Duration = Duration::from_secs(60 * 60 * 24 * 7);

pub struct AppState {
    pub favicons: Mutex<FaviconCache>,
    pub svgs: Mutex<SvgCache>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            favicons: Mutex::new(FaviconCache::default()),
            svgs: Mutex::new(SvgCache::default()),
        }
    }
}

#[derive(Default)]
pub struct FaviconCache {
    entries: HashMap<LinkId, CacheEntry>,
}

impl FaviconCache {
    pub fn get_mut(&mut self, id: LinkId, now: Instant) -> Option<&mut CachedImage> {
        let expired = self
            .entries
            .get(&id)
            .map(|entry| now > entry.expires_at)
            .unwrap_or(true);
        if expired {
            self.entries.remove(&id);
            return None;
        }
        self.entries.get_mut(&id).map(|entry| &mut entry.image)
    }

    pub fn insert(&mut self, id: LinkId, image: CachedImage, now: Instant) {
        self.entries.insert(
            id,
            CacheEntry {
                image,
                expires_at: now + FAVICON_TTL,
            },
        );
    }
}

#[derive(Default)]
pub struct SvgCache {
    pub icon: Option<CachedImage>,
    pub ogp: Option<CachedImage>,
}

struct CacheEntry {
    image: CachedImage,
    expires_at: Instant,
}
