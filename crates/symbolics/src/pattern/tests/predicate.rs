// ------------------------------------------------------------------------------------------------------
// PREDICATE TESTS
// ------------------------------------------------------------------------------------------------------
//
//  Pattern                                                               | Test Expr | Expected Matches
//  ----------------------------------------------------------------------|-----------|------------------
//  PatternTest[Blank[], IsNumber]                                       | 5         | 1
//  PatternTest[Blank[], IsNumber]                                       | foo       | 0
//  PatternTest[Blank[], IsNumber]                                       | f[1]      | 0
//  PatternTest[Blank[], IsSymbol]                                       | foo       | 1
//  PatternTest[Blank[], IsSymbol]                                       | 5         | 0
//  f[PatternTest[Blank[], IsNumber]]                                    | f[3]      | 1
//  f[PatternTest[Blank[], IsNumber]]                                    | f[x]      | 0
//  f[PatternTest[Blank[], IsNumber], PatternTest[Blank[], IsSymbol]]   | f[1, x]   | 1
//  f[PatternTest[Blank[], IsNumber], PatternTest[Blank[], IsSymbol]]   | f[x, 1]   | 0
//  f[PatternTest[Pattern[x, Blank[]], IsNumber]]                        | f[3]      | 1
//  f[PatternTest[Pattern[x, Blank[]], IsNumber]]                        | f[foo]    | 0
//  Add[PatternTest[Blank[], IsNumber], PatternTest[Blank[], IsSymbol]] | Add[x, 1] | 1
//  Add[PatternTest[Blank[], IsNumber], PatternTest[Blank[], IsSymbol]] | Add[1, 2] | 0

use crate::norm_expr;
use crate::pattern::tests::utils::count_matches;

#[test]
fn test_predicate_1() {
    let pattern = norm_expr! { PatternTest[Blank[], IsNumber] };
    let subject = norm_expr! { 5 };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_2() {
    let pattern = norm_expr! { PatternTest[Blank[], IsNumber] };
    let subject = norm_expr! { foo };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_3() {
    let pattern = norm_expr! { PatternTest[Blank[], IsNumber] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_4() {
    let pattern = norm_expr! { PatternTest[Blank[], IsSymbol] };
    let subject = norm_expr! { foo };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_5() {
    let pattern = norm_expr! { PatternTest[Blank[], IsSymbol] };
    let subject = norm_expr! { 5 };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_6() {
    let pattern = norm_expr! { f[PatternTest[Blank[], IsNumber]] };
    let subject = norm_expr! { f[3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_7() {
    let pattern = norm_expr! { f[PatternTest[Blank[], IsNumber]] };
    let subject = norm_expr! { f[x] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_8() {
    let pattern = norm_expr! { f[PatternTest[Blank[], IsNumber], PatternTest[Blank[], IsSymbol]] };
    let subject = norm_expr! { f[1, x] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_9() {
    let pattern = norm_expr! { f[PatternTest[Blank[], IsNumber], PatternTest[Blank[], IsSymbol]] };
    let subject = norm_expr! { f[x, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_10() {
    let pattern = norm_expr! { f[PatternTest[Pattern[x, Blank[]], IsNumber]] };
    let subject = norm_expr! { f[3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_11() {
    let pattern = norm_expr! { f[PatternTest[Pattern[x, Blank[]], IsNumber]] };
    let subject = norm_expr! { f[foo] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_12() {
    let pattern =
        norm_expr! { Add[PatternTest[Blank[], IsNumber], PatternTest[Blank[], IsSymbol]] };
    let subject = norm_expr! { Add[x, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_13() {
    let pattern =
        norm_expr! { Add[PatternTest[Blank[], IsNumber], PatternTest[Blank[], IsSymbol]] };
    let subject = norm_expr! { Add[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}
