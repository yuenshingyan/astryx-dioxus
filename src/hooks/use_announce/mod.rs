// Adapted from Astryx useAnnounce. See THIRD_PARTY_NOTICES.md.

// Only the internal polite writer and native-dialog fallback are needed here.
// ModalAnnounceProvider's presentation/exit routing is not a migrated surface.
pub(crate) const ANNOUNCE_CONTROLLER: &str = include_str!("use_announce.js");
