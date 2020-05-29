pub(crate) struct GameCell {}

impl GameCell {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_occupied(&self) -> bool {
        false
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_is_occupied() {}
}
