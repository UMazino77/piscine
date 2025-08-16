#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Cart {
    pub items : Vec<(String, f32)>,
    pub receipt : Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart{items : vec![], receipt : vec![]}
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let mut  price = 0.0 ;
        for (i,v) in &s.products {
            if *i == ele {
                price = *v ;
            }
        }
        self.items.push((ele, price))
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let format = |x: f32| -> f32 {
            (x * 100.0).round() / 100.0
        };
        
        let mut a = vec![];
        for (_, v) in &self.items {
            a.push(*v);
        }
        a.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let mut i = 0.;
        // let mut jj = 0;
        let mut b = vec![];
        // let mut bb = 0.;

        // let mut j: f32 = 0.;
        
        // for aa in &a {
        //     if i%3. == 0. {
        //         bb += *aa;
        //     }
        //     i+=1.;
        // }

        i = a[0..a.len()/3].iter().sum::<f32>()/(a.iter().sum::<f32>());
        println!("{i}"); 
        // i = bb/(a.iter().sum::<f32>()) ;
        println!("{:?}", a);
        println!("{i}"); 


        for aa in &a {
            b.push(((((*aa-(*aa)*i)*100.)).round())/100.)
        }
        self.receipt = b.clone();
        b
    }


}

pub fn format(a : f32)->f32 {
    let mut ab = a.to_string();
    ab = format!("{:.02}", a);
    let c = ab.parse::<f32>().unwrap_or(0.0);
    c
}