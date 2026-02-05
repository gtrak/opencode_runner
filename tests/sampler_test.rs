use opencode_runner::sampler::{Sampler, SamplerEvent};

#[cfg(test)]
mod tests {
    use super::*;

    // Test basic sampler functionality
    #[test]
    fn test_sampler_basic() {
        let mut sampler = Sampler::new(5);

        sampler.add_line("Line 1");
        sampler.add_line("Line 2");
        sampler.add_line("Line 3");

        assert_eq!(sampler.line_count(), 3);
        let sample = sampler.sample();
        assert_eq!(sample, "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_sampler_overflow() {
        let mut sampler = Sampler::new(3);

        sampler.add_line("Line 1");
        sampler.add_line("Line 2");
        sampler.add_line("Line 3");
        sampler.add_line("Line 4");
        sampler.add_line("Line 5");

        assert_eq!(sampler.line_count(), 3);
        let sample = sampler.sample();
        // With FIFO eviction, after adding 5 lines to a buffer of 3:
        // - Lines 1, 2 are evicted (oldest)
        // - Lines 3, 4, 5 remain
        assert_eq!(sample, "Line 3\nLine 4\nLine 5");
    }

    #[test]
    fn test_sampler_empty_lines() {
        let mut sampler = Sampler::new(5);

        sampler.add_line("Line 1");
        sampler.add_line("");
        sampler.add_line("   ");
        sampler.add_line("Line 2");

        assert_eq!(sampler.line_count(), 2);
    }

    #[test]
    fn test_sampler_clear() {
        let mut sampler = Sampler::new(5);

        sampler.add_line("Line 1");
        sampler.add_line("Line 2");
        assert_eq!(sampler.line_count(), 2);

        sampler.clear();
        assert_eq!(sampler.line_count(), 0);
        assert!(sampler.sample().is_empty());
    }

    // Test whitespace trimming behavior
    #[test]
    fn test_sampler_whitespace_trimmed() {
        let mut sampler = Sampler::new(10);

        sampler.add_line("  Leading spaces");
        sampler.add_line("Trailing spaces  ");
        sampler.add_line("Multiple   spaces between words");

        assert_eq!(sampler.line_count(), 3);
        let sample = sampler.sample();
        assert!(sample.contains("Leading spaces"));
        assert!(sample.contains("Trailing spaces"));
        assert!(sample.contains("spaces between words"));
        assert!(!sample.contains("  Leading spaces")); // Should be trimmed
        assert!(!sample.contains("Trailing spaces  ")); // Should be trimmed
    }

    // Test buffer overflow behavior
    #[test]
    fn test_sampler_buffer_overflow() {
        let mut sampler = Sampler::new(3);

        // Add lines one by one to overflow buffer
        for i in 1..=10 {
            sampler.add_line(&format!("Line {}", i));
        }

        assert_eq!(
            sampler.line_count(),
            3,
            "Buffer should have exactly 3 lines"
        );
        // With FIFO eviction, after adding 10 lines to a buffer of 3:
        // - Lines 1, 2, 3, 4 are evicted (oldest)
        // - Lines 5, 6, 7, 8, 9, 10 remain
        assert_eq!(sampler.sample(), "Line 8\nLine 9\nLine 10");
    }

    // Test max_lines preservation with overflow
    #[test]
    fn test_sampler_max_lines_preservation() {
        let mut sampler = Sampler::new(5);

        // Add exactly max_lines
        for i in 1..=5 {
            sampler.add_line(&format!("Line {}", i));
        }

        assert_eq!(sampler.line_count(), 5);
        let sample = sampler.sample();
        assert_eq!(sample, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5");

        // Add one more line (should overflow - evict Line 1)
        sampler.add_line("Line 6");
        assert_eq!(sampler.line_count(), 5);
        // After overflow, Line 1 is evicted, so buffer has: Line 2, Line 3, Line 4, Line 5, Line 6
        assert_eq!(sampler.sample(), "Line 2\nLine 3\nLine 4\nLine 5\nLine 6");
    }

    // Test complex text processing with multi-line input
    #[test]
    fn test_sampler_complex_text() {
        let mut sampler = Sampler::new(20);

        let text = "Line 1: Start of processing\nLine 2: Intermediate step\nLine 3: Final result\nLine 4: Additional notes";
        sampler.add_lines(&text);

        assert_eq!(sampler.line_count(), 4);
        assert_eq!(sampler.sample(), "Line 1: Start of processing\nLine 2: Intermediate step\nLine 3: Final result\nLine 4: Additional notes");
    }

    // Test empty buffer
    #[test]
    fn test_sampler_empty_buffer() {
        let sampler = Sampler::new(5);
        assert_eq!(sampler.line_count(), 0);
        assert!(sampler.sample().is_empty());
    }

    // Test single line buffer
    #[test]
    fn test_sampler_single_line() {
        let mut sampler = Sampler::new(5);
        sampler.add_line("Single line");
        assert_eq!(sampler.line_count(), 1);
        assert!(sampler.sample().contains("Single line"));
    }

    // Test repeated lines
    #[test]
    fn test_sampler_repeated_lines() {
        let mut sampler = Sampler::new(5);
        sampler.add_line("Repeated");
        sampler.add_line("Repeated");
        sampler.add_line("Repeated");

        assert_eq!(sampler.line_count(), 3);
        let sample = sampler.sample();
        assert!(sample.contains("Repeated"));
        assert_eq!(sample.matches("Repeated").count(), 3);
    }

    // Test special characters
    #[test]
    fn test_sampler_special_characters() {
        let mut sampler = Sampler::new(5);
        sampler.add_line("Line with special chars: @#$%^&*()");
        sampler.add_line("Line with emojis: 🎉🎉🎉");
        sampler.add_line("Line with unicode: 你好世界");

        assert_eq!(sampler.line_count(), 3);
        let sample = sampler.sample();
        assert!(sample.contains("special chars"));
        assert!(sample.contains("emojis"));
        assert!(sample.contains("unicode"));
    }
}
