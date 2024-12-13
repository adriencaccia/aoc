use arrayvec::ArrayVec;

const SIZE: usize = 140;

pub fn part1(input: &str) -> u32 {
    let mut grid = [[b'0'; SIZE]; SIZE];
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
        });
    });
    let mut visited = [[false; SIZE]; SIZE];
    let mut total = 0;
    let mut stack = ArrayVec::<(usize, usize), { 3 * SIZE }>::new();

    for i in 0..SIZE {
        for j in 0..SIZE {
            if visited[i][j] || grid[i][j] == b'0' {
                continue;
            }

            let c_char = grid[i][j];
            let mut perimeter = 0;
            let mut area = 0;
            stack.clear();
            stack.push((i, j));

            while let Some((i, j)) = stack.pop() {
                if visited[i][j] || grid[i][j] != c_char {
                    continue;
                }
                area += 1;
                visited[i][j] = true;
                // on the edge
                if i == SIZE - 1 || grid[i + 1][j] == b'0' {
                    perimeter += 1;
                }
                if j == SIZE - 1 || grid[i][j + 1] == b'0' {
                    perimeter += 1;
                }
                if i == 0 {
                    perimeter += 1;
                }
                if j == 0 {
                    perimeter += 1;
                }
                // test neighbors for perimeter
                if i > 0 {
                    if grid[i - 1][j] != c_char {
                        perimeter += 1;
                    } else {
                        stack.push((i - 1, j));
                    }
                }
                if j > 0 {
                    if grid[i][j - 1] != c_char {
                        perimeter += 1;
                    } else {
                        stack.push((i, j - 1));
                    }
                }

                if i < SIZE - 1 && grid[i + 1][j] != b'0' {
                    if grid[i + 1][j] != c_char {
                        perimeter += 1;
                    } else {
                        stack.push((i + 1, j));
                    }
                }
                if j < SIZE - 1 && grid[i][j + 1] != b'0' {
                    if grid[i][j + 1] != c_char {
                        perimeter += 1;
                    } else {
                        stack.push((i, j + 1));
                    }
                }
            }

            total += area * perimeter;
        }
    }

    total
}

pub fn part2(input: &str) -> u32 {
    let mut grid = [[b'0'; SIZE]; SIZE];
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
        });
    });
    let mut visited = [[false; SIZE]; SIZE];
    let mut total = 0;
    let mut stack = ArrayVec::<(usize, usize), { 3 * SIZE }>::new();

    for i in 0..SIZE {
        for j in 0..SIZE {
            if visited[i][j] || grid[i][j] == b'0' {
                continue;
            }

            let c_char = grid[i][j];
            let mut corners = 0;
            let mut area = 0;
            stack.clear();
            stack.push((i, j));

            while let Some((i, j)) = stack.pop() {
                if visited[i][j] || grid[i][j] != c_char {
                    continue;
                }
                area += 1;
                visited[i][j] = true;

                // test if top-left corner
                if i > 0
                    && j > 0
                    && ((grid[i - 1][j - 1] != c_char
                        && grid[i - 1][j] == c_char
                        && grid[i][j - 1] == c_char)
                        || (grid[i - 1][j] != c_char && grid[i][j - 1] != c_char))
                {
                    corners += 1;
                }
                // test if top-left corner when i==0
                if i == 0 && j > 0 && grid[i][j - 1] != c_char {
                    corners += 1;
                }
                // test if top-left corner when j==0
                if j == 0 && i > 0 && grid[i - 1][j] != c_char {
                    corners += 1;
                }
                // test if top-left corner when i==0 and j==0
                if i == 0 && j == 0 {
                    corners += 1;
                }

                // test if top-right corner
                if i > 0
                    && j < SIZE - 1
                    && ((grid[i - 1][j + 1] != c_char
                        && grid[i - 1][j + 1] != b'0'
                        && grid[i - 1][j] == c_char
                        && (grid[i][j + 1] == c_char || grid[i][j + 1] == b'0'))
                        || (grid[i - 1][j] != c_char && grid[i][j + 1] != c_char))
                {
                    corners += 1;
                }
                // test if top-right corner when i==0
                if i == 0 && j < SIZE - 1 && (grid[i][j + 1] != c_char || grid[i][j + 1] == b'0') {
                    corners += 1;
                }
                // test if top-right corner when j==SIZE-1
                if j == SIZE - 1 && i > 0 && grid[i - 1][j] != c_char {
                    corners += 1;
                }
                // test if top-right corner when i==0 and j==SIZE-1
                if i == 0 && j == SIZE - 1 {
                    corners += 1;
                }

                // test if bottom-right corner
                if i < SIZE - 1
                    && j < SIZE - 1
                    && ((grid[i + 1][j + 1] != c_char
                        && grid[i + 1][j + 1] != b'0'
                        && (grid[i + 1][j] == c_char || grid[i + 1][j] == b'0')
                        && (grid[i][j + 1] == c_char || grid[i][j + 1] == b'0'))
                        || (grid[i + 1][j] != c_char && grid[i][j + 1] != c_char))
                {
                    corners += 1;
                }
                // test if bottom-right corner when i==SIZE-1
                if i == SIZE - 1
                    && j < SIZE - 1
                    && (grid[i][j + 1] != c_char || grid[i][j + 1] == b'0')
                {
                    corners += 1;
                }
                // test if bottom-right corner when j==SIZE-1
                if j == SIZE - 1
                    && i < SIZE - 1
                    && (grid[i + 1][j] != c_char || grid[i + 1][j] == b'0')
                {
                    corners += 1;
                }
                // test if bottom-right corner when i==SIZE-1 and j==SIZE-1
                if i == SIZE - 1 && j == SIZE - 1 {
                    corners += 1;
                }

                // test if bottom-left corner
                if i < SIZE - 1
                    && j > 0
                    && ((grid[i + 1][j - 1] != c_char
                        && grid[i + 1][j - 1] != b'0'
                        && (grid[i + 1][j] == c_char || grid[i + 1][j] == b'0')
                        && grid[i][j - 1] == c_char)
                        || (grid[i + 1][j] != c_char && grid[i][j - 1] != c_char))
                {
                    corners += 1;
                }
                // test if bottom-left corner when i==SIZE-1
                if i == SIZE - 1 && j > 0 && grid[i][j - 1] != c_char {
                    corners += 1;
                }
                // test if bottom-left corner when j==0
                if j == 0 && i < SIZE - 1 && (grid[i + 1][j] != c_char || grid[i + 1][j] == b'0') {
                    corners += 1;
                }
                // test if bottom-left corner when i==SIZE-1 and j==0
                if i == SIZE - 1 && j == 0 {
                    corners += 1;
                }

                // push neighbors to the stack
                if i > 0 && grid[i - 1][j] == c_char {
                    stack.push((i - 1, j));
                }
                if j > 0 && grid[i][j - 1] == c_char {
                    stack.push((i, j - 1));
                }
                if i < SIZE - 1 && grid[i + 1][j] != b'0' && grid[i + 1][j] == c_char {
                    stack.push((i + 1, j));
                }
                if j < SIZE - 1 && grid[i][j + 1] != b'0' && grid[i][j + 1] == c_char {
                    stack.push((i, j + 1));
                }
            }

            total += area * corners;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 1930);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 1433460);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 1206);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 855082);
    }
}
