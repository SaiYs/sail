use super::{SuffixSort, LS};
use itertools::Itertools;
use rand::{prelude::SliceRandom, thread_rng};

#[derive(Debug, Clone)]
pub enum InducedSort {}

fn induced_sort(n: usize, s: &[u8], ty: &[LS], seed: Vec<usize>) -> Vec<usize> {
    let mut buckets = vec![0usize; std::u8::MAX as usize + 1];
    for &c in s {
        buckets[c as usize] += 1;
    }
    let mut bin1 = vec![];
    let mut counts = 0usize;
    for count in buckets.into_iter() {
        bin1.push((counts, counts + count));
        counts += count;
    }
    let mut bin2 = bin1.clone();

    let mut sa = vec![None; n];
    for lms in seed.into_iter().rev() {
        sa[bin1[s[lms] as usize].1 - 1] = Some(lms);
        bin1[s[lms] as usize].1 -= 1;
    }

    sa[bin1[s[n - 1] as usize].0] = Some(n - 1);
    bin1[s[n - 1] as usize].0 += 1;
    for i in 0..n {
        if let Some(prev) = sa[i] {
            if prev >= 1 && ty[prev - 1].is_l() {
                sa[bin1[s[prev - 1] as usize].0] = Some(prev - 1);
                bin1[s[prev - 1] as usize].0 += 1;
            }
        }
    }

    for i in (0..n).rev() {
        if let Some(prev) = sa[i] {
            if prev >= 1 && ty[prev - 1].is_s() {
                sa[bin2[s[prev - 1] as usize].1 - 1] = Some(prev - 1);
                bin2[s[prev - 1] as usize].1 -= 1;
            }
        }
    }

    sa.into_iter().map(|x| x.unwrap()).collect_vec()
}

fn sa_is(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut ty = vec![LS::L; n];
    ty[n - 1] = LS::L;
    for i in (0..n - 1).rev() {
        ty[i] = match s[i].cmp(&s[i + 1]) {
            std::cmp::Ordering::Less => LS::S,
            std::cmp::Ordering::Equal => ty[i + 1],
            std::cmp::Ordering::Greater => {
                if ty[i + 1].is_s() {
                    ty[i + 1] = LS::Lms;
                }
                LS::L
            }
        }
    }

    let rep = (0..n).filter(|&x| ty[x].is_lms()).collect_vec();
    let m = rep.len();
    let mut repc = rep.clone();
    repc.shuffle(&mut thread_rng());

    let sa = induced_sort(n, s, &ty, repc);

    let mut cur = 0;
    let mut t = vec![None; n];
    t[n - 1] = Some(0);
    for (&i, &j) in sa.iter().filter(|&&x| ty[x].is_lms()).tuple_windows() {
        for d in (0..).take_while(|&x| {
            x == 0 || i + x < n && j + x < n && !ty[i + x].is_lms() && !ty[j + x].is_lms()
        }) {
            if s[i + d] != s[j + d] {
                cur += 1;
                break;
            }
        }
        t[j] = Some(cur);
    }
    let t = t.into_iter().flatten().collect_vec();

    let seed = if m == 0 {
        vec![]
    } else if cur as usize + 1 < m {
        sa_is(&t)
    } else {
        let mut sa = vec![None; m];
        t.into_iter()
            .enumerate()
            .for_each(|(i, j)| sa[j as usize] = Some(i));
        sa.into_iter().map(|x| x.unwrap()).collect_vec()
    }
    .into_iter()
    .map(|x| rep[x])
    .collect_vec();

    induced_sort(n, s, &ty, seed)
}

impl SuffixSort for InducedSort {
    fn sort(s: &[u8]) -> Vec<usize> {
        sa_is(s)
    }
}
