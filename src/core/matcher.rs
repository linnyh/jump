/// FZF 风格的模糊匹配评分
/// 返回得分：0 表示不匹配，正数表示匹配程度
pub fn fuzzy_score(input: &str, target: &str) -> u32 {
    let input_chars: Vec<char> = input.to_lowercase().chars().collect();
    let target_chars: Vec<char> = target.to_lowercase().chars().collect();

    if input_chars.is_empty() {
        return 0;
    }

    let mut score: u32 = 0;
    let mut input_idx = 0;
    let mut prev_match_idx: Option<usize> = None;
    let mut consecutive_bonus = 0;

    for (target_idx, tc) in target_chars.iter().enumerate() {
        if input_idx >= input_chars.len() {
            break;
        }

        if *tc == input_chars[input_idx] {
            score += 10;
            input_idx += 1;

            if let Some(prev) = prev_match_idx {
                if target_idx == prev + 1 {
                    consecutive_bonus += 5;
                    score += consecutive_bonus;
                } else {
                    consecutive_bonus = 0;
                }
            }

            if target_idx == 0 {
                score += 15;
            }

            if target_idx > 0 && target_chars[target_idx - 1] == '/' {
                score += 8;
            }

            prev_match_idx = Some(target_idx);
        }
    }

    if input_idx < input_chars.len() {
        return 0;
    }

    if target.len() > input.len() * 3 {
        score = score.saturating_sub(5);
    }

    score
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub path: String,
    pub score: u32,
}

pub fn fuzzy_match(input: &str, candidates: &[&str]) -> Vec<MatchResult> {
    let mut results: Vec<MatchResult> = candidates
        .iter()
        .filter_map(|&path| {
            let score = fuzzy_score(input, path);
            if score > 0 {
                Some(MatchResult {
                    path: path.to_string(),
                    score,
                })
            } else {
                None
            }
        })
        .collect();

    results.sort_by(|a, b| b.score.cmp(&a.score));
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match_high_score() {
        let score = fuzzy_score("project", "project");
        let partial = fuzzy_score("proj", "project");
        assert!(score > partial);
    }

    #[test]
    fn test_consecutive_match_bonus() {
        let consecutive = fuzzy_score("pro", "project");
        let scattered = fuzzy_score("prj", "project");
        assert!(consecutive > scattered);
    }

    #[test]
    fn test_no_match() {
        let score = fuzzy_score("xyz", "project");
        assert_eq!(score, 0);
    }

    #[test]
    fn test_camel_case_match() {
        let score = fuzzy_score("MP", "MyProject");
        assert!(score > 0);
    }
}
