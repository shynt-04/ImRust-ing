use std::io;

fn input() -> Vec<i64> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("");
    let vec = buf
                .split_whitespace()
                .map(|x| x.parse::<i64>().expect(""))
                .collect::<Vec<i64>>();
    vec
}

fn calc(l: usize, r: usize, cnt: &Vec<Vec<i64>>) -> i64 {
    let mut res: i64 = 0;
    for i in 0..31 {
        let mut val = 0;
        if l > 0 {
            val = cnt[i][l - 1];
        }
        if cnt[i][r] - val == (r - l + 1) as i64 {
            res += 1 << i;
        }
    }
    res
}

fn solve() {
    let n: usize = input()[0] as usize;
    let arr: Vec<i64> = input();
    let mut cnt:Vec<Vec<i64>> = vec![vec![0; n]; 31];
    for i in 0..31 {
        for j in 0usize..n {
            cnt[i][j] = arr[j] >> i & 1;
            if j > 0 {
                cnt[i][j] += cnt[i][j - 1];
            }
        }
    }
    let mut q = input()[0] as u32;
    while q > 0 {
        let lines = input();
        let l: i64 = lines[0] - 1;
        let k: i64 = lines[1];
        let mut le: i64 = l - 1;
        let mut ri: i64 = n as i64;
        while ri - le > 1 {
            let mid = (le + ri) >> 1;
            // println!("mid: {}", mid);
            if calc(l as usize, mid as usize, &cnt) >= k {
                le = mid
            } else {
                ri = mid
            }
            // println!("l: {}", le);
            // println!("r: {}", ri);
        }
        q -= 1;
        if le < l {
            print!("-1 ");
        } else {
            print!("{} ", le + 1);
        }
    }
}
fn main() {
    let ntest: i64 = input()[0];
    for _ in 0..ntest {
        solve();
        print!("\n");
    }
}