extern crate xgboost_runner;

use xgboost_runner::Runner;

#[test]
fn test_add() {
    let runner = Runner::new(String::from("ätsch"), 
                             vec![String::from("f0"), 
                                  String::from("f1"), 
                                  String::from("f2"), 
                                  String::from("f3"), 
                                  String::from("f4")]);
    let result = runner.predict(vec![0.0, 0.2, 0.4, 0.6, 0.8]);
    assert_eq!(result, 10.1);
}