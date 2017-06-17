#[derive(Debug)]
struct BIT {
    tree: Vec<usize>,
    length: usize
}

impl BIT {
    fn new(n: usize) -> BIT {
        BIT {
            tree: vec![0; n+1],
            length: n
        }
    }

    fn add(&mut self, n: usize, mut k: usize) {
        while k <= self.length {
            self.tree[k] += n;
            k += k & (!k + 1);
        };
    }

    fn sum(&self, mut k: usize) -> usize {
        let mut sum: usize = 0;
        while k > 0 {
            sum += self.tree[k];
            k -= k & (!k + 1);
        };
        sum
    }
}

pub fn count_inc_seq(seq: Vec<usize>) -> usize {
    let len = seq.len();
    let MOD = 1_000_000_007;

    struct Item {
        number: usize,
        index: usize
    };

    let mut s: Vec<Item> = Vec::new();

    for i in 0..len {
        let item = Item {number: seq[i], index: i};
        s.push(item);
    };
    s.sort_by(|a, b| b.number.cmp(&a.number));

    let mut tree = BIT::new(len);

    for item in s.iter() {
        let c = (1 + tree.sum(len) - tree.sum(item.index + 1)) % MOD;
        tree.add(c, item.index + 1);
    };
    tree.sum(len) % MOD
}


#[test]
fn test_BIT() {
    let mut tree = BIT::new(5);
    assert_eq!(vec![0; 6], tree.tree);
    tree.add(1, 1);
    assert_eq!(vec![0, 1, 1, 0, 1, 0], tree.tree);
    tree.add(2, 2);
    assert_eq!(vec![0, 1, 3, 0, 3, 0], tree.tree);
    tree.add(3, 3);
    assert_eq!(vec![0, 1, 3, 3, 6, 0], tree.tree);
    tree.add(4, 4);
    assert_eq!(vec![0, 1, 3, 3, 10, 0], tree.tree);
    tree.add(5, 5);
    assert_eq!(vec![0, 1, 3, 3, 10, 5], tree.tree);
    assert_eq!(1, tree.sum(1));
    assert_eq!(3, tree.sum(2));
    assert_eq!(6, tree.sum(3));
    assert_eq!(10, tree.sum(4));
    assert_eq!(15, tree.sum(5));
    let s = vec![1, 2, 0, 0, 0, 4];
    assert_eq!(13, count_inc_seq(s));
}
