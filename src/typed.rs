//! A line somebody types, and who asked for it. Spec 0059.
//!
//! **One field, not a widget set.** This building has one thing that asks
//! you to write something and will have two, and a text field that could
//! be laid out anywhere, scrolled, selected in and pasted into is a
//! toolkit nobody asked for. What is here is what a line of typing needs
//! and nothing else: the characters, where the caret is, and what to do
//! when Return lands.
//!
//! The characters come off `Game::typed`, which is the engine's spec
//! 0087: what a press came to after the layout had its say, rather than
//! which key it was. Backspace, Return and Escape are keys and arrive the
//! other way, which is where a field has to look for them anyway.

/// What a field is open for, which is what its answer goes back to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asking {
    /// The watchman at the top of the bridge. Spec 0058.
    Watchman,
}

/// The one open field: what it is for, what has been typed, and what it
/// is asking.
///
/// **The question travels with the field.** It is drawn over the words
/// you are writing, and a field that only held the answer would need
/// whoever draws it to work out what was asked, which is the same fact
/// in two places.
#[derive(Clone, Debug)]
pub struct Field {
    pub asking: Asking,
    pub question: String,
    /// Whether the last line handed over was not an answer, so whoever
    /// draws this knows to say so. Spec 0058: not a fail, a reply.
    pub again: bool,
    said: String,
}

/// The most a line can hold.
///
/// **Short, because this is an answer and not a message.** Long enough
/// for a sentence somebody means, short enough that the line never has to
/// wrap or scroll, which is the whole of what keeps this a field and not
/// a text box.
pub const MOST: usize = 48;

impl Field {
    pub fn asked(asking: Asking, question: &str) -> Self {
        Self {
            asking,
            question: question.to_owned(),
            again: false,
            said: String::new(),
        }
    }

    /// What has been typed so far.
    pub fn said(&self) -> &str {
        &self.said
    }

    /// And what it comes to as an answer.
    pub fn answer(&self) -> String {
        folded(&self.said)
    }

    /// Clears the line and says the last one was not an answer, which is
    /// what a wrong try does. The question stays, because he does not
    /// ask a different one.
    pub fn try_again(&mut self) {
        self.said.clear();
        self.again = true;
    }

    /// Whether anything has been typed into it.
    pub fn empty(&self) -> bool {
        self.answer().is_empty()
    }

    /// Takes what a press came to, which may be several characters and
    /// may be none.
    ///
    /// Stops at [`MOST`] rather than scrolling. A field that runs off its
    /// own plate is a field you cannot read what you wrote in.
    pub fn typed(&mut self, text: &str) {
        for one in text.chars() {
            if self.said.chars().count() >= MOST {
                return;
            }

            self.said.push(one);
        }
    }

    /// And takes one character back off.
    ///
    /// By character and not by byte, or holding backspace over anything
    /// somebody typed with an umlaut in it panics on a boundary.
    pub fn rubbed_out(&mut self) {
        self.said.pop();
    }
}

/// A line of typing as the words somebody meant: trimmed, folded to
/// lower case, and with the runs of space squeezed.
///
/// **Because somebody typing an answer is not typing a password.**
/// Whoever reads one is comparing it with words, and a capital, a stray
/// space at the end or two in the middle is not somebody being wrong.
///
/// **One function, used on both sides of the comparison.** What is typed
/// and what is accepted have to be folded the same way or the fold is
/// the thing that refuses a right answer, and two copies of these four
/// lines is how that happens.
pub fn folded(said: &str) -> String {
    said.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field() -> Field {
        Field::asked(Asking::Watchman, "what does it look like")
    }

    /// Spec 0059: what arrives is what was typed.
    #[test]
    fn it_takes_what_was_typed() {
        let mut one = field();

        one.typed("a");
        one.typed(" ");
        one.typed("tower");

        assert_eq!(one.said(), "a tower");
        assert_eq!(one.answer(), "a tower");
        assert!(!one.empty());
    }

    /// And an answer is read the way somebody means it.
    ///
    /// **Both halves, because a fold that did nothing would pass a test
    /// that only typed lower case.** A capital, the spaces round it and
    /// the double space in the middle are each a way of being right.
    #[test]
    fn an_answer_is_read_the_way_it_was_meant() {
        let mut one = field();

        one.typed("  A  Tower  ");

        assert_eq!(one.answer(), "a tower");
        assert_ne!(one.said(), one.answer(), "nothing was folded at all");
    }

    /// Spec 0059: and a field with nothing in it says so, whatever the
    /// spacebar has been doing.
    #[test]
    fn a_field_of_spaces_is_empty() {
        let mut one = field();

        one.typed("   ");

        assert!(one.empty());
        assert!(!one.said().is_empty(), "the spaces were never taken");
    }

    /// Spec 0059: backspace takes a character and not a byte.
    ///
    /// **Written because the obvious one panics.** `said.truncate(len - 1)`
    /// on anything with an umlaut in it cuts a character in half and Rust
    /// refuses, which is a crash in a field anybody can type into.
    #[test]
    fn it_rubs_out_a_character_and_not_a_byte() {
        let mut one = field();

        one.typed("sjö");
        assert_eq!(one.said(), "sjö");

        one.rubbed_out();
        assert_eq!(one.said(), "sj");

        one.rubbed_out();
        one.rubbed_out();
        one.rubbed_out();
        assert_eq!(one.said(), "", "rubbing out an empty field is not an error");
    }

    /// Spec 0059: it stops rather than running off its own plate.
    #[test]
    fn it_holds_no_more_than_a_line() {
        let mut one = field();

        for _ in 0..MOST * 2 {
            one.typed("x");
        }

        assert_eq!(one.said().chars().count(), MOST);

        // and a press that brings several characters is held to the same
        // line rather than being let through whole
        let mut two = field();
        two.typed(&"y".repeat(MOST * 2));

        assert_eq!(two.said().chars().count(), MOST);
    }
}
