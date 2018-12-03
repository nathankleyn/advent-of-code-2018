use std::collections::HashSet;

#[allow(dead_code)]
fn day_1_part_1(input: &str) -> i32 {
  day_1_parse(input).iter().fold(0, |acc, x| x + acc)
}

#[allow(dead_code)]
fn day_1_part_2(input: &str) -> i32 {
  let mut seen: HashSet<i32> = HashSet::new();
  let mut acc = 0;

  for x in day_1_parse(input).iter().cycle() {
      if seen.contains(&acc) {
        return acc;
      }
      seen.insert(acc);
      acc = acc + x;
  };

  unreachable!();
}

fn day_1_parse(input: &str) -> Vec<i32> {
  input.lines().filter(|x| !x.is_empty()).map(|x| {
    x.parse::<i32>().expect(&format!("Could not parse '{}' as i32.", x))
  }).collect()
}

#[cfg(test)]
mod tests {
    use day_1_part_1;
    use day_1_part_2;

    #[test]
    fn day_1_part_1_examples() {
        assert_eq!(day_1_part_1("+1\n-2\n+3\n+1"), 3);
        assert_eq!(day_1_part_1("+1\n+1\n+1"), 3);
        assert_eq!(day_1_part_1("+1\n+1\n-2"), 0);
        assert_eq!(day_1_part_1("-1\n-2\n-3"), -6);
    }

    #[test]
    fn day_1_part_1_test_input() {
        assert_eq!(day_1_part_1(include_str!("input")), 408);
    }

    #[test]
    fn day_1_part_2_examples() {
        assert_eq!(day_1_part_2("+1\n-2\n+3\n+1"), 2);
        assert_eq!(day_1_part_2("+1\n-1"), 0);
        assert_eq!(day_1_part_2("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(day_1_part_2("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(day_1_part_2("+7\n+7\n-2\n-7\n-4"), 14);
    }

    #[test]
    fn day_1_part_2_test_input() {
        assert_eq!(day_1_part_2(include_str!("input")), 55250);
    }
}
