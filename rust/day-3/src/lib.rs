#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
struct ClaimResolution {
    overlaps: usize,
    non_overlapping: Option<usize>
}

#[allow(dead_code)]
fn day_3(input: &str) -> ClaimResolution {
    let claims: Vec<Claim> = input.lines()
        .filter(|s| !s.is_empty())
        .map(|s| Claim::new(s))
        .collect();
    let claim_ids: HashSet<usize> = claims.iter().map(|claim| claim.id).collect();

    let mut fabric: HashMap<(usize, usize), usize> = HashMap::new();

    let mut overlapping_claim_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut overlapping_claim_ids: HashSet<usize> = HashSet::new();

    for claim in claims.iter() {
        // Improve cache hit access by doing y outer, x inner.
        for y in claim.y..(claim.y + claim.height) {
            for x in claim.x..(claim.x + claim.width) {
                fabric.entry((x, y))
                    // If the space was already taken, record the two IDs
                    // that clashed and the coords.
                    .and_modify(|existing_claim_id| {
                        overlapping_claim_coords.insert((x, y));
                        overlapping_claim_ids.insert(*existing_claim_id);
                        overlapping_claim_ids.insert(claim.id);
                    })
                    .or_insert(claim.id);
            }
        }
    };

    ClaimResolution {
        overlaps: overlapping_claim_coords.len(),
        non_overlapping: claim_ids.difference(&overlapping_claim_ids).cloned().next()
    }
}

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

impl Claim {
    fn new(claim: &str) -> Claim {
        let caps = CLAIM_PATTERN.captures(claim)
            .expect(&format!("Regex didn't match passed claim '{}'.", claim));

        Claim {
          id: caps["id"].parse().expect(&format!("id '{}' could not be parsed to usize.", &caps["id"])),
          x: caps["x"].parse().expect(&format!("x-coordinate '{}' could not be parsed to usize.", &caps["x"])),
          y: caps["y"].parse().expect(&format!("y-coordinate '{}' could not be parsed to usize.", &caps["y"])),
          width: caps["width"].parse().expect(&format!("width '{}' could not be parsed to usize.", &caps["width"])),
          height: caps["height"].parse().expect(&format!("height '{}' could not be parsed to usize.", &caps["height"]))
        }
    }
}

lazy_static! {
    static ref CLAIM_PATTERN: Regex = Regex::new(r"(?x)
\#(?P<id>\d+) # ID
\s@\s
(?P<x>\d+) # x-coordinate
,
(?P<y>\d+) # y-coordinate
:\s
(?P<width>\d+) # width
x
(?P<height>\d+) # height
").unwrap();
}

#[cfg(test)]
mod tests {
    use day_3;

    #[test]
    fn day_3_part_1_examples() {
        assert_eq!(day_3(include_str!("examples")).overlaps, 4);
    }

    #[test]
    fn day_3_part_1_test_input() {
        assert_eq!(day_3(include_str!("input")).overlaps, 119551);
    }

    #[test]
    fn day_3_part_2_examples() {
        assert_eq!(day_3(include_str!("examples")).non_overlapping, Some(3));
    }

    #[test]
    fn day_3_part_2_test_input() {
        assert_eq!(day_3(include_str!("input")).non_overlapping, Some(1124));
    }
}
