pub struct LinedStr<'a> {
    line_cap: usize,
    string: &'a str,
    start_index: usize,
    end_index: usize,
}

impl<'a> LinedStr<'a> {
    pub fn as_whole_str(&self) -> &'a str {
        self.string.as_ref()
    }

    pub fn as_str(&self) -> &'a str {
        &self.string[self.start_index..]
    }

    pub fn line_capicity(&self) -> usize {
        self.line_cap
    }
}

impl<'a> Iterator for LinedStr<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.string[self.start_index..].chars();
        while let Some(ch) = chars.next() {
            self.end_index += 1;
            if ch == '\n' {
                let start = self.start_index;
                self.start_index = self.end_index;
                return Some(&self.string[start..self.end_index - 1]);
            } else if self.end_index - self.start_index >= self.line_cap {
                let start = self.start_index;
                self.start_index = self.end_index;
                return Some(&self.string[start..self.end_index]);
            }
        }
        if self.start_index != self.end_index {
            let start = self.start_index;
            self.start_index = self.end_index;
            Some(&self.string[start..])
        } else {
            None
        }
    }
}

pub fn str_to_lines<'a, S>(string: &'a S, line_cap: usize) -> LinedStr<'a>
where
    S: AsRef<str> + ?Sized,
{
    LinedStr {
        line_cap,
        string: string.as_ref(),
        start_index: 0,
        end_index: 0,
    }
}

#[test]
fn test_str_to_lines() {
    let test_string = "123456\n123456789\n12345678901234567";
    assert_eq!(
        str_to_lines(test_string, 8).collect::<Vec<&str>>(),
        &["123456", "12345678", "9", "12345678", "90123456", "7"]
    );
}
