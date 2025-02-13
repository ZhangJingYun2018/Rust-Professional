/*
    Find Intersection of Two Arrays
    Given two arrays, find the intersection of the arrays and return the elements of the intersection (without duplicates).
    The result should not contain any duplicate elements.

    You need to implement the function `intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the elements that are in both arrays.

    Hint: You can solve this problem using sorting, hash sets, or the two-pointer technique.
*/

use std::fmt::{self, Display, Formatter};

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // TODO: Implement the logic to find the intersection of two arrays
    // You can use sorting, hash sets, or the two-pointer technique.
   //使用set去除重复元素
    let mut set1: Vec<i32> = nums1;
    set1.sort();
    let mut set2: Vec<i32> = nums2;
    set2.sort();
    let mut result: Vec<i32> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < set1.len() && j < set2.len() {
        if set1[i] == set2[j] {
            if result.len() == 0 || result[result.len() - 1]!= set1[i] {
                result.push(set1[i]);
            }
            i += 1;
            j += 1;
        } else if set1[i] < set2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    result // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_intersection_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn test_intersection_3() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_intersection_4() {
        let nums1 = vec![1, 1, 1];
        let nums2 = vec![1, 1, 1];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_intersection_5() {
        let nums1 = vec![10, 20, 30];
        let nums2 = vec![30, 40, 50];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![30]);
    }
}
