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
        let y = self
            .test
            .split("\r\n")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| {
                x.split("   ")
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        let mut xs = vec![vec![], vec![]];
        for i in &y {
            xs[0].push(i[0]);
        }
        for i in &y {
            xs[1].push(i[1]);
        }
        xs[0].sort();
        xs[1].sort();
        dbg!(&xs);
        xs[0]
            .clone()
            .into_iter()
            .zip(xs[1].clone())
            .map(|x| (x.0 - x.1).abs())
            .sum()
    }
    pub fn solve_input(&self) -> isize {
        let y = self
            .input
            .split("\r\n")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| {
                x.split("   ")
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        let mut xs = vec![vec![], vec![]];
        for i in &y {
            xs[0].push(i[0]);
        }
        for i in &y {
            xs[1].push(i[1]);
        }
        xs[0].sort();
        xs[1].sort();
        dbg!(&xs);
        xs[0]
            .clone()
            .into_iter()
            .zip(xs[1].clone())
            .map(|x| (x.0 - x.1).abs())
            .sum()
    }

    pub fn solve_input_part_two(&self) -> isize {
        let y = self
            .input
            .split("\r\n")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| {
                x.split("   ")
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        let mut xs = vec![vec![], vec![]];
        for i in &y {
            xs[0].push(i[0]);
        }
        for i in &y {
            xs[1].push(i[1]);
        }
        xs[0].sort();
        xs[1].sort();
        // dbg!(&xs);
        xs[0]
            .clone()
            .into_iter()
            .map(|x| {
                xs[1]
                    .clone()
                    .into_iter()
                    .filter(move |y| {
                        // dbg!(&y);
                        x == *y
                    })
                    .count() as isize
                    * x
            })
            .sum()
    }

    pub fn solve_test_part_two(&self) -> isize {
        let y = self
            .test
            .split("\r\n")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| {
                x.split("   ")
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        let mut xs = vec![vec![], vec![]];
        for i in &y {
            xs[0].push(i[0]);
        }
        for i in &y {
            xs[1].push(i[1]);
        }
        xs[0].sort();
        xs[1].sort();
        // dbg!(&xs);
        xs[0]
            .clone()
            .into_iter()
            .map(|x| {
                xs[1]
                    .clone()
                    .into_iter()
                    .filter(move |y| {
                        // dbg!(&y);
                        x == *y
                    })
                    .count() as isize
                    * x
            })
            .sum()
    }

    // pub fn solve_test(&self) -> i32 {
    //     let mut x = self.parse_test();
    //     let len = x.len() / 2;
    //     dbg!(&x);

    //     let temp = x.split_at_mut(len);
    //     let z = temp.1;
    //     z.sort();
    //     let x = temp.0;
    //     x.sort();

    //     dbg!(&x);
    //     dbg!(&z);
    //     z.into_iter().zip(x).map(|x| (*x.0 - *x.1).abs()).sum()
    // }
}
