use std::collections::VecDeque;

use crate::sampler::Sampler;

#[cfg(test)]
mod tests {
    use super::*;

    // Platform-specific imports for testing
    use opencode_rs::types::event::Event as TestEvent;

    // Test basic sampler functionality
    #[test]
    fn test_sampler_basic() {
        let mut sampler = Sampler::new(5);

        sampler.add_line("Line 1");
        sampler.add_line("Line 2");
        sampler.add_line("Line 3");

        assert_eq!(sampler.line_count(), 3);
        assert!(sampler.sample().contains("Line 1"));
        assert!(sampler.sample().contains("Line 2"));
        assert!(sampler.sample().contains("Line 3"));
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
        assert!(!sample.contains("Line 1")); // Should be evicted
        assert!(!sample.contains("Line 2")); // Should be evicted
        assert!(sample.contains("Line 3"));
        assert!(sample.contains("Line 4"));
        assert!(sample.contains("Line 5"));
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

    // Test PartAdded event (text content capture)
    #[test]
    fn test_sampler_add_line() {
        let mut sampler = Sampler::new(10);
        sampler.add_line("Test content");
        assert_eq!(sampler.line_count(), 1);
        assert!(sampler.sample().contains("Test content"));
    }

    // Test Event filtering and processing
    #[test]
    fn test_sampler_process_event_part_added() {
        let mut sampler = Sampler::new(10);
        let event = SamplerEvent::PartAdded {
            text: "This is a test message".to_string(),
        };
        sampler.process_event(&event);

        let sample = sampler.sample();
        assert!(sample.contains("This is a test message"));
        assert_eq!(sampler.line_count(), 1);
    }

    #[test]
    fn test_sampler_process_event_part_updated() {
        let mut sampler = Sampler::new(10);
        let event = SamplerEvent::PartUpdated {
            delta: "Updated content\nAdditional line".to_string(),
        };
        sampler.process_event(&event);

        let sample = sampler.sample();
        assert!(sample.contains("Updated content"));
        assert!(sample.contains("Additional line"));
        assert_eq!(sampler.line_count(), 2);
    }

    #[test]
    fn test_sampler_process_event_tool_call() {
        let mut sampler = Sampler::new(10);
        let event = SamplerEvent::ToolCall {
            name: "grep".to_string(),
            params: serde_json::json!({"pattern": "test"}),
        };
        sampler.process_event(&event);

        let sample = sampler.sample();
        assert!(sample.contains("[Tool: grep"));
    }

    #[test]
    fn test_sampler_process_event_tool_result_skipped() {
        let mut sampler = Sampler::new(10);
        let event = SamplerEvent::ToolResult {
            result: serde_json::json!("very verbose output..."),
        };
        sampler.process_event(&event);

        // Tool results should not appear in sample
        let sample = sampler.sample();
        assert!(!sample.contains("very verbose output"));
        assert_eq!(sampler.line_count(), 0);
    }

    #[test]
    fn test_sampler_process_event_error() {
        let mut sampler = Sampler::new(10);
        let event = SamplerEvent::Error {
            error: "Some error occurred".to_string(),
        };
        sampler.process_event(&event);

        let sample = sampler.sample();
        assert!(sample.contains("[Error: Some error occurred]"));
    }

    #[test]
    fn test_sampler_process_event_thinking_skipped() {
        let mut sampler = Sampler::new(10);
        let event = SamplerEvent::Thinking {
            thought: "Let me think about this...".to_string(),
        };
        sampler.process_event(&event);

        let sample = sampler.sample();
        assert!(!sample.contains("Let me think about this..."));
        assert_eq!(sampler.line_count(), 0);
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

        for i in 1..=10 {
            sampler.add_line(format!("Line {}", i));
        }

        assert_eq!(sampler.line_count(), 3);
        let sample = sampler.sample();
        // Lines 1-8 should be evicted, only 9 and 10 remain
        assert!(!sample.contains("Line 1"));
        assert!(!sample.contains("Line 2"));
        assert!(!sample.contains("Line 3"));
        assert!(!sample.contains("Line 4"));
        assert!(!sample.contains("Line 5"));
        assert!(!sample.contains("Line 6"));
        assert!(!sample.contains("Line 7"));
        assert!(!sample.contains("Line 8"));
        assert!(sample.contains("Line 9"));
        assert!(sample.contains("Line 10"));
    }

    // Test max_lines preservation with overflow
    #[test]
    fn test_sampler_max_lines_preservation() {
        let mut sampler = Sampler::new(5);

        // Add exactly max_lines
        for i in 1..=5 {
            sampler.add_line(format!("Line {}", i));
        }

        assert_eq!(sampler.line_count(), 5);
        let sample = sampler.sample();
        assert_eq!(sample.lines().count(), 5);

        // Add one more line (should overflow)
        sampler.add_line("Line 6");
        assert_eq!(sampler.line_count(), 5);
        assert!(!sample.contains("Line 1")); // Line 1 should be evicted
        assert!(sample.contains("Line 2"));
        assert!(sample.contains("Line 6"));
    }

    // Test complex text processing with multi-line input
    #[test]
    fn test_sampler_complex_text() {
        let mut sampler = Sampler::new(20);

        let text = "Line 1: Start of processing
Line 2: Intermediate step
Line 3: Final result
Line 4: Additional notes";
        sampler.add_line(text.to_string());

        assert_eq!(sampler.line_count(), 4);
        let sample = sampler.sample();
        assert!(sample.contains("Line 1: Start of processing"));
        assert!(sample.contains("Line 2: Intermediate step"));
        assert!(sample.contains("Line 3: Final result"));
        assert!(sample.contains("Line 4: Additional notes"));
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
