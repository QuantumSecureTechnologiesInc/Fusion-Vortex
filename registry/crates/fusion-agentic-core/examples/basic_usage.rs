//! Example: Using the agentic core for problem-solving

use fusion_agentic_core::{AgenticCore, Result};

fn main() -> Result<()> {
    println!("=== Fusion Agentic Core Example ===\n");

    // Create the agentic core
    let core = AgenticCore::new();

    // Example 1: Process a complex problem
    println!("Example 1: Problem-Solving with Agentic Reasoning");
    println!("---");

    let problem = "Design a high-performance caching system for a distributed application";
    println!("Problem: {}", problem);

    match core.process_problem(problem) {
        Ok(solution) => {
            println!("Solution:\n{}\n", solution);
        }
        Err(e) => {
            println!("Error: {}\n", e);
        }
    }

    // Example 2: Vibe coding
    println!("Example 2: Vibe Coding");
    println!("---");

    let intent = "filter even numbers and transform them";
    println!("Intent: {}", intent);

    match core.vibe_code(intent) {
        Ok(code) => {
            println!("Generated Code:\n{}\n", code);
        }
        Err(e) => {
            println!("Error: {}\n", e);
        }
    }

    // Example 3: Code excellence check
    println!("Example 3: Code Excellence Analysis");
    println!("---");

    let sample_code = r#"
/// Calculate the factorial of a number
fn factorial(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
    }
}
"#;

    match core.check_excellence(sample_code) {
        Ok(metrics) => {
            println!("Quality Metrics:");
            println!("  Overall Score: {:.1}", metrics.overall_score);
            println!("  Readability: {:.1}", metrics.readability);
            println!("  Maintainability: {:.1}", metrics.maintainability);
            println!("  Performance: {:.1}", metrics.performance);
            println!("  Security: {:.1}", metrics.security);
            println!("  Test Coverage: {:.1}", metrics.test_coverage);
            println!("  Documentation: {:.1}", metrics.documentation);

            if !metrics.recommendations.is_empty() {
                println!("\nRecommendations:");
                for rec in &metrics.recommendations {
                    println!("  - {}", rec);
                }
            }

            if metrics.is_excellent() {
                println!("\n✨ Code quality is EXCELLENT!");
            } else if metrics.is_good() {
                println!("\n✅ Code quality is GOOD!");
            } else if metrics.needs_improvement() {
                println!("\n⚠️  Code needs improvement");
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Example 4: Iterative refinement
    println!("\nExample 4: Iterative Solution Refinement");
    println!("---");

    let initial = "fn add(a: i32, b: i32) -> i32 { a + b }";
    let feedback = "Make it generic and add documentation";

    println!("Initial: {}", initial);
    println!("Feedback: {}", feedback);

    match core.iterate_solution(initial, feedback) {
        Ok(refined) => {
            println!("Refined:\n{}\n", refined);
        }
        Err(e) => {
            println!("Error: {}\n", e);
        }
    }

    println!("=== Example Complete ===");

    Ok(())
}
