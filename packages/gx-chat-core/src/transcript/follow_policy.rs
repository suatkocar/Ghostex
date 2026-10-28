//! When the transcript may move itself to new content, on every renderer.
//!
//! CDXC:SessionChat 2026-10-27 DECISION:
//! User: "O an soru seçiliyse, aktifse, ben dışarıya başka bir yere tıklamadıkça, en aşağı git
//! demedikçe … odağı kaybetmemesi gerekmiyor mu?" and, widening it, "yeni cevap geldikçe en sona
//! atıyor hep oysa ben scroll ile yukarı bi yere gitmişim yani niye force'luyor beni": new content is
//! signalled, never applied by force. The viewport follows the tail only while the reader is at the
//! bottom and nothing in the transcript holds him, and only his own actions bring it back: a wheel
//! or trackpad gesture that reaches the transcript's end (the list's scroll handler), a scrollbar
//! drag (released as soon as the bar is dragged), the "Scroll to bottom" pill or its shortcut, and a
//! press outside the transcript (`reader_hold.rs`). A press outside while he is still above the end
//! leaves him where he reads. Supersedes the 2026-10-04 rule in
//! `apps/desktop/src/app/native_chat/scroll_bottom.rs` that only a scroll up stops following.
//!
//! The renderers own the inputs — whether the list is at the bottom, whether a press landed inside
//! the transcript, whether a text box in it holds the keyboard. The desktop calls this function
//! directly (`native_chat/reader_hold.rs`); the phone asks it through the core's `followTail` query
//! (`query.rs`, one call per hold or position transition, never per frame) from
//! `apps/mobile/app/src/chat/native/transcript/Transcript.tsx`.

/// The reader's place is held: new content must not move the viewport, and must not take focus.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ReaderHold {
    /// Nothing in the transcript holds the reader.
    #[default]
    Nothing,
    /// He pressed inside the transcript, or a text box in it took the keyboard, and has not left it
    /// with a press outside, a scroll to the end, the pill or a send.
    InTranscript,
}

/// Whether new content may move the viewport: never while it is held, and only at the bottom.
///
/// `at_bottom` is the renderer's own reading of "the reader is at the transcript's end" while the
/// list is not already following; a renderer that follows its tail is at the bottom by definition.
pub fn follows_tail(hold: ReaderHold, at_bottom: bool) -> bool {
    hold == ReaderHold::Nothing && at_bottom
}
