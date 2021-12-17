fn simulate(
    initial_speed: (isize, isize),
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
) -> bool {
    let (mut speed_x, mut speed_y) = initial_speed;
    let mut x = 0;
    let mut y = 0;
    loop {
        x += speed_x;
        y += speed_y;
        if (x_min..=x_max).contains(&x) && (y_min..=y_max).contains(&y) {
            return true;
        } else if x > x_max || y < y_min {
            return false;
        }
        speed_y -= 1;
        speed_x = match speed_x {
            0 => 0,
            v if v > 0 => v - 1,
            v => v + 1,
        };
    }
}

fn possible_speeds(x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> Vec<(isize, isize)> {
    let mut res = Vec::new();
    for vx in 1..=x_max {
        for vy in y_min..y_min.abs() {
            if simulate((vx, vy), x_min, x_max, y_min, y_max) {
                res.push((vx, vy))
            }
        }
    }
    res
}

#[aoc(day17, part1)]
pub fn part1(_input: &str) -> isize {
    let x_min: isize = 206;
    let x_max: isize = 250;
    let y_min: isize = -105;
    let y_max: isize = -57;
    let speeds = possible_speeds(x_min, x_max, y_min, y_max);
    let vy_max = speeds.iter().map(|(_, y)| y).max().unwrap();
    // we have y(t) = v_0*t - t(t-1)/2
    // maximized at t=v_O with the value below
    vy_max * (vy_max + 1) / 2
}

#[aoc(day17, part2)]
pub fn part2(_input: &str) -> usize {
    let x_min: isize = 206;
    let x_max: isize = 250;
    let y_min: isize = -105;
    let y_max: isize = -57;
    possible_speeds(x_min, x_max, y_min, y_max).len()
}
