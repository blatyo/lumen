use super::*;

use num_traits::Num;

use crate::process::IntoProcess;

#[test]
fn with_atom_returns_bad_argument() {
    let mut process: Process = Default::default();
    let atom_term = Term::str_to_atom("😈🤘", Existence::DoNotCare, &mut process).unwrap();
    let base_term: Term = 16.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_in_base_to_integer(atom_term, base_term, &mut process),
        process
    );
}

#[test]
fn with_empty_list_returns_bad_argument() {
    let mut process: Process = Default::default();
    let base_term: Term = 16.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_in_base_to_integer(Term::EMPTY_LIST, base_term, &mut process),
        process
    );
}

#[test]
fn with_list_is_bad_argument() {
    let mut process: Process = Default::default();
    let list_term = list_term(&mut process);
    let base_term: Term = 16.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_in_base_to_integer(list_term, base_term, &mut process),
        process
    );
}

#[test]
fn with_small_integer_is_bad_argument() {
    let mut process: Process = Default::default();
    let small_integer_term = 0usize.into_process(&mut process);
    let base_term: Term = 16.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_in_base_to_integer(small_integer_term, base_term, &mut process),
        process
    );
}

#[test]
fn with_big_integer_is_bad_argument() {
    let mut process: Process = Default::default();
    let big_integer_term: Term = <BigInt as Num>::from_str_radix("18446744073709551616", 10)
        .unwrap()
        .into_process(&mut process);
    let base_term: Term = 16.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_in_base_to_integer(big_integer_term, base_term, &mut process),
        process
    );
}

#[test]
fn with_float_is_bad_argument() {
    let mut process: Process = Default::default();
    let float_term = 1.0.into_process(&mut process);
    let base_term: Term = 16.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_in_base_to_integer(float_term, base_term, &mut process),
        process
    );
}

#[test]
fn with_local_pid_is_bad_argument() {
    let mut process: Process = Default::default();
    let local_pid_term = Term::local_pid(0, 0).unwrap();
    let base_term: Term = 16.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_in_base_to_integer(local_pid_term, base_term, &mut process),
        process
    );
}

#[test]
fn with_external_pid_is_bad_argument() {
    let mut process: Process = Default::default();
    let external_pid_term = Term::external_pid(1, 0, 0, &mut process).unwrap();
    let base_term: Term = 16.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_in_base_to_integer(external_pid_term, base_term, &mut process),
        process
    );
}

#[test]
fn with_tuple_is_bad_argument() {
    let mut process: Process = Default::default();
    let tuple_term = Term::slice_to_tuple(&[], &mut process);
    let base_term: Term = 16.into_process(&mut process);

    assert_bad_argument!(
        erlang::binary_in_base_to_integer(tuple_term, base_term, &mut process),
        process
    );
}

#[test]
fn with_heap_binary_with_min_small_integer_returns_small_integer() {
    let mut process: Process = Default::default();
    let heap_binary_term = Term::slice_to_binary("-800000000000000".as_bytes(), &mut process);
    let base_term: Term = 16.into_process(&mut process);

    let integer_result =
        erlang::binary_in_base_to_integer(heap_binary_term, base_term, &mut process);

    assert_eq_in_process!(
        integer_result,
        Ok(<BigInt as Num>::from_str_radix("-576460752303423488", 10)
            .unwrap()
            .into_process(&mut process)),
        process
    );
    assert_eq!(integer_result.unwrap().tag(), Tag::SmallInteger);
}

#[test]
fn with_heap_binary_with_max_small_integer_returns_small_integer() {
    let mut process: Process = Default::default();
    let heap_binary_term = Term::slice_to_binary("7FFFFFFFFFFFFFF".as_bytes(), &mut process);
    let base_term: Term = 16.into_process(&mut process);

    let integer_result =
        erlang::binary_in_base_to_integer(heap_binary_term, base_term, &mut process);

    assert_eq_in_process!(
        integer_result,
        Ok(<BigInt as Num>::from_str_radix("576460752303423487", 10)
            .unwrap()
            .into_process(&mut process)),
        process
    );
    assert_eq!(integer_result.unwrap().tag(), Tag::SmallInteger);
}

#[test]
fn with_heap_binary_with_less_than_min_small_integer_returns_big_integer() {
    let mut process: Process = Default::default();
    let heap_binary_term = Term::slice_to_binary("-800000000000001".as_bytes(), &mut process);
    let base_term: Term = 16.into_process(&mut process);

    let integer_result =
        erlang::binary_in_base_to_integer(heap_binary_term, base_term, &mut process);

    assert_eq_in_process!(
        integer_result,
        Ok(<BigInt as Num>::from_str_radix("-576460752303423489", 10)
            .unwrap()
            .into_process(&mut process)),
        process
    );

    let integer = integer_result.unwrap();

    assert_eq!(integer.tag(), Tag::Boxed);

    let unboxed: &Term = integer.unbox_reference();

    assert_eq!(unboxed.tag(), Tag::BigInteger);
}

#[test]
fn with_heap_binary_with_greater_than_max_small_integer_returns_big_integer() {
    let mut process: Process = Default::default();
    let heap_binary_term = Term::slice_to_binary("800000000000000".as_bytes(), &mut process);
    let base_term: Term = 16.into_process(&mut process);

    let integer_result =
        erlang::binary_in_base_to_integer(heap_binary_term, base_term, &mut process);

    assert_eq_in_process!(
        integer_result,
        Ok(<BigInt as Num>::from_str_radix("576460752303423488", 10)
            .unwrap()
            .into_process(&mut process)),
        process
    );

    let integer = integer_result.unwrap();

    assert_eq!(integer.tag(), Tag::Boxed);

    let unboxed: &Term = integer.unbox_reference();

    assert_eq!(unboxed.tag(), Tag::BigInteger);
}

#[test]
fn with_subbinary_with_min_small_integer_returns_small_integer() {
    let mut process: Process = Default::default();
    // <<1::1, Integer.to_string(-576460752303423488, 16) :: binary>>
    let heap_binary_term = Term::slice_to_binary(
        &[
            150,
            156,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            0b0000_0000,
        ],
        &mut process,
    );
    let subbinary_term = Term::subbinary(heap_binary_term, 0, 1, 16, 0, &mut process);
    let base_term: Term = 16.into_process(&mut process);

    let integer_result = erlang::binary_in_base_to_integer(subbinary_term, base_term, &mut process);

    assert_eq_in_process!(
        integer_result,
        Ok(<BigInt as Num>::from_str_radix("-576460752303423488", 10)
            .unwrap()
            .into_process(&mut process)),
        process
    );
    assert_eq!(integer_result.unwrap().tag(), Tag::SmallInteger);
}

#[test]
fn with_subbinary_with_max_small_integer_returns_small_integer() {
    let mut process: Process = Default::default();
    // <<1::1, Integer.to_string(576460752303423487, 16) :: binary>>
    let heap_binary_term = Term::slice_to_binary(
        &[
            155,
            163,
            35,
            35,
            35,
            35,
            35,
            35,
            35,
            35,
            35,
            35,
            35,
            35,
            35,
            0b0000_0000,
        ],
        &mut process,
    );
    let subbinary_term = Term::subbinary(heap_binary_term, 0, 1, 15, 0, &mut process);
    let base_term: Term = 16.into_process(&mut process);

    let integer_result = erlang::binary_in_base_to_integer(subbinary_term, base_term, &mut process);

    assert_eq_in_process!(
        integer_result,
        Ok(<BigInt as Num>::from_str_radix("576460752303423487", 10)
            .unwrap()
            .into_process(&mut process)),
        process
    );
    assert_eq!(integer_result.unwrap().tag(), Tag::SmallInteger);
}

#[test]
fn with_subbinary_with_less_than_min_small_integer_returns_big_integer() {
    let mut process: Process = Default::default();
    // <<1::1, Integer.to_string(-576460752303423489, 16) :: binary>>
    let heap_binary_term = Term::slice_to_binary(
        &[
            150,
            156,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            0b1000_0000,
        ],
        &mut process,
    );
    let subbinary_term = Term::subbinary(heap_binary_term, 0, 1, 16, 0, &mut process);
    let base_term: Term = 16.into_process(&mut process);

    let integer_result = erlang::binary_in_base_to_integer(subbinary_term, base_term, &mut process);

    assert_eq_in_process!(
        integer_result,
        Ok(
            <BigInt as Num>::from_str_radix("-5764_60_752_303_423_489", 10)
                .unwrap()
                .into_process(&mut process)
        ),
        process
    );

    let integer = integer_result.unwrap();

    assert_eq!(integer.tag(), Tag::Boxed);

    let unboxed: &Term = integer.unbox_reference();

    assert_eq!(unboxed.tag(), Tag::BigInteger);
}

#[test]
fn with_subbinary_with_greater_than_max_small_integer_returns_big_integer() {
    let mut process: Process = Default::default();
    // <<1::1, Integer.to_string(576460752303423488, 16) :: binary>>
    let heap_binary_term = Term::slice_to_binary(
        &[
            156,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            24,
            0b0000_0000,
        ],
        &mut process,
    );
    let subbinary_term = Term::subbinary(heap_binary_term, 0, 1, 15, 0, &mut process);
    let base_term: Term = 16.into_process(&mut process);

    let integer_result = erlang::binary_in_base_to_integer(subbinary_term, base_term, &mut process);

    assert_eq_in_process!(
        integer_result,
        Ok(<BigInt as Num>::from_str_radix("576460752303423488", 10)
            .unwrap()
            .into_process(&mut process)),
        process
    );

    let integer = integer_result.unwrap();

    assert_eq!(integer.tag(), Tag::Boxed);

    let unboxed: &Term = integer.unbox_reference();

    assert_eq!(unboxed.tag(), Tag::BigInteger);
}
