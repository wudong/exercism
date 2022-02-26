use std::cmp::Ordering;
use std::iter::once;

pub fn lowest_price(books: &[u32]) -> u32 {
    let groups = create_groups(books);
    total(&groups)
}

fn create_groups(books: &[u32]) -> Vec<u32> {
    let mut groups = vec![];

    for book in books {
        let group_idx = choose_group_with_cost(&groups, *book);
        if group_idx == groups.len() {
            groups.push(add_to_group(0, *book));
        } else {
            let g = groups.get_mut(group_idx).unwrap();
            *g = add_to_group(*g, *book);
        }
    }
    groups
}

// choose which group to add with cost calculated.
fn choose_group_with_cost(groups: &[u32], book: u32) -> usize {
    let cost = groups
        .iter()
        .chain(once(&0))
        .map(|g| {
            if contain_in_group(*g, book) {
                u32::MAX
            } else {
                let new_g = add_to_group(*g, book);
                cost_of_group(new_g) - cost_of_group(*g)
            }
        })
        .enumerate()
        .map(|x| {
            println!("{:?}", x);
            x
        })
        .min_by(|x, y| {
            let ord = u32::cmp(&x.1, &y.1);
            match ord {
                Ordering::Equal => usize::cmp(&x.0, &y.0).reverse(),
                x => x,
            }
        })
        .unwrap();
    println!("choose {:?}", cost);

    cost.0
}

fn contain_in_group(group: u32, book: u32) -> bool {
    group & (1 << (book - 1)) != 0
}

fn add_to_group(group: u32, book: u32) -> u32 {
    group | (1 << (book - 1))
}

fn cost_of_group(group: u32) -> u32 {
    let mut count = 0;
    for i in 1..=5 {
        if contain_in_group(group, i) {
            count += 1;
        }
    }
    let total = count * 800;

    match count {
        1 => total,
        2 => total / 100 * 95,
        3 => total / 100 * 90,
        4 => total / 100 * 80,
        5 => total / 100 * 75,
        _ => 0,
    }
}

fn total(groups: &[u32]) -> u32 {
    groups.iter().map(|x| cost_of_group(*x)).sum()
}
