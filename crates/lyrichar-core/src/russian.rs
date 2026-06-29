use std::str::Lines;

mod sealed {
    pub trait Sealed {}
}

#[macro_export]
macro_rules! lower_russian_vowels {
    () => {
        'а' | 'е' | 'ё' | 'и' | 'о' | 'у' | 'ы' | 'э' | 'ю' | 'я'
    };
}

#[macro_export]
macro_rules! upper_russian_vowels {
    () => {
        'А' | 'Е' | 'Ё' | 'И' | 'О' | 'У' | 'Ы' | 'Э' | 'Ю' | 'Я'
    };
}

#[macro_export]
macro_rules! russian_vowels {
    () => {
        upper_russian_vowels!() | lower_russian_vowels!()
    };
}

pub const fn is_russian_vowel(character: char) -> bool {
    matches!(character, russian_vowels!())
}

pub trait RussianVowel: sealed::Sealed {
    fn is_russian_vowel(&self) -> bool;
}

impl sealed::Sealed for char {}

impl RussianVowel for char {
    fn is_russian_vowel(&self) -> bool {
        matches!(self, russian_vowels!())
    }
}

pub trait CountRussianVowels: AsRef<str> {
    fn count_russian_vowels(&self) -> RussianVowelCounts<'_> {
        RussianVowelCounts::new(self.as_ref().lines())
    }
}

impl<T: AsRef<str> + ?Sized> CountRussianVowels for T {}

pub struct RussianVowelCounts<'a> {
    lines: Lines<'a>,
}

impl<'a> RussianVowelCounts<'a> {
    pub(crate) const fn new(lines: Lines<'a>) -> Self {
        Self { lines }
    }
}

impl<'a> Iterator for RussianVowelCounts<'a> {
    type Item = (&'a str, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            (
                line,
                line.chars()
                    .filter(|character| character.is_russian_vowel())
                    .count(),
            )
        })
    }
}
