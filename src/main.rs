#[macro_use]
extern crate mysql;

use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}


// "Update TABLE tblpayment set account_name=:account_name, amount=:amount where  customer_id=:customer_id"
fn main()
{
    //insert();
    //fetch();

    // let pool = my::Pool::new("mysql://root:@localhost:3306/todo").unwrap();
    // let payments = vec![
    //     Payment { customer_id: 1, amount: 4000, account_name: Some("xyz".into()) },
    // ];
    // for mut stmt in pool.prepare(r"Update tblpayment set account_name=:account_name, amount=:amount where  customer_id=:customer_id").into_iter() 
    //                 {
    //             for p in payments.iter() {
    //                 // `execute` takes ownership of `params` so we pass account name by reference.
    //                 // Unwrap each result just to make sure no errors happened.
    //                 stmt.execute(params!{
    //                     "customer_id" => p.customer_id,
    //                     "amount" => p.amount,
    //                     "account_name" => &p.account_name,
    //                 }).unwrap();
    //                 println!("Inner");
    //             }
    //     println!("Outter");
    // }
     let pool = my::Pool::new("mysql://root:@localhost:3306/todo").unwrap();
    let payments = vec![
        Payment { customer_id: 1, amount: 4000, account_name: Some("xyz".into()) },
    ];
    for mut stmt in pool.prepare(r"Delete FROM tblpayment where customer_id=:customer_id").into_iter() 
                    {
                for p in payments.iter() {
                    // `execute` takes ownership of `params` so we pass account name by reference.
                    // Unwrap each result just to make sure no errors happened.
                    stmt.execute(params!{
                        "customer_id" => p.customer_id,
                        "amount" => p.amount,
                        "account_name" => &p.account_name,
                    }).unwrap();
                    println!("Inner");
                }
        println!("Outter");
    }
}

fn fetch()
{
    let pool = my::Pool::new("mysql://root:@localhost:3306/todo").unwrap();

    let selected_payments: Vec<Payment> =
    pool.prep_exec("SELECT customer_id, amount, account_name FROM tblpayment", ())
    .map(|result| { 
        
        result.map(|x| x.unwrap()).map(|row| {
            let (customer_id, amount, account_name) = my::from_row(row);
            Payment {
                customer_id: customer_id,
                amount: amount,
                account_name: account_name,
            }
        }).collect()
    }).unwrap();
    println!("{:#?}",selected_payments);
    println!("Yay!");
}

fn insert(){
    let pool = my::Pool::new("mysql://root:@localhost:3306/todo").unwrap();
    let payments = vec![
        Payment { customer_id: 1, amount: 20000, account_name: None },
        Payment { customer_id: 3, amount: 4000, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6000, account_name: None },
        Payment { customer_id: 7, amount: 800, account_name: None },
        Payment { customer_id: 9, amount: 1000, account_name: Some("bar".into()) },
    ];

for mut stmt in pool.prepare(r"INSERT INTO tblpayment
                    (customer_id, amount, account_name)
                VALUES
                    (:customer_id, :amount, :account_name)").into_iter() 
                    {
                for p in payments.iter() {
                    // `execute` takes ownership of `params` so we pass account name by reference.
                    // Unwrap each result just to make sure no errors happened.
                    stmt.execute(params!{
                        "customer_id" => p.customer_id,
                        "amount" => p.amount,
                        "account_name" => &p.account_name,
                    }).unwrap();
                    println!("Inner");
                }
        println!("Outter");
    }
}






