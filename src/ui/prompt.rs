//! Text input widget.

#[derive(Clone, Default)]
pub struct Prompt {
    /// Current value of the input box.
    pub input: String,
    /// Cursor position measured in characters (not bytes).
    pub character_index: usize,
    /// The last submitted value, set by [`Prompt::submit_message`].
    pub message: String,
}

impl Prompt {
    /// Creates a new empty `Prompt` with the cursor at position 0.
    pub fn new() -> Self {
        Self {
            input: "".to_string(),
            character_index: 0,
            message: String::new(),
        }
    }

    /// Moves the cursor one character to the left, clamped at the start.
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    /// Moves the cursor one character to the right, clamped at the end of input.
    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    /// Clamps `new_cursor_pos` to `[0, input.len()]` in characters.
    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    /// Resets the cursor to position 0 without clearing the input.
    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    /// Clears both `input` and `message` and resets the cursor.
    pub fn reset(&mut self) {
        self.input.clear();
        self.message.clear();
        self.reset_cursor();
    }

    /// Copies `input` into `message`, then clears `input` and resets the cursor.
    pub fn submit_message(&mut self) {
        self.message = self.input.clone();
        self.input.clear();
        self.reset_cursor();
    }

    /// Inserts `new_char` at the cursor position if the input is under 200 bytes.
    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        // Limit accommodates long API tokens (Lichess tokens can be 100+ chars).
        if index < 200 {
            self.input.insert(index, new_char);
            self.move_cursor_right();
        }
    }

    /// Returns the byte offset of the current cursor position.
    ///
    /// Each Unicode scalar can occupy multiple bytes, so the character index
    /// cannot be used directly as a byte index into the backing `String`.
    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    /// Deletes the character immediately to the left of the cursor.
    ///
    /// Uses char-boundary-safe reconstruction instead of `String::remove` to
    /// avoid panicking on multi-byte characters.
    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
}
