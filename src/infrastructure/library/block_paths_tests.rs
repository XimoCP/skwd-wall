#![cfg(test)]

use super::far_block_path;

#[test]
fn far_block_kinds() {
    assert_eq!(far_block_path("/c/skwd-wall-v2/thumbs/a.webp"), "/c/skwd-wall-v2/blocks/a.bc1");
    assert_eq!(
        far_block_path("/c/skwd-wall-v2/video-thumbs/clip.webp"),
        "/c/skwd-wall-v2/blocks/vid--clip.bc1"
    );
    assert_eq!(
        far_block_path("/c/skwd-wall-v2/we-thumbs/77.webp"),
        "/c/skwd-wall-v2/blocks/we--77.bc1"
    );
}
