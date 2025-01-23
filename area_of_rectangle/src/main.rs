#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self) -> bool{
        self.width > other.width && self.height >other.height
    }
  }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_hold_edge_cases() {
        let small_rect = Rectangle { width: 10, height: 5 };
        let large_rect = Rectangle { width: 20, height: 10 };
        let equal_rect = Rectangle { width: 10, height: 5 };

        // Large rectangle should not fit inside small rectangle
        assert!(!large_rect.can_hold(&small_rect));

        // Small rectangle should fit inside itself
        assert!(small_rect.can_hold(&small_rect));

        // Equal rectangle should fit inside itself
        assert!(equal_rect.can_hold(&equal_rect));

        // Small rectangle should fit inside large rectangle
        assert!(small_rect.can_hold(&large_rect));

        // Large rectangle should not fit inside equal rectangle
        assert!(!large_rect.can_hold(&equal_rect));
    }
}