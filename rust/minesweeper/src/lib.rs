use std::str;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let map: Vec<&[u8]> = minefield.iter().map(|&x| x.as_bytes()).collect();
    return if map.len() == 0 {
        vec![]
    } else {
        let length = map.len();
        let width = map[0].len();

        let vvv = (0..length)
            .map(|i| {
                let vv = (0..width)
                    .map(|j| convert(&map, (i, j), (length, width)))
                    .collect::<Vec<u8>>();
                String::from_utf8(vv).unwrap()
            })
            .collect();
        vvv
    };
}

fn convert(map: &Vec<&[u8]>, pp: (usize, usize), bound: (usize, usize)) -> u8 {
    let check_points = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (-1, 1),
        (1, 1),
        (-1, -1),
        (1, -1),
    ];
    if is_mine(map, &pp) {
        return b'*';
    }
    let count = check_points
        .iter()
        .filter_map(|diff| add_points(&pp, bound, diff))
        .filter(|p| is_mine(map, p))
        .count();
    match count {
        0 => b' ',
        x => x as u8 + b'0',
    }
}

fn is_mine(map: &Vec<&[u8]>, p: &(usize, usize)) -> bool {
    map[p.0][p.1] == b'*'
}

fn add_points(
    pp: &(usize, usize),
    bound: (usize, usize),
    diff: &(i32, i32),
) -> Option<(usize, usize)> {
    let x = add(pp.0, diff.0);
    let y = add(pp.1, diff.1);
    match (x, y) {
        (x, y) if x >= 0 && x < bound.0 as i32 && y >= 0 && y < bound.1 as i32 => {
            Some((x as usize, y as usize))
        }
        _ => None,
    }
}

fn add(x: usize, y: i32) -> i32 {
    return x as i32 + y;
}
