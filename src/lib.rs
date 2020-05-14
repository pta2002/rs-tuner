pub mod note {
    /// Represents a Note that is some distance away from a base frequency
    #[derive(Debug)]
    pub struct Note {
        pub octaves: i32,
        pub semitones: i32,
        pub cents: i32,
        base: f32,
    }

    /// Represents a note absolutely
    #[derive(Debug)]
    pub struct AbsNote {
        pub octave: i32,
        pub semitone: i32,
        pub cents: i32,
        base: f32,
    }

    impl Note {
        /// Creates a new Note and calculates its position relative to the base frequency
        ///
        /// # Examples
        /// ```
        /// let base = 440.0;
        /// let note = tuner::note::Note::new(base, 220.0);
        ///
        /// assert_eq!(note.octaves, -1);
        /// assert_eq!(note.semitones, 0);
        /// assert_eq!(note.cents, 0);
        /// ```
        pub fn new(base: f32, freq: f32) -> Note {
            let semitones_away = (freq / 440.0).log2() * 12.0;
            let cents = (semitones_away * 100.0).trunc() as i32;
            let semitones = cents / 100;
            let octaves = semitones / 12;

            Note {
                octaves,
                semitones: semitones - octaves * 12,
                cents: cents - semitones * 100, // TODO check this it's probably wrong :s
                base,
            }
        }

        /// Converts a Note to an absolute Note
        ///
        /// # Examples
        /// ```
        /// let note = tuner::note::Note::new(440.0, 440.0);
        /// let abs = note.abs_note();
        ///
        /// assert_eq!(abs.octave, 4);
        /// assert_eq!(abs.semitone, 0);
        /// assert_eq!(abs.cents, 0);
        /// ```
        ///
        /// ```
        /// let note = tuner::note::Note::new(440.0, 82.407);
        /// let abs = note.abs_note();
        ///
        /// assert_eq!(abs.octave, 2);
        /// assert_eq!(abs.semitone, 8);
        /// ```
        pub fn abs_note(&self) -> AbsNote {
            let octave = 4 + self.octaves;
            let semitone = ((self.semitones % 12) + 12) % 12;
            let cents = self.cents % 100;

            AbsNote {
                octave,
                semitone,
                cents,
                base: self.base,
            }
        }
    }

    impl AbsNote {
        pub fn as_text(&self) -> String {
            let note = match self.semitone {
                0 => "A",
                1 => "A#",
                2 => "B",
                3 => "C",
                4 => "C#",
                5 => "D",
                6 => "D#",
                7 => "E",
                8 => "F",
                9 => "F#",
                10 => "G",
                _ => "F#",
            };

            format!("{}{} {}", note, self.octave, self.cents)
        }
    }
}
