use std::{collections::VecDeque, sync::Arc};

pub fn take_until(s: &str, until: char) -> Option<(Arc<str>, &str)> {
    let mut taken = String::new();
    let mut chars: VecDeque<char> = s.chars().collect();

    loop {
        if let Some(zeroth) = chars.pop_front() {
            if zeroth == until {
                let taken_str = s.strip_prefix(format!("{taken}{until}").as_str()).unwrap();
                return Some((Arc::from(taken.as_str()), taken_str));
            } else {
                taken.push(zeroth);
            }
        } else {
            return None;
        }
    }
}

pub fn take_string(s: &str) -> Option<(Arc<str>, &str)> {
    let mut taken = String::new();
    let mut chars: VecDeque<char> = s.chars().collect();
    let mut num_quotes = 0;

    loop {
        if let Some(zeroth) = chars.pop_front() {
            taken.push(zeroth);
            if zeroth == '"' {
                num_quotes += 1;
                if num_quotes == 2 {
                    let taken_str = s.strip_prefix(taken.as_str()).unwrap();
                    return Some((Arc::from(taken.as_str()), taken_str));
                }
            }
        } else {
            return None;
        }
    }
}

pub trait Tail<T> {
    fn tail(&self) -> &[T];
}

impl<T> Tail<T> for [T] {
    fn tail(&self) -> &[T] {
        return &self[1..];
    }
}
