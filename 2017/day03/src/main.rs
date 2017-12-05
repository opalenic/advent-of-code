use std::collections::HashMap;

fn get_distance(pos_val: usize) -> usize {

    let mut curr_pos: (isize, isize) = (0, 0);
    let mut min = (0, 0);
    let mut max = (0, 0);

    for _ in 1..pos_val {
        curr_pos = if curr_pos == max {
            // end of spiral, start a new one
            min = (min.0 - 1, min.1 - 1);
            max = (max.0 + 1, max.1 + 1);

            (curr_pos.0 + 1, curr_pos.1)

        } else if curr_pos.0 == max.0 {
            if curr_pos.1 != min.1 {
                // right edge
                (curr_pos.0, curr_pos.1 - 1)
            } else {
                // top right corner
                (curr_pos.0 - 1, curr_pos.1)
            }
        } else if curr_pos.1 == min.1 {
            if curr_pos.0 != min.0 {
                // top edge
                (curr_pos.0 - 1, curr_pos.1)
            } else {
                // top left corner
                (curr_pos.0, curr_pos.1 + 1)
            }
        } else if curr_pos.0 == min.0 {
            if curr_pos.1 != max.1 {
                // left edge
                (curr_pos.0, curr_pos.1 + 1)
            } else {
                // bottom left corner
                (curr_pos.0 + 1, curr_pos.1)
            }
        } else if curr_pos.1 == max.1 {
            // bottom edge
            (curr_pos.0 + 1, curr_pos.1)
        } else {
            panic!("invalid position");
        };
    }

    (curr_pos.0.abs() + curr_pos.1.abs()) as usize
}


fn get_first_larger(pos_val: usize) -> usize {
    let mut curr_pos: (isize, isize) = (0, 0);
    let mut min = (0, 0);
    let mut max = (0, 0);

    let mut vals = HashMap::new();
    vals.insert((0, 0), 1);

    for _ in 1.. {
        curr_pos = if curr_pos == max {
            // end of spiral, start a new one
            min = (min.0 - 1, min.1 - 1);
            max = (max.0 + 1, max.1 + 1);

            (curr_pos.0 + 1, curr_pos.1)

        } else if curr_pos.0 == max.0 {
            if curr_pos.1 != min.1 {
                // right edge
                (curr_pos.0, curr_pos.1 - 1)
            } else {
                // top right corner
                (curr_pos.0 - 1, curr_pos.1)
            }
        } else if curr_pos.1 == min.1 {
            if curr_pos.0 != min.0 {
                // top edge
                (curr_pos.0 - 1, curr_pos.1)
            } else {
                // top left corner
                (curr_pos.0, curr_pos.1 + 1)
            }
        } else if curr_pos.0 == min.0 {
            if curr_pos.1 != max.1 {
                // left edge
                (curr_pos.0, curr_pos.1 + 1)
            } else {
                // bottom left corner
                (curr_pos.0 + 1, curr_pos.1)
            }
        } else if curr_pos.1 == max.1 {
            // bottom edge
            (curr_pos.0 + 1, curr_pos.1)
        } else {
            panic!("invalid position");
        };


        let v = vals.get(&(curr_pos.0 - 1, curr_pos.1 - 1)).unwrap_or(&0) +
            vals.get(&(curr_pos.0, curr_pos.1 - 1)).unwrap_or(&0) +
            vals.get(&(curr_pos.0 + 1, curr_pos.1 - 1)).unwrap_or(&0) +
            vals.get(&(curr_pos.0 - 1, curr_pos.1)).unwrap_or(&0) +
            vals.get(&(curr_pos.0 + 1, curr_pos.1)).unwrap_or(&0) +
            vals.get(&(curr_pos.0 - 1, curr_pos.1 + 1)).unwrap_or(&0) +
            vals.get(&(curr_pos.0, curr_pos.1 + 1)).unwrap_or(&0) +
            vals.get(&(curr_pos.0 + 1, curr_pos.1 + 1)).unwrap_or(&0);

        if v > pos_val {
            return v;
        }

        vals.insert(curr_pos, v);
    }

    unreachable!();
}

fn main() {
    let input = 277678;

    println!("The distance to {} is {}", input, get_distance(input));
    println!(
        "The first number larger than {} is {}",
        input,
        get_first_larger(input)
    );
}


#[cfg(test)]
mod tests {
    use super::get_distance;
    use super::get_first_larger;

    #[test]
    fn test_get_distance() {
        assert_eq!(4, get_distance(21));
    }

    #[test]
    fn test_get_first_larger() {
        assert_eq!(806, get_first_larger(800));
    }
}
