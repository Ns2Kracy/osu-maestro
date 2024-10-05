#[derive(thiserror::Error, Debug)]
pub enum OverlayError {
    #[error("failed to download the gosumemory overlay")]
    DownloadOverlay,
    #[error("failed to inject the overlay into the osu game")]
    InjectOverlay,
    #[error("failed to remove injected overlay from the osu game")]
    RemoveOverlay,
    #[error("overlay is not supported on this platform")]
    UnsupportedPlatform,
}

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum MaestroError {
    #[error("{0}")]
    OverlayError(#[from] OverlayError),
}

impl MaestroError {
    pub fn overlay_error(error: OverlayError) -> Self {
        MaestroError::OverlayError(error)
    }
}
