mod mask;

/// Selects a subset from a vector using the given `selection` bitset as a
/// selection mask -- if the `nth` bit is high, then the `nth` element will be
/// chosen
pub fn select_subset<T>(selection: u64, vector: &[T]) -> Vec<T>
where
    T: Copy,
{
    let mut out = vec![];
    for i in 0..64 {
        if selection & (1 << i) > 0 {
            out.push(vector[i])
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_subset_test() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let sub = select_subset(0b0000111100001111, &v);
        assert_eq!(sub, vec![0, 1, 2, 3, 8, 9, 10, 11]);
    }
}
