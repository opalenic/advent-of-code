use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashSet;


// Each path consists of a possibly shared init part, and a unique last part
#[derive(Debug)]
struct Path(Rc<RefCell<Vec<(usize, usize)>>>, (usize, usize));

impl Path {
    fn new(start_pos: (usize, usize)) -> Path {
        Path(Rc::new(RefCell::new(Vec::new())), start_pos)
    }

    fn expand_path(&self, maze: &Maze, visited: &mut HashSet<(usize, usize)>) -> Vec<Path> {

        // Create new path init part
        let new_path_rc = Rc::new(RefCell::new(self.0.borrow().clone()));
        new_path_rc.borrow_mut().push(self.1);

        // Get possible movements, filter already seen positions
        let opts = maze.movement_options(self.1).into_iter().filter(|opt| visited.insert(*opt));

        // Create new paths
        opts.map(|new_pos| Path(new_path_rc.clone(), new_pos)).collect()
    }

    fn to_vec(&self) -> Vec<(usize, usize)> {
        let mut out = (*self.0.borrow()).clone();
        out.push(self.1);
        out
    }
}

// -----> x
// |
// |
// v
// y
struct Maze {
    fav_num: usize
}

impl Maze {
    fn new(fav_num: usize) -> Maze {
        Maze { fav_num }
    }

    fn is_wall(&self, pos: (usize, usize)) -> bool {
        let val = pos.0 * pos.0 + 3 * pos.0 + 2 * pos.0 * pos.1 + pos.1 + pos.1 * pos.1 + self.fav_num;

        (val.count_ones() % 2) != 0
    }

    fn movement_options(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {

        let mut opts = Vec::new();

        // up
        if pos.1 > 0 && !self.is_wall((pos.0, pos.1 - 1)) {
            opts.push((pos.0, pos.1 - 1));
        }

        // down
        if !self.is_wall((pos.0, pos.1 + 1)) {
            opts.push((pos.0, pos.1 + 1));
        }

        // left
        if pos.0 > 0 && !self.is_wall((pos.0 - 1, pos.1)) {
            opts.push((pos.0 - 1, pos.1));
        }

        // right
        if !self.is_wall((pos.0 + 1, pos.1)) {
            opts.push((pos.0 + 1, pos.1));
        }

        opts
    }
}

fn format_maze_with_path(maze: &Maze, size: (usize, usize), path: &Vec<(usize, usize)>) -> String {
    let mut out = String::with_capacity((size.0 + 1) * size.1);

    for y in 0..size.1 {
        for x in 0..size.0 {

            let ch = if path.contains(&(x, y)) { 'O' } else if maze.is_wall((x, y)) { '#' } else { '.' };

            out.push(ch);
        }
        out.push('\n');
    }

    out
}



fn find_path(maze: &Maze, start_pos: (usize, usize), end_pos: (usize, usize)) -> Vec<(usize, usize)> {

    let mut visited = HashSet::new();
    visited.insert(start_pos);

    let mut open_paths = vec![Path::new(start_pos)];
    let mut new_paths = Vec::new();

    loop {

        if let Some(curr_path) = open_paths.pop() {

            for new_path in curr_path.expand_path(maze, &mut visited) {
                if new_path.1 == end_pos {
                    return new_path.to_vec();
                }

                new_paths.push(new_path);
            }

        } else {

            open_paths = new_paths;
            new_paths = vec![];

            if open_paths.len() == 0 {
                panic!("no new paths");
            }
        }
    }
}


fn find_places(maze: &Maze, start_pos: (usize, usize), max_depth: usize) -> HashSet<(usize, usize)> {

    let mut visited = HashSet::new();
    visited.insert(start_pos);

    let mut open_paths = vec![Path::new(start_pos)];
    let mut new_paths = Vec::new();


    for _ in 0..max_depth {
        loop {

            if let Some(curr_path) = open_paths.pop() {

                for new_path in curr_path.expand_path(maze, &mut visited) {
                    new_paths.push(new_path);
                }

            } else {

                open_paths = new_paths;
                new_paths = vec![];

                if open_paths.len() == 0 {
                    panic!("no new paths");
                }

                break;
            }
        }
    }

    return visited;
}




fn main() {
    let maze = Maze::new(1362);

    let path = find_path(&maze, (1, 1), (31, 39));
    println!("{}", format_maze_with_path(&maze, (33, 41), &path));
    println!("Total path length: {}", path.len() - 1);


    let places_vec = find_places(&maze, (1, 1), 50).into_iter().collect();
    println!("{}", format_maze_with_path(&maze, (33, 41), &places_vec));
    println!("Unique places visited at depth 50: {}", places_vec.len());
}


#[cfg(test)]
mod tests {

    use super::Maze;
    use super::format_maze_with_path;
    use super::find_path;

    const TEST_MAZE: &str = ".#.####.##\n\
                             ..#..#...#\n\
                             #....##...\n\
                             ###.#.###.\n\
                             .##..#..#.\n\
                             ..##....#.\n\
                             #...##.###\n";

    const TEST_MAZE_WITH_PATH: &str = ".#.####.##\n\
                                       .O#..#...#\n\
                                       #OOO.##...\n\
                                       ###O#.###.\n\
                                       .##OO#OO#.\n\
                                       ..##OOO.#.\n\
                                       #...##.###\n";

    #[test]
    fn wall_calc_test() {
        let maze = Maze::new(10);

        assert_eq!(TEST_MAZE, format_maze_with_path(&maze, (10, 7), vec![]));
    }

    #[test]
    fn path_format_test() {

        let maze = Maze::new(10);
        let path = vec![(1, 1), (1, 2), (2, 2), (3, 2), (3, 3), (3, 4), (4, 4), (4, 5), (5, 5), (6, 5), (6, 4), (7, 4)];

        assert_eq!(TEST_MAZE_WITH_PATH, format_maze_with_path(&maze, (10, 7), &path));
    }

    #[test]
    fn find_solution_test() {
        let maze = Maze::new(10);

        let path = find_path(&maze, (1, 1), (7, 4));

        assert_eq!(11, path.len() - 1);
    }
}