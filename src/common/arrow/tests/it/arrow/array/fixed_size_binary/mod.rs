// Copyright 2020-2022 Jorge C. Leitão
// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_arrow::arrow::array::FixedSizeBinaryArray;
use common_arrow::arrow::bitmap::Bitmap;
use common_arrow::arrow::buffer::Buffer;
use common_arrow::arrow::datatypes::DataType;

mod mutable;

#[test]
fn basics() {
    let array = FixedSizeBinaryArray::new(
        DataType::FixedSizeBinary(2),
        Buffer::from(vec![1, 2, 3, 4, 5, 6]),
        Some(Bitmap::from([true, false, true])),
    );
    assert_eq!(array.size(), 2);
    assert_eq!(array.len(), 3);
    assert_eq!(array.validity(), Some(&Bitmap::from([true, false, true])));

    assert_eq!(array.value(0), [1, 2]);
    assert_eq!(array.value(2), [5, 6]);

    let array = array.sliced(1, 2);

    assert_eq!(array.value(1), [5, 6]);
}

#[test]
fn with_validity() {
    let a = FixedSizeBinaryArray::new(
        DataType::FixedSizeBinary(2),
        vec![1, 2, 3, 4, 5, 6].into(),
        None,
    );
    let a = a.with_validity(Some(Bitmap::from([true, false, true])));
    assert!(a.validity().is_some());
}

#[test]
fn debug() {
    let a = FixedSizeBinaryArray::new(
        DataType::FixedSizeBinary(2),
        vec![1, 2, 3, 4, 5, 6].into(),
        Some(Bitmap::from([true, false, true])),
    );
    assert_eq!(format!("{a:?}"), "FixedSizeBinary(2)[[1, 2], None, [5, 6]]");
}

#[test]
fn empty() {
    let array = FixedSizeBinaryArray::new_empty(DataType::FixedSizeBinary(2));
    assert_eq!(array.values().len(), 0);
    assert_eq!(array.validity(), None);
}

#[test]
fn null() {
    let array = FixedSizeBinaryArray::new_null(DataType::FixedSizeBinary(2), 2);
    assert_eq!(array.values().len(), 4);
    assert_eq!(array.validity().cloned(), Some([false, false].into()));
}

#[test]
fn from_iter() {
    let iter = std::iter::repeat(vec![1u8, 2]).take(2).map(Some);
    let a = FixedSizeBinaryArray::from_iter(iter, 2);
    assert_eq!(a.len(), 2);
}

#[test]
fn wrong_size() {
    let values = Buffer::from(b"abb".to_vec());
    assert!(FixedSizeBinaryArray::try_new(DataType::FixedSizeBinary(2), values, None).is_err());
}

#[test]
fn wrong_len() {
    let values = Buffer::from(b"abba".to_vec());
    let validity = Some([true, false, false].into()); // it should be 2
    assert!(FixedSizeBinaryArray::try_new(DataType::FixedSizeBinary(2), values, validity).is_err());
}

#[test]
fn wrong_data_type() {
    let values = Buffer::from(b"abba".to_vec());
    assert!(FixedSizeBinaryArray::try_new(DataType::Binary, values, None).is_err());
}

#[test]
fn to() {
    let values = Buffer::from(b"abba".to_vec());
    let a = FixedSizeBinaryArray::new(DataType::FixedSizeBinary(2), values, None);

    let extension = DataType::Extension(
        "a".to_string(),
        Box::new(DataType::FixedSizeBinary(2)),
        None,
    );
    let _ = a.to(extension);
}
