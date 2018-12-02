use std::collections::HashMap;

#[allow(dead_code)]
fn day_2_part_1(input: &str) -> i32 {
    let (triples, doubles) =
        input.lines().map(|s| HasDoubleOrTriple::new(s))
            .fold((0, 0), |acc, hdot| {
                let triples = if hdot.has_triple {
                    acc.0 + 1
                } else  {
                    acc.0
                };

                let doubles = if hdot.has_double {
                    acc.1 + 1
                } else {
                    acc.1
                };

                (triples, doubles)
            });

  triples * doubles
}

struct HasDoubleOrTriple {
  has_double: bool,
  has_triple: bool
}

impl HasDoubleOrTriple {
  fn new(input: &str) -> Self {
    let zero: HashMap<char, i32> = HashMap::new();
    let other_zero = HasDoubleOrTriple {
        has_double: false,
        has_triple: false
    };

    input.chars()
        .fold(zero, |mut acc, c| {
            acc.entry(c)
                .and_modify(|e| { *e += 1 })
                .or_insert(1);
            acc
        })
        .values()
        .fold(other_zero, |acc, val| {
          if *val == 3 {
            HasDoubleOrTriple {
                has_triple: true,
                has_double: acc.has_double
            }
          } else if *val == 2 {
            HasDoubleOrTriple {
                has_triple: acc.has_triple,
                has_double: true
            }
          } else {
            acc
          }
        })
  }
}

/// We brute-force the solution here - there are only 250 items in the list, so
/// seems easier to do so.
#[allow(dead_code)]
fn day_2_part_2(input: &str) -> String {
    let ids: Vec<&str> = input.lines().collect();
    for id in ids.iter() {
        for other_id in ids.iter() {
            let res: String = id.chars().zip(other_id.chars()).flat_map(|(c1, c2)| {
                if c1 == c2 {
                    Some(c1)
                } else {
                    None
                }
            }).collect();
            if id.len() - res.len() == 1 {
                return res;
            }
        }
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use day_2_part_1;
    use day_2_part_2;

    #[test]
    fn day_2_part_1_examples() {
        assert_eq!(day_2_part_1(include_str!("examples_part_1")), 12);
    }

    #[test]
    fn day_2_part_1_test_input() {
        assert_eq!(day_2_part_1(include_str!("input")), 4712);
    }

    #[test]
    fn day_2_part_2_examples() {
        assert_eq!(day_2_part_2(include_str!("examples_part_2")), "fgij");
    }

    #[test]
    fn day_2_part_2_test_input() {
        assert_eq!(day_2_part_2(include_str!("input")), "lufjygedpvfbhftxiwnaorzmq");
    }
}
