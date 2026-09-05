use crate::contracts::browser::{Source, SourceAvailability, SourceUnavailableReason};

use super::super::config::Config;

pub fn source_availability(
    config: &Config,
    steam_helper_available: Option<bool>,
) -> Vec<SourceAvailability> {
    Source::ALL
        .into_iter()
        .map(|source| {
            let unavailable = match source {
                Source::Wallhaven => None,
                Source::Steam => {
                    if !config.steam_enabled() {
                        Some(SourceUnavailableReason::Disabled)
                    } else if config.steam_uses_client() && steam_helper_available == Some(false) {
                        Some(SourceUnavailableReason::MissingSteamHelper)
                    } else {
                        None
                    }
                }
                Source::Unsplash => {
                    if !config.source_enabled("unsplash") {
                        Some(SourceUnavailableReason::Disabled)
                    } else if config.unsplash_access_key().is_empty() {
                        Some(SourceUnavailableReason::MissingCredentials)
                    } else {
                        None
                    }
                }
                Source::Pexels => {
                    if !config.source_enabled("pexels") {
                        Some(SourceUnavailableReason::Disabled)
                    } else if config.pexels_api_key().is_empty() {
                        Some(SourceUnavailableReason::MissingCredentials)
                    } else {
                        None
                    }
                }
                Source::Youtube | Source::Bing => (!config.source_enabled(source.key()))
                    .then_some(SourceUnavailableReason::Disabled),
            };
            SourceAvailability { source, unavailable }
        })
        .collect()
}
