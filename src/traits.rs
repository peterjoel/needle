
pub trait Search<'a>{
    fn first_index_of(&self, haystack: &'a [u8]) -> Option<usize>;
}
