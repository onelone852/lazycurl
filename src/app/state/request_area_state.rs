use reqwest::Method;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RequestAreaEditing {
    #[default]
    None,
    Link,
}

#[derive(Debug)]
pub struct RequestAreaState {
    methods: Vec<Box<str>>,
    method_index: usize,
    pub is_editing: RequestAreaEditing,
    pub link: String,
    request_item_num: usize,
}

impl Default for RequestAreaState {
    fn default() -> Self {
        let methods = vec!["GET".into(), "POST".into(), "PUT".into(), "DELETE".into()];
        Self {
            methods,
            method_index: 0,
            is_editing: RequestAreaEditing::None,
            link: "https://www.example.com".to_string(),
            request_item_num: 1,
        }
    }
}

impl RequestAreaState {
    #[inline]
    pub fn methods(&self) -> &[Box<str>] {
        &self.methods
    }

    #[inline]
    pub fn method_index(&self) -> usize {
        self.method_index
    }

    #[inline]
    pub fn get_request_item_num(&mut self) -> usize {
        let request_item_num = self.request_item_num;
        self.request_item_num += 1;
        request_item_num
    }

    #[inline]
    pub fn next_method(&mut self) {
        self.method_index += 1;
        if self.method_index >= self.methods.len() {
            self.method_index = 0;
        }
    }

    #[inline]
    pub fn prev_method(&mut self) {
        if self.method_index == 0 {
            self.method_index = self.methods.len();
        }
        self.method_index -= 1;
    }

    #[inline]
    pub fn reset_editing(&mut self) {
        self.is_editing = RequestAreaEditing::None;
    }

    #[inline]
    pub fn get_method(&self) -> Method {
        Method::from_bytes(
            self.methods
                .get(self.method_index)
                .expect("Index should be in methods vector")
                .as_bytes(),
        )
        .expect("Should not be empty string")
    }
}
