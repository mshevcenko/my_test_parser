use my_test_parser_mshevchenko::list_parser;

pub fn main() {
    println!("{:?}", list_parser::list("[1,1,2,3,5,8]"));
        assert_eq!(list_parser::list("[1,1,2,3,5,8]"), Ok(vec![1, 1, 2, 3, 5, 8]));
}