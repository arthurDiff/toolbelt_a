extern crate toolbelt_a;

#[cfg(feature = "proc_macro")]
use toolbelt_a::proc_macro::comp;

#[cfg(feature = "proc_macro")]
#[test]
fn test_comp_map_values() {
    let result = comp![x*x for x in [1,2,3]].collect::<Vec<_>>();
    assert_eq!(result, [1, 4, 9]);

    let result = comp![(num * 2, String::from("hi") + t) for (num, t) in [(4, "hello"),(5, "world"),(6, "person")]].collect::<Vec<_>>();
    assert_eq!(
        result,
        [
            (8, String::from("hihello")),
            (10, String::from("hiworld")),
            (12, String::from("hiperson"))
        ]
    );

    let result = comp![num*8 for num in [2,3,4,5,7] if num % 2 == 0].collect::<Vec<_>>();
    assert_eq!(result, [16, 32]);
}

#[cfg(feature = "proc_macro")]
#[test]
fn test_comp_filters_values() {
    let result = comp![num*8 for num in [2,3,4,5,7] if num % 2 == 0].collect::<Vec<_>>();
    assert_eq!(result, [16, 32]);

    let result = comp![x for x in ["hello", "world", "bye", "bye"] if x.chars().count() > 4]
        .collect::<Vec<&str>>();
    assert_eq!(result, ["hello", "world"]);
}

#[cfg(feature = "proc_macro")]
#[test]
fn test_comp_multiple_for_if_clauses() {
    let result = comp![(x, y) for x in [1,2,3] for y in [6,7,8]].collect::<Vec<(_, _)>>();

    assert_eq!(
        result,
        [
            (1, 6),
            (1, 7),
            (1, 8),
            (2, 6),
            (2, 7),
            (2, 8),
            (3, 6),
            (3, 7),
            (3, 8)
        ]
    )
}
