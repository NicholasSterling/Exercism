pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = String::new();
    if let Some(first) = list.first() {
        for (&a, &b) in list.iter().zip(list.iter().skip(1)) {
            proverb.push_str(format!("For want of a {} the {} was lost.\n", a, b).as_str());
        }
//        for two in list.windows(2) {  // can't say  for &[a, b] in ...
//            if let &[a, b] = two {
//                proverb.push_str(format!("For want of a {} the {} was lost.\n", a, b).as_str());
//            }
//        }
        proverb.push_str(format!("And all for the want of a {}.", first).as_str());
    }
    proverb
}
