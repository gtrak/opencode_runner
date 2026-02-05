use opencode_runner::reviewer;
use opencode_runner::reviewer::{
    ReviewerAction, ReviewerClient, ReviewerContext, ReviewerDecision,
};

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test context
    fn create_test_context(
        task: &str,
        iteration: usize,
        previous_summaries: Vec<String>,
        current_sample: &str,
    ) -> ReviewerContext {
        ReviewerContext {
            task_description: task.to_string(),
            iteration,
            previous_summaries,
            current_sample: current_sample.to_string(),
        }
    }

    #[test]
    fn test_reviewer_context_creation() {
        let context = create_test_context(
            "Write a hello world program",
            1,
            vec!["Initial assessment: Continue".to_string()],
            "Generating code...\nChecking syntax...\nDone!",
        );

        assert_eq!(context.task_description, "Write a hello world program");
        assert_eq!(context.iteration, 1);
        assert_eq!(context.previous_summaries.len(), 1);
        assert_eq!(
            context.current_sample,
            "Generating code...\nChecking syntax...\nDone!"
        );
    }

    #[test]
    fn test_prompt_no_previous_summaries() {
        let context = create_test_context("Test task", 1, vec![], "Sample output text");

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let prompt = client.build_prompt(&context);
        assert!(prompt.contains("No previous assessments."));
        assert!(prompt.contains("Test task"));
        assert!(prompt.contains("Sample output text"));
    }

    #[test]
    fn test_prompt_with_previous_summaries() {
        let context = create_test_context(
            "Test task",
            2,
            vec![
                "Iteration 1: Making progress".to_string(),
                "Iteration 2: Continue".to_string(),
            ],
            "More output",
        );

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let prompt = client.build_prompt(&context);
        assert!(!prompt.contains("No previous assessments."));
        assert!(prompt.contains("1. Iteration 1: Making progress"));
        assert!(prompt.contains("2. Iteration 2: Continue"));
    }

    #[test]
    fn test_parse_continue_decision() {
        let json = r#"{"action": "continue", "reason": "Making progress"}"#;
        let decision: ReviewerDecision = serde_json::from_str(json).unwrap();

        match decision.action {
            ReviewerAction::Continue => assert!(true),
            ReviewerAction::Abort => panic!("Expected Continue, got Abort"),
        }
        assert_eq!(decision.reason, "Making progress");
    }

    #[test]
    fn test_parse_abort_decision() {
        let json = r#"{"action": "abort", "reason": "Stuck in loop"}"#;
        let decision: ReviewerDecision = serde_json::from_str(json).unwrap();

        match decision.action {
            ReviewerAction::Abort => assert!(true),
            ReviewerAction::Continue => panic!("Expected Abort, got Continue"),
        }
        assert_eq!(decision.reason, "Stuck in loop");
    }

    #[test]
    fn test_parse_invalid_decision() {
        let json = r#"{"action": "invalid", "reason": "Unknown"}"#;
        let result = serde_json::from_str::<ReviewerDecision>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_decision_summary_continue() {
        let decision = ReviewerDecision {
            action: ReviewerAction::Continue,
            reason: "Making progress".to_string(),
        };

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let summary = ReviewerClient::format_decision_summary(&decision, 3);
        assert!(summary.contains("Iter 3: Continue"));
        assert!(summary.contains("Making progress"));
    }

    #[test]
    fn test_format_decision_summary_abort() {
        let decision = ReviewerDecision {
            action: ReviewerAction::Abort,
            reason: "Looping indefinitely".to_string(),
        };

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let summary = ReviewerClient::format_decision_summary(&decision, 5);
        assert!(summary.contains("Iter 5: Abort"));
        assert!(summary.contains("Looping indefinitely"));
    }

    #[test]
    fn test_client_creation_default() {
        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());

        assert_eq!(client.base_url, "http://localhost:11434");
        assert_eq!(client.model, "llama3");
        assert_eq!(client.max_retries, 3);
    }

    #[test]
    fn test_client_creation_custom_retries() {
        let mut client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());

        client.max_retries = 5;
        assert_eq!(client.max_retries, 5);
    }

    #[test]
    fn test_build_prompt_with_multiline_output() {
        let context =
            create_test_context("Complex task", 1, vec![], "Line 1\nLine 2\nLine 3\nLine 4");

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let prompt = client.build_prompt(&context);
        assert!(prompt.contains("Line 1"));
        assert!(prompt.contains("Line 2"));
        assert!(prompt.contains("Line 3"));
        assert!(prompt.contains("Line 4"));
    }

    #[test]
    fn test_build_prompt_with_emojis() {
        let context = create_test_context(
            "Task with emojis",
            1,
            vec![],
            "✓ Fixed bug\n⏳ Still working\n🔍 Looking for issue",
        );

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let prompt = client.build_prompt(&context);
        assert!(prompt.contains("✓ Fixed bug"));
        assert!(prompt.contains("⏳ Still working"));
        assert!(prompt.contains("🔍 Looking for issue"));
    }

    #[test]
    fn test_build_prompt_with_code_blocks() {
        let context = create_test_context(
            "Task",
            1,
            vec![],
            "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```",
        );

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let prompt = client.build_prompt(&context);
        assert!(prompt.contains("```rust"));
        assert!(prompt.contains("fn main()"));
    }

    #[test]
    fn test_build_prompt_empty_current_sample() {
        let context = create_test_context("Task", 1, vec![], "");

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let prompt = client.build_prompt(&context);
        assert!(prompt.contains("Current output (last 0 lines):"));
        assert!(prompt.contains("```\n"));
    }

    #[test]
    fn test_build_prompt_large_iteration_number() {
        let context = create_test_context("Task", 100, vec![], "Output");

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let prompt = client.build_prompt(&context);
        assert!(prompt.contains("Current iteration: 100"));
    }

    #[test]
    fn test_build_prompt_large_previous_summaries() {
        let summaries: Vec<String> = (1..=20)
            .map(|i| format!("Iteration {}: Continue", i))
            .collect();

        let context = create_test_context("Task", 1, summaries, "Output");

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let prompt = client.build_prompt(&context);
        assert!(prompt.contains("20. Iteration 20: Continue"));
    }

    #[test]
    fn test_build_prompt_mixed_previous_summaries() {
        let context = create_test_context(
            "Task",
            1,
            vec![
                "Good progress".to_string(),
                "Still working".to_string(),
                "Continue".to_string(),
                "Okay".to_string(),
                "Keep going".to_string(),
            ],
            "Output",
        );

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let prompt = client.build_prompt(&context);
        assert!(prompt.contains("1. Good progress"));
        assert!(prompt.contains("2. Still working"));
        assert!(prompt.contains("3. Continue"));
        assert!(prompt.contains("4. Okay"));
        assert!(prompt.contains("5. Keep going"));
    }

    #[test]
    fn test_parse_complex_json_decision() {
        let json = r#"{
  "action": "continue",
  "reason": "The assistant is making meaningful progress by implementing the feature."
}"#;
        let decision: ReviewerDecision = serde_json::from_str(json).unwrap();

        assert_eq!(decision.action, ReviewerAction::Continue);
        assert!(decision.reason.contains("meaningful progress"));
    }

    #[test]
    fn test_parse_json_with_extra_whitespace() {
        let json = r#"{
  "action": "abort",
  "reason": "The assistant is repeating the same code blocks."
}"#;
        let decision: ReviewerDecision = serde_json::from_str(json).unwrap();

        assert_eq!(decision.action, ReviewerAction::Abort);
        assert_eq!(
            decision.reason,
            "The assistant is repeating the same code blocks."
        );
    }

    #[test]
    fn test_context_default_values() {
        let context = ReviewerContext {
            task_description: String::new(),
            iteration: 0,
            previous_summaries: vec![],
            current_sample: String::new(),
        };

        assert_eq!(context.task_description, "");
        assert_eq!(context.iteration, 0);
        assert!(context.previous_summaries.is_empty());
        assert_eq!(context.current_sample, "");
    }

    #[test]
    fn test_context_with_special_characters() {
        let context = create_test_context(
            "Task with <special> characters & symbols",
            1,
            vec![],
            "Output with special chars <>&\"'",
        );

        assert_eq!(
            context.task_description,
            "Task with <special> characters & symbols"
        );
        assert_eq!(context.current_sample, "Output with special chars <>&\"'");
    }

    #[test]
    fn test_context_with_unicode() {
        let context = create_test_context("日本語のタスク", 1, vec![], "出力: こんにちは世界");

        assert_eq!(context.task_description, "日本語のタスク");
        assert_eq!(context.current_sample, "出力: こんにちは世界");
    }

    #[test]
    fn test_format_decision_summary_reason_variations() {
        let short_reason = ReviewerDecision {
            action: ReviewerAction::Continue,
            reason: "Good".to_string(),
        };

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let summary = ReviewerClient::format_decision_summary(&short_reason, 1);
        assert_eq!(summary, "Iter 1: Continue - Good");

        let long_reason = ReviewerDecision {
            action: ReviewerAction::Abort,
            reason: "This is a very long reason that contains many details about why the assistant is stuck in a loop".to_string(),
        };

        let client =
            ReviewerClient::new("http://localhost:11434".to_string(), "llama3".to_string());
        let summary = ReviewerClient::format_decision_summary(&long_reason, 5);
        assert!(summary.contains("Iter 5: Abort"));
        assert!(summary.contains("This is a very long reason that contains many details about why the assistant is stuck in a loop"));
    }
}
