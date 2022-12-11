#[derive(Hash, Eq, PartialEq, Debug)]
struct Monke {
    items: Vec<u64>,
    operation: char,
    second_param: String,
    test_exp_modulo: u64,
    test_true: usize,
    test_false: usize,
    inspected: u64
}

impl Monke {
    fn calculate(&self, item_idx: usize) -> u64 {
        let mut second = 0;
        if self.second_param == "self" {
            second = self.items[item_idx];
        } else {
            second = self.second_param.parse().unwrap();
        }
        
        // /= 
        return match self.operation {
            'm' => (self.items[item_idx] * second) % (2 * 17 * 19 * 3 * 5 * 13 * 7 * 11),
            'a' => (self.items[item_idx] + second) % (2 * 17 * 19 * 3 * 5 * 13 * 7 * 11),
            _ => panic!()
        }
    }

    fn test(&self, item_idx: usize) -> bool {
        return self.items[item_idx] % self.test_exp_modulo == 0;
    }

    fn throw(&mut self, exp_result: bool) -> usize {
        if exp_result == true {
            return self.test_true;
        } else {
            return self.test_false;
        }
    }
}

fn main() {
    let mut monkes = Vec::<Monke>::new();
    
    // fuck it lazy to parse input
    monkes.push(Monke{items:vec![83, 62, 93], operation: 'm', second_param: "17".to_string(), test_exp_modulo: 2, test_true: 1, test_false: 6, inspected: 0});
    monkes.push(Monke{items:vec![90, 55], operation: 'a', second_param: "1".to_string(), test_exp_modulo: 17, test_true: 6, test_false: 3, inspected: 0});
    monkes.push(Monke{items:vec![91, 78, 80, 97, 79, 88], operation: 'a', second_param: "3".to_string(), test_exp_modulo: 19, test_true: 7, test_false: 5, inspected: 0});
    monkes.push(Monke{items:vec![64, 80, 83, 89, 59], operation: 'a', second_param: "5".to_string(), test_exp_modulo: 3, test_true: 7, test_false: 2, inspected: 0});
    monkes.push(Monke{items:vec![98, 92, 99, 51], operation: 'm', second_param: "self".to_string(), test_exp_modulo: 5, test_true: 0, test_false: 1, inspected: 0});
    monkes.push(Monke{items:vec![68, 57, 95, 85, 98, 75, 98, 75], operation: 'a', second_param: "2".to_string(), test_exp_modulo: 13, test_true: 4, test_false: 0, inspected: 0});
    monkes.push(Monke{items:vec![74], operation: 'a', second_param: "4".to_string(), test_exp_modulo: 7, test_true: 3, test_false: 2, inspected: 0});
    monkes.push(Monke{items:vec![68, 64, 60, 68, 87, 80, 82], operation: 'm', second_param: "19".to_string(), test_exp_modulo: 11, test_true: 4, test_false: 5, inspected: 0});
    // monkes.push(Monke{items:vec![79, 98], operation: 'm', second_param: "19".to_string(), test_exp_modulo: 23, test_true: 2, test_false: 3, inspected: 0});
    // monkes.push(Monke{items:vec![54, 65, 75, 74], operation: 'a', second_param: "6".to_string(), test_exp_modulo: 19, test_true: 2, test_false: 0, inspected: 0});
    // monkes.push(Monke{items:vec![79, 60, 97], operation: 'm', second_param: "self".to_string(), test_exp_modulo: 13, test_true: 1, test_false: 3, inspected: 0});
    // monkes.push(Monke{items:vec![74], operation: 'a', second_param: "3".to_string(), test_exp_modulo: 17, test_true: 0, test_false: 1, inspected: 0});

    //let divider = 2 * 17 * 19 * 3 * 5 * 13 * 7 * 11
    for _ in 0..10000 {
        for i in 0..monkes.len() {
            let mut changes = Vec::new();

            for item_idx in 0..monkes[i].items.len() {
                let item = monkes[i].calculate(item_idx);
                monkes[i].items[item_idx] = item;
                monkes[i].inspected += 1;
                let exp_test = monkes[i].test(item_idx);
                let idx: usize = monkes[i].throw(exp_test);
                //println!("{:?}", monkes[i]);

                monkes[idx].items.push(item);
                changes.push(item);
            }

            for item in changes {
                let index = monkes[i].items.iter().position(|x| *x == item).unwrap();
                monkes[i].items.remove(index);
            }
        }
        
    }

    monkes.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    println!("Monkey inspected {:?}", monkes[0].inspected * monkes[1].inspected);

}
