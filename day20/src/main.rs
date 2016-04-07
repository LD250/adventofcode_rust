const PRESENTS_COUNT: usize = 33100000;

fn main() {
    fn get_house_index(pr_per_house: usize) -> usize {
        let need_result: usize = PRESENTS_COUNT / pr_per_house;
        let mut results: Vec<u32> = vec![pr_per_house as u32; PRESENTS_COUNT / 10];
        for i in 2..PRESENTS_COUNT / 10 {
            let mut p = i - 1;
            while p < need_result {
                results[p] += i as u32;
                p += i;
            }
        };
        let need_result: u32 = need_result as u32;
        results.iter().enumerate().find(|&(_, delivered)| delivered >= &need_result).unwrap().0
    }
    let i: usize = get_house_index(10);
    println!("House number {}", i + 1);

    fn get_house_index2(pr_per_house: u64, house_limits: usize) -> usize {
        let mut results: Vec<u64> = vec![pr_per_house; PRESENTS_COUNT / 10];
        for i in 2..PRESENTS_COUNT / 10 {
            let mut p = i - 1;
            let mut v = 0;
            while p < PRESENTS_COUNT / 10 && v <= house_limits {
                results[p] += i as u64 * pr_per_house;
                p += i;
                v += 1;
            }
        };
        let need_result: u64 = PRESENTS_COUNT as u64;
        results.iter().enumerate().find(|&(_, delivered)| delivered >= &need_result).unwrap().0
    }
    let i: usize = get_house_index2(11, 50);
    println!("House number {}", i + 1);
}
