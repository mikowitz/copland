#[derive(Debug, Default)]
pub struct Fill {
    tie_across_boundaries: bool,
    mask: Vec<u32>,
}

impl Fill {
    fn new(tie_across_boundaries: bool, mask: Vec<u32>) -> Self {
        let mask = mask.to_vec();
        Self {
            tie_across_boundaries,
            mask,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let generator = Fill::default();

        assert!(!generator.tie_across_boundaries);
        assert!(generator.mask.is_empty());
    }

    #[test]
    fn new() {
        let generator = Fill::new(true, vec![1, 0]);

        assert!(generator.tie_across_boundaries);
        assert_eq!(generator.mask, vec![1, 0]);
    }
}
