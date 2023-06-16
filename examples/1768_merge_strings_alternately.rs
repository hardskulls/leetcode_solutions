fn main() {}

fn solution(word_1: impl AsRef<str>, word_2: impl AsRef<str>) -> String {
    let (word_1, word_2) = (word_1.as_ref(), word_2.as_ref());

    let mut buffer = String::with_capacity(word_1.len() + word_2.len());
    let max_length = std::cmp::max(word_1.len(), word_2.len());

    for i in 0..max_length {
        let next = i + 1;

        if let Some(s) = word_1.get(i..next) {
            buffer.push_str(s)
        }

        if let Some(s) = word_2.get(i..next) {
            buffer.push_str(s)
        }
    }

    buffer
}

#[cfg(test)]
mod tests {
    use crate::solution;
    use leetcode_solutions::benchmarking::bench_times;

    #[test]
    fn solution_test() {
        let (words_1, words_2) = ("a", "b");
        let expect = "ab".to_owned();
        let res = solution(words_1, words_2);
        assert_eq!(res, expect);

        let (words_1, words_2) = ("abc", "pq");
        let expect = "apbqc".to_owned();
        let res = solution(words_1, words_2);
        assert_eq!(res, expect);

        let (words_1, words_2) = (
            "khgkutdyrstrdjytgfkjhjk",
            "lohulikjhfuytdyyfytsturstrurtdyjtdy",
        );
        let expect = "klhoghkuultidkyjrhsfturydtjdyytygffyktjshtjukrstrurtdyjtdy".to_owned();
        assert_eq!(solution(words_1, words_2), expect);
    }
}
