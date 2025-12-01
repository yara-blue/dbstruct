// #[dbstruct::dbstruct(db=btreemap)]
// pub struct Test {
//     numbers: Vec<u32>,
//     letters: Vec<String>,
// }

// #[test]
// fn clear() {
//     let db = Test::new().unwrap();

//     let primes = [2, 3, 5, 7];
//     db.numbers().extend(&primes).unwrap();
//     db.numbers().clear().unwrap();
//     db.numbers().push(&3).unwrap();
//     db.numbers().push(&5).unwrap();
//     db.numbers().push(&2).unwrap();
//     db.numbers().push(&7).unwrap();

//     assert_eq!(Some(7), db.numbers().pop().unwrap());
//     assert_eq!(Some(2), db.numbers().pop().unwrap());

//     db.numbers().clear().unwrap();
//     assert_eq!(db.numbers().len(), 0);
// }

mod drops_in_between {
    #[dbstruct::dbstruct(db=trait)]
    pub struct Test {
        numbers: Vec<u32>,
        letters: Vec<String>,
    }

    #[test]
    fn push_then_pop() {
        let ds = dbstruct::stores::BTreeMap::new();
        let db = Test::open(ds).unwrap();
        db.numbers().push(&3).unwrap();
        db.numbers().push(&5).unwrap();

        let ds = db.into_inner();
        let db = Test::open(ds).unwrap();
        assert_eq!(Some(5), db.numbers().pop().unwrap());
        assert_eq!(Some(3), db.numbers().pop().unwrap());
    }
}

//     #[test]
//     fn push_then_iter() {
//         let ds = dbstruct::stores::BTreeMap::new();
//         let ds = {
//             let db = Test::open(ds).unwrap();
//             db.numbers().push(&3).unwrap();
//             db.numbers().push(&5).unwrap();
//             assert_eq!(db.numbers().len(), 2);
//             db.into_inner()
//         };

//         let db = Test::open(ds).unwrap();
//         assert_eq!(db.numbers().len(), 2);
//         assert_eq!(
//             db.numbers().iter().collect::<Result<Vec<_>, _>>().unwrap(),
//             vec![3, 5]
//         );
//     }
// }

// mod given_empty {
//     use super::*;

//     #[test]
//     fn len_is_zero() {
//         let db = Test::new().unwrap();
//         assert_eq!(db.numbers().len(), 0);
//     }

//     #[test]
//     fn push_increases_the_len() {
//         let db = Test::new().unwrap();
//         db.numbers().push(&42).unwrap();
//         assert_eq!(db.numbers().len(), 1)
//     }

//     #[test]
//     fn pop_return_none() {
//         let db = Test::new().unwrap();
//         let elem = db.numbers().pop().unwrap();
//         assert_eq!(elem, None)
//     }

//     #[test]
//     fn get_return_none() {
//         let db = Test::new().unwrap();
//         let elem = db.numbers().get(0).unwrap();
//         assert_eq!(elem, None)
//     }
// }

// mod push_pop {
//     use super::*;

//     #[test]
//     fn len_is_two() {
//         let db = Test::new().unwrap();
//         db.numbers().push(&42).unwrap();
//         db.numbers().push(&43).unwrap();

//         assert_eq!(db.numbers().len(), 2);
//     }

//     #[test]
//     fn element_pop_in_the_right_order() {
//         let db = Test::new().unwrap();
//         db.numbers().push(&42).unwrap();
//         db.numbers().push(&43).unwrap();

//         assert_eq!(db.numbers().pop().unwrap(), Some(43));
//         assert_eq!(db.numbers().pop().unwrap(), Some(42));
//     }

//     #[test]
//     fn third_pop_is_none() {
//         let db = Test::new().unwrap();
//         db.numbers().push(&42).unwrap();
//         db.numbers().push(&43).unwrap();

//         db.numbers().pop().unwrap();
//         db.numbers().pop().unwrap();
//         let elem = db.numbers().pop().unwrap();
//         assert_eq!(elem, None)
//     }

//     #[test]
//     fn get_returns_some() {
//         let db = Test::new().unwrap();
//         db.numbers().push(&42).unwrap();
//         db.numbers().push(&43).unwrap();

//         assert!(db.numbers().get(0).unwrap().is_some());
//         assert!(db.numbers().get(1).unwrap().is_some());
//         assert!(db.numbers().get(0).unwrap().is_some());
//     }
// }

// mod iterator {
//     use super::*;

//     #[test]
//     fn trivial() {
//         let db = Test::new().unwrap();
//         db.numbers().push(&42).unwrap();
//         db.numbers().push(&13).unwrap();
//         db.numbers().push(&7).unwrap();

//         let mut sum = 0;
//         for elem in &db.numbers() {
//             sum += elem.unwrap();
//         }
//         assert_eq!(sum, 62);
//     }

//     #[test]
//     fn push_back_post_iter() {
//         let db = Test::new().unwrap();
//         db.numbers().push(&42).unwrap();
//         db.numbers().push(&13).unwrap();

//         let list = db.numbers();
//         let iter = list.into_iter();
//         db.numbers().push(&7).unwrap();

//         let mut sum = 0;
//         for elem in iter {
//             sum += elem.unwrap();
//         }
//         assert_eq!(sum, 62);
//     }

//     #[test]
//     fn pop_back_post_iter_is_seen() {
//         let db = Test::new().unwrap();
//         db.numbers().push(&42).unwrap();
//         db.numbers().push(&13).unwrap();

//         let mut sum = 0;
//         let list = db.numbers();
//         let iter = list.into_iter();
//         db.numbers().pop().unwrap();

//         for elem in iter {
//             sum += elem.unwrap();
//         }
//         assert_eq!(sum, 42);
//     }

//     #[test]
//     fn pop_back_during_iter() {
//         let db = Test::new().unwrap();
//         db.numbers().push(&42).unwrap();
//         db.numbers().push(&13).unwrap();

//         let list = db.numbers();
//         let mut iter = list.into_iter();
//         iter.next();
//         iter.next();
//         db.numbers().pop().unwrap();
//         assert!(iter.next().is_none());
//     }
// }

// mod extend {
//     use super::*;

//     #[test]
//     fn push_str_slices() {
//         let db = Test::new().unwrap();

//         let iter = ["a", "b", "c", "d"];
//         db.letters().extend(iter).unwrap();
//         assert_eq!(db.letters().len(), 4);
//         assert_eq!(db.letters().pop().unwrap(), Some("d".to_string()));
//     }

//     #[test]
//     fn push_strings() {
//         let db = Test::new().unwrap();

//         let iter = [
//             "a".to_owned(),
//             "b".to_owned(),
//             "c".to_owned(),
//             "d".to_owned(),
//         ];
//         db.letters().extend(&iter).unwrap();
//     }
// }
