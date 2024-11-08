use tryiter::TryIteratorExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MyErr;

fn example_next(vals: Vec<Result<i32, MyErr>>) -> Result<(), MyErr> {
    for result in vals {
        let result = result?;
        println!("{}", result);
    }
    Ok(())
}

fn example_try_next(vals: Vec<Result<i32, MyErr>>) -> Result<(), MyErr> {
    let mut iter = vals.into_iter();
    while let Some(result) = iter.try_next()? {
        println!("{}", result);
    }
    Ok(())
}

#[test]
fn test_sanity() {
    let vals = vec![Ok(1), Ok(2), Ok(3), Err(MyErr), Ok(4)];

    example_next(vals.clone()).expect_err("error");
    example_try_next(vals.clone()).expect_err("error");

    // TryIteratorExt also provides helpful Result friendly methods:
    let vals: Vec<Result<i32, MyErr>> = vec![Ok(1), Ok(2), Ok(3), Ok(4)];

    // assert that all elements are less than 5
    assert_eq!(vals.iter().cloned().try_all(|x| Ok(x < 5)), Ok(true));

    // is any element equal to 3?
    assert_eq!(vals.iter().cloned().try_any(|x| Ok(x == 3)), Ok(true));

    // do a series of fallible operations
    let mut iter = vals
        .iter()
        .cloned()
        .map_ok(|x| Ok(x * 2))
        .try_filter(|x| Ok(*x < 4));
    while let Some(val) = iter.try_next().expect("error") {
        println!("{}", val);
    }

    // raise an error during processing
    vals.iter()
        .cloned()
        .map_ok(|_| Err::<i32, _>(MyErr))
        .try_next()
        .expect_err("error");
}