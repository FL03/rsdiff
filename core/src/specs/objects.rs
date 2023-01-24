/*
    Appellation: objects <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

///
pub trait BaseObject {
    fn count(&self) -> usize;
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_ascii_lowercase()
    }
    fn symbol(&self) -> String;
}
