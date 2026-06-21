use std::fs;

#[derive(Debug)]
struct Violation {
    line: usize,
    message: String,
}

struct Linter {
    rules: Vec<Box<dyn Fn(&str) -> Vec<Violation>>>,
    fixers: Vec<Box<dyn Fn(&mut String) -> bool>>,
}

impl Linter {
    fn new() -> Self {
        Linter {
            rules: Vec::new(),
            fixers: Vec::new(),
        }
    }

    fn add_rule<F>(&mut self, f: F)
    where
        F: Fn(&str) -> Vec<Violation> + 'static,
    {
        self.rules.push(Box::new(f));
    }

    fn add_fixer<F>(&mut self, f: F)
    where
        F: Fn(&mut String) -> bool + 'static,
    {
        self.fixers.push(Box::new(f));
    }

    fn check(&self, content: &str) -> Vec<Violation> {
        let mut all_violations = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            for rule in &self.rules {
                let violations = rule(line);
                for v in violations {
                    all_violations.push(Violation {
                        line: line_num + 1,
                        message: v.message,
                    });
                }
            }
        }
        all_violations
    }

    fn fix(&self, content: &mut String) -> usize {
        let mut fixed_count = 0;
        for fixer in &self.fixers {
            if fixer(content) {
                fixed_count += 1;
            }
        }
        fixed_count
    }
}

fn main() {
    let mut linter = Linter::new();

    // rule 1: check for trailing whitespaces
    linter.add_rule(|line| {
        let mut violations = Vec::new();
        if line.len() > line.trim_end().len() {
            violations.push(Violation {
                line: 0,
                message: "Trailing whitespace".to_string(),
            });
        }
        violations
    });

    // rule 2: check for lines longer that 80 chars
    linter.add_rule(|line| {
        let mut violations = Vec::new();
        if line.len() > 80 {
            violations.push(Violation {
                line: 0,
                message: format!("Line too long ({} characters)", line.len()),
            });
        }
        violations
    });

    // rule 3: Check for hard tabs
    linter.add_rule(|line| {
        let mut violations = Vec::new();
        if line.contains('\t') {
            violations.push(Violation {
                line: 0,
                message: "Hard tab detected, use spaces".to_string(),
            });
        }
        violations
    });

    // rule 4: Check for TODO and FIXME comments
    linter.add_rule(|line| {
        let mut violations = Vec::new();
        if line.contains("TODO") || line.contains("FIXME") {
            violations.push(Violation {
                line: 0,
                message: "Contains TODO or FIXME marker".to_string(),
            });
        }
        violations
    });

    // fixer: removes trailing whitespaces
    linter.add_fixer(|input| {
        let original = input.clone();
        *input = input
            .lines()
            .map(|l| l.trim_end())
            .collect::<Vec<_>>()
            .join("\n");

        if original.ends_with('\n') {
            input.push('\n');
        }

        *input != original
    });

    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: cargo run -- <file>");
        return;
    };
    let mut content = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", file_path, e);
            return;
        }
    };
    let violations = linter.check(&content);
    if violations.is_empty() {
        println!("No violations found.");
    } else {
        for v in &violations {
            println!("{}:{} {}", file_path, v.line, v.message);
        }
        println!("\n{} violation(s) found.", violations.len());
    }
    let fixed_count = linter.fix(&mut content);
    if fixed_count > 0 {
        println!("\n{} fixer(s) applied.", fixed_count);
        match fs::write(file_path, &content) {
            Ok(()) => println!("File updated."),
            Err(e) => eprintln!("Error writing {}: {}", file_path, e),
        }
    }
}
