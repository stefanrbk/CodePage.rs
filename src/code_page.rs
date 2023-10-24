pub trait CodePage {
    fn to_utf8(string: &Vec<u8>) -> String;
    fn from_utf8(utf8: String) -> Vec<u8>;
}
