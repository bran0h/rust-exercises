trait MyAdd {
    fn my_add(&self, other: &Self) -> Self;
}
impl MyAdd for i32 {
    fn my_add(&self, other: &i32) -> i32 {
        self + other
    }
}
impl MyAdd for f32 {
    fn my_add(&self, other: &f32) -> f32 {
        self + other
    }
}

fn add_values<T: MyAdd>(left: &T, right: &T) -> T {
    left.my_add(right)
}

struct IBAN(String);

fn check_iban(iban: &IBAN) -> bool {
    todo!("Check IBAN: {}", iban.0)
}

fn main() {
    let sum_one = add_values(&6, &8);
    assert_eq!(sum_one, 14);
    let sum_two = add_values(&6.5, &7.5);
    println!("Sum two: {}", sum_two); // 14
}
