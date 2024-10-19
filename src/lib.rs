// lib.rs

pub fn initialize_distances(token1_len: usize, token2_len: usize) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![0; token2_len + 1]; token1_len + 1];

    // Initialize first column
    for t1 in 0..=token1_len {
        distances[t1][0] = t1;
    }

    // Initialize first row
    for t2 in 0..=token2_len {
        distances[0][t2] = t2;
    }

    distances
}

pub fn calculate_minimum(a: usize, b: usize, c: usize) -> usize {
    usize::min(usize::min(a, b), c)
}

pub fn fill_distances(token1: &str, token2: &str, distances: &mut Vec<Vec<usize>>) {
    let token1_len = token1.len();
    let token2_len = token2.len();

    for t1 in 1..=token1_len {
        for t2 in 1..=token2_len {
            let c1 = token1.chars().nth(t1 - 1).unwrap();
            let c2 = token2.chars().nth(t2 - 1).unwrap();

            if c1 == c2 {
                distances[t1][t2] = distances[t1 - 1][t2 - 1];
            } else {
                let a = distances[t1][t2 - 1];
                let b = distances[t1 - 1][t2];
                let c = distances[t1 - 1][t2 - 1];

                distances[t1][t2] = calculate_minimum(a, b, c) + 1;
            }
        }
    }
}

pub fn levenshtein_distance_dp(token1: &str, token2: &str) -> usize {
    let token1_len = token1.len();
    let token2_len = token2.len();

    // Initialize distances
    let mut distances = initialize_distances(token1_len, token2_len);

    // Fill the distances matrix
    fill_distances(token1, token2, &mut distances);

    distances[token1_len][token2_len]
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_distances() {
        let distances = initialize_distances(2, 2);
        assert_eq!(distances[0][0], 0);
        assert_eq!(distances[1][0], 1);
        assert_eq!(distances[2][0], 2);
        assert_eq!(distances[0][1], 1);
        assert_eq!(distances[0][2], 2);
    }

    #[test]
    fn test_calculate_minimum() {
        assert_eq!(calculate_minimum(1, 2, 3), 1);
        assert_eq!(calculate_minimum(5, 5, 5), 5);
        assert_eq!(calculate_minimum(3, 1, 2), 1);
    }

    #[test]
    fn test_fill_distances() {
        let token1 = "abc";
        let token2 = "adc";
        let mut distances = initialize_distances(token1.len(), token2.len());

        fill_distances(token1, token2, &mut distances);

        assert_eq!(distances[1][1], 0);
        assert_eq!(distances[2][2], 1);
        assert_eq!(distances[3][3], 1);
    }

    #[test]
    fn test_levenshtein_distance_dp() {
        assert_eq!(levenshtein_distance_dp("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance_dp("flaw", "lawn"), 2);
        assert_eq!(levenshtein_distance_dp("intention", "execution"), 5);
    }
}
