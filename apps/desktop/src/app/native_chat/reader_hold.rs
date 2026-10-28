//! Holding the reader's place while new content arrives.
//!
//! CDXC:SessionChat 2026-10-27 SEE-ALSO:
//! The rule is `ghostex_gx_chat_core::transcript::follow_policy` (its DECISION has the user's
//! words); the phone applies the same one in `apps/mobile/app/src/chat/native/transcript/`.
//! A press inside the transcript, or a text box that answers the chat's blocks taking the keyboard
//! (a form's note or answer, the question card's answer, the answer dock outside the transcript),
//! holds the reader's place. Only his own actions release it: a press outside the transcript (this
//! pane or any other pane, the sidebar and the Session details dock included), a scrollbar drag (the
//! bar is the reader's own hand on the list), a wheel or trackpad gesture that reaches the
//! transcript's end, or the "Scroll to bottom" pill and its shortcut. A press outside while he is
//! still above the end leaves him where he reads.
//!
//! The list is left in [`gpui::FollowMode::Normal`] while it is held, not in a stopped `Tail`: a
//! stopped `Tail` arms itself again on any layout that leaves the viewport within a pixel of the
//! end (`ListState::layout_items`), which is every streaming frame of a reply the reader is reading
//! the top of.

use super::state::NativeChatView;
use ghostex_gx_chat_core::transcript::follow_policy::{ReaderHold, follows_tail};
use gpui::{Context, FollowMode};

impl NativeChatView {
    /// A press anywhere in the chat: inside the transcript it holds the reader's place, outside it
    /// hands the list back — following when he is at the end, stopped where he reads otherwise.
    pub(super) fn note_chat_press(&mut self, inside_transcript: bool, cx: &mut Context<Self>) {
        if inside_transcript {
            self.hold_reader_place(cx);
        } else if self.release_reader_place() {
            cx.notify();
        }
    }

    /// A press inside the transcript, or one of its text boxes taking the keyboard: new content
    /// waits.
    pub(super) fn hold_reader_place(&mut self, cx: &mut Context<Self>) {
        if self.reader_hold == ReaderHold::InTranscript {
            return;
        }
        self.reader_hold = ReaderHold::InTranscript;
        // Normal even when the list is already stopping its tail: a stopped tail arms itself again
        // on any layout that leaves the viewport within a pixel of the end.
        self.list.set_follow_mode(FollowMode::Normal);
        cx.notify();
    }

    /// The reader left the transcript: the list is handed back in the state it would have had —
    /// following from the bottom, or stopped where he reads. Returns whether anything changed.
    pub(super) fn release_reader_place(&mut self) -> bool {
        if self.reader_hold == ReaderHold::Nothing {
            return false;
        }
        self.reader_hold = ReaderHold::Nothing;
        let top = self.list.logical_scroll_top();
        let follows = self.may_follow_tail();
        self.list.set_follow_mode(FollowMode::Tail);
        if !follows {
            self.list.scroll_to(top);
        }
        true
    }

    /// He asked for the bottom himself: the pill, its shortcut, or a wheel that reached the end.
    pub(super) fn reader_asked_for_bottom(&mut self) {
        self.reader_hold = ReaderHold::Nothing;
        self.list.set_follow_mode(FollowMode::Tail);
    }

    /// Whether the list may follow its tail this frame (the core's rule).
    pub(super) fn may_follow_tail(&self) -> bool {
        follows_tail(
            self.reader_hold,
            !super::scroll_bottom::transcript_left_bottom(&self.list),
        )
    }

    /// A wheel or trackpad gesture reached the transcript's end: the list follows again. The list's
    /// scroll handler calls this; it never fires for a scrollbar drag, which `render` releases on its
    /// own (`is_scrollbar_dragging`), because a drag sets the offset directly.
    pub(super) fn note_scrolled_to_end(&mut self) {
        if self.reader_hold != ReaderHold::Nothing || self.list.is_following_tail() {
            return;
        }
        if super::scroll_bottom::transcript_distance_to_end(&self.list)
            > gpui::px(super::scroll_bottom::LIST_RESUME_TOLERANCE)
        {
            return;
        }
        self.list.set_follow_mode(FollowMode::Tail);
    }

    /// A scrollbar drag is the reader's own hand on the list: it ends a hold, which matters in a
    /// narrow pane where the bar overlays the transcript and a press on it starts one.
    pub(super) fn release_hold_for_scrollbar_drag(&mut self) {
        if self.reader_hold != ReaderHold::Nothing && self.list.is_scrollbar_dragging() {
            self.release_reader_place();
        }
    }
}
