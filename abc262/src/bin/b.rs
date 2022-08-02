use proconio::input;

fn main() {
    input! {
        N: usize,
        M: i32,
       a: [[i32; 2]; M],
    }

    let mut count = 0;
    let mut graph: [usize; N];

    for i in 0..N {
        graph.push(Vec::new());
    }

    for _a in a {
        graph[(_a[0] - 1) as usize].push(_a[1] - 1);
    }

    for connects in &graph {
        for connect in connects {
            let connects_other_node = &graph[*connect as usize];

            for connects_this_node in connects {
                if connects_other_node
                    .iter()
                    .find(|x| **x == *connects_this_node)
                    .is_some()
                {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
