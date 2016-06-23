
use std::env;

fn find_code(code_pos: (usize, usize)) -> u64 {
    if code_pos.0 == 0 || code_pos.1 == 0 {
        panic!("Invalid position in matrix: {:?}", code_pos);
    }
    // code_pos (row, column)
    let mut pos: (usize, usize) = (1, 1);

    let mut code = 20151125;

    while pos != code_pos {

        if (pos.0 - 1) == 0 {
            pos.0 = pos.1 + 1;
            pos.1 = 1;
        } else {
            pos.0 -= 1;
            pos.1 += 1;
        }

        code = (code * 252533) % 33554393;
    }

    code
}


fn main() {
    let mut a = env::args();

    a.next(); // The first argument is the binary name/path

    let row = a.next().unwrap().parse::<usize>().unwrap();
    let col = a.next().unwrap().parse::<usize>().unwrap();


    println!("The code at row {}, column {}: {}",
             row,
             col,
             find_code((row, col)));
}


#[cfg(test)]
mod tests {

    use super::find_code;

    #[test]
    fn find_code_test() {
        //    |    1         2         3         4         5         6
        // ---+---------+---------+---------+---------+---------+---------+
        //  1 | 20151125  18749137  17289845  30943339  10071777  33511524
        //  2 | 31916031  21629792  16929656   7726640  15514188   4041754
        //  3 | 16080970   8057251   1601130   7981243  11661866  16474243
        //  4 | 24592653  32451966  21345942   9380097  10600672  31527494
        //  5 |    77061  17552253  28094349   6899651   9250759  31663883
        //  6 | 33071741   6796745  25397450  24659492   1534922  27995004

        let expected = vec![
            vec![20151125, 18749137, 17289845, 30943339, 10071777, 33511524],
            vec![31916031, 21629792, 16929656,  7726640, 15514188,  4041754],
            vec![16080970,  8057251,  1601130,  7981243, 11661866, 16474243],
            vec![24592653, 32451966, 21345942,  9380097, 10600672, 31527494],
            vec![   77061, 17552253, 28094349,  6899651,  9250759, 31663883],
            vec![33071741,  6796745, 25397450, 24659492,  1534922, 27995004],
        ];

        for row in 0..expected.len() {
            for col in 0..expected[row].len() {
                assert_eq!(find_code((row + 1, col + 1)), expected[row][col]);
            }
        }
    }
}
