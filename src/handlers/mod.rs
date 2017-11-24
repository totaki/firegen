pub mod react;

pub trait BaseHandler {

    fn compare_strings(s: &str, t: &str) -> bool {
        s == t
    }

    fn is_current(&self, s: &str) -> bool;
}

