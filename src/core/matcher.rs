use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

/// FZF 风格的模糊匹配评分
/// 返回得分：0 表示不匹配，正数表示匹配程度
pub fn fuzzy_score(input: &str, target: &str) -> i64 {
    let matcher = SkimMatcherV2::default();
    matcher.fuzzy_match(target, input).unwrap_or(0)
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub path: String,
    pub score: i64,
}

pub fn fuzzy_match(input: &str, candidates: &[&str]) -> Vec<MatchResult> {
    let matcher = SkimMatcherV2::default();
    let mut results: Vec<MatchResult> = candidates
        .iter()
        .filter_map(|&path| {
            matcher.fuzzy_match(path, input).map(|score| MatchResult {
                path: path.to_string(),
                score,
            })
        })
        .collect();

    results.sort_by(|a, b| b.score.cmp(&a.score));
    results
}

/// 带频率加成的模糊匹配（备用，目前用于测试验证）
#[allow(dead_code)]
pub fn fuzzy_match_with_frequency(
    input: &str,
    candidates: &[(&str, u32)],
) -> Vec<MatchResult> {
    let matcher = SkimMatcherV2::default();
    let mut results: Vec<MatchResult> = candidates
        .iter()
        .filter_map(|&(path, freq)| {
            matcher.fuzzy_match(path, input).map(|score| {
                // 频率加成：访问次数越多，额外加分越多（最多 +100）
                let freq_bonus = ((freq as f64).log2() * 10.0) as i64;
                let total_score = score + freq_bonus;
                MatchResult {
                    path: path.to_string(),
                    score: total_score,
                }
            })
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
        let exact = fuzzy_score("project", "project");
        let partial = fuzzy_score("proj", "project");
        assert!(exact > partial, "exact={}, partial={}", exact, partial);
    }

    #[test]
    fn test_consecutive_match_bonus() {
        let consecutive = fuzzy_score("pro", "project");
        let scattered = fuzzy_score("prj", "project");
        assert!(
            consecutive > scattered,
            "consecutive={}, scattered={}",
            consecutive,
            scattered
        );
    }

    #[test]
    fn test_no_match() {
        let score = fuzzy_score("xyz", "project");
        assert_eq!(score, 0);
    }

    #[test]
    fn test_camel_case_match() {
        let score = fuzzy_score("MP", "MyProject");
        assert!(score > 0, "score={}", score);
    }

    #[test]
    fn test_frequency_bonus() {
        // 相同匹配下，频率高的应该得分更高
        let candidates = vec![
            ("/path/a", 1u32),
            ("/path/b", 10),
            ("/path/c", 100),
        ];
        let results = fuzzy_match_with_frequency("path", &candidates);
        assert!(!results.is_empty());
        // 频率最高的应该在前面
        assert_eq!(results[0].path, "/path/c");
        assert_eq!(results[1].path, "/path/b");
        assert_eq!(results[2].path, "/path/a");
    }
}
