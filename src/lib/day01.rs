pub struct Day1<'a> {
    input: &'a str,
    test: &'a str,
}

impl<'a> Day1<'a> {
    pub fn new() -> Self {
        Self {
            input: include_str!("C:/Users/Tea/dev/r_aoc/src/input/input01.g"),
            test: include_str!("C:/Users/Tea/dev/r_aoc/src/input/test01.g"),
        }
    }
    pub fn solve_test(&self) -> isize {
        let mut xs = vec![];
        let mut ys = vec![];
        self.test
            .lines()
            .map(|x| {
                x.split("   ")
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .flatten()
            .enumerate()
            .for_each(|(i, x)| match i % 2 == 0 {
                true => xs.push(x),
                false => ys.push(x),
            });
        xs.sort();
        ys.sort();
        xs.into_iter()
            .zip(ys.into_iter())
            .map(|(x, y)| (x - y).abs())
            .sum()
    }

    pub fn solve_input(&self) -> isize {
        let mut xs = vec![];
        let mut ys = vec![];
        self.input
            .lines()
            .map(|x| {
                x.split("   ")
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .flatten()
            .enumerate()
            .for_each(|(i, x)| match i % 2 == 0 {
                true => xs.push(x),
                false => ys.push(x),
            });
        xs.sort();
        ys.sort();
        xs.into_iter()
            .zip(ys.into_iter())
            .map(|(x, y)| (x - y).abs())
            .sum()
    }

    pub fn solve_input_part_two(&self) -> isize {
        let mut xs = vec![];
        let mut ys = vec![];
        self.test
            .lines()
            .map(|x| {
                x.split("   ")
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .flatten()
            .enumerate()
            .for_each(|(i, x)| match i % 2 == 0 {
                true => xs.push(x),
                false => ys.push(x),
            });
        xs.sort();
        ys.sort();
        xs.into_iter()
            .map(|x| x * ys.clone().into_iter().filter(|&y| x == y).count() as isize)
            .sum()
    }

    pub fn solve_test_part_two(&self) -> isize {
        let mut xs = vec![];
        let mut ys = vec![];
        self.input
            .lines()
            .map(|x| {
                x.split("   ")
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .flatten()
            .enumerate()
            .for_each(|(i, x)| match i % 2 == 0 {
                true => xs.push(x),
                false => ys.push(x),
            });
        xs.sort();
        ys.sort();
        xs.into_iter()
            .map(|x| x * ys.clone().into_iter().filter(|&y| x == y).count() as isize)
            .sum()
    }
}
