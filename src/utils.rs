pub mod csv {
    // ####################
    // CSV UTILS
    // ####################
    pub trait ToCSV {
        fn to_csv(&self) -> String;
    }

    pub trait FromCSV {
        fn from_csv(&self) -> Self;
    }

    pub fn from_vec<T: ToString>(vec: &Vec<T>) -> String {
        vec.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    #[cfg(test)]
    mod tests {
        use super::from_vec;

        #[test]
        fn test_from_vec() {
            let numbers = vec![1, 2, 3];
            let result = from_vec(&numbers);
            let expected = String::from("1,2,3");
            assert_eq!(result, expected);
        }
    }
}
