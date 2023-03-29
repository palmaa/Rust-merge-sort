use std::collections::HashMap;
use rand::Rng;


/*
 / Creates a size elements vector containing random i32 elements
 / input @param: size -> number of elements of the vector to be returned
 / output @param: unsorted_vector -> vector of size elements randomly initialized
 */
fn generate_random_list(size : i32, min_value: i32, max_value: i32) -> Vec<i32> {
    if size <= 0 {
        return Vec::new();
    }

    let mut unsorted_vector = Vec::new();
    let mut counter = 0;

    while counter < size {
        unsorted_vector.push(rand::thread_rng().gen_range(min_value..max_value));
        counter += 1;
    }
    
    return unsorted_vector;

}

fn sort_vector(vector : &Vec<i32>) -> Vec<i32> {
    return merge_sort(&vector)
}

/*
 / Recursive merge sort function
 / input @param: vector -> reference to the vector that has to be sorted
 / output @param: vector -> sorted vector (from minor to major)
 */
fn merge_sort(vector : &Vec<i32>) -> Vec<i32> {

    if vector.len() < 2 {
        return vector.to_vec();
    }

    let size = vector.len() / 2;
    let left_vector = merge_sort(&vector[0..size].to_vec());
    let right_vector = merge_sort(&vector[size..].to_vec());
    let merged = merge(&left_vector, &right_vector);
    return merged;
}

/*
 / Recives two vectors, compares them position to position and return a vector resulting of sorting both input vectors
 / input @param: left_vector -> reference to one vector
 / input @param: right_vector -> reference to one vector
 / output @param: merged_vector -> result of sorting and merging left and right vector 
 */
fn merge(left_vector : &Vec<i32>, right_vector : &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged_vector : Vec<i32> = Vec::new();

    while i < left_vector.len() && j < right_vector.len() {
        if left_vector[i] < right_vector[j]{
            merged_vector.push(left_vector[i]);
            i = i + 1;
            continue;
        }

        merged_vector.push(right_vector[j]);
        j = j + 1;
        continue;
    }

    // Push to the merged vector the remaning values from left or right vector
    if i < left_vector.len(){
        while i < left_vector.len() {
            merged_vector.push(left_vector[i]);
            i = i + 1;
        }
    }
    else {
        while j < right_vector.len() {
            merged_vector.push(right_vector[j]);
            j = j + 1;
        }
    }

    return merged_vector;
}


/*
 / Return the median value of the vector (the one in the middle)
 / input @param: vector -> reference to the vector
 / output @param: element -> the median element of the vector
 */
fn median_value(vector : &Vec<i32>) -> i32 {
    let mid_index = vector.len() / 2;
    return vector[mid_index];
}

/*
 / Return the value that occurs the most in the vector and the number of times it occurs
 / input @param: vector  -> vector reference
 / output @param: (i32, i32) -> (number, ocurrences)
 */

fn mode_value(vector : Vec<i32>) -> (i32, i32) {
    let mut map : HashMap<i32, i32> = HashMap::new();
    let size = vector.len();
    let mut index = 0;
    let mut value : i32;

    // Map each value with the number of ocurrences
    while index < size {
        value = vector[index];
        index = index + 1;
        if map.contains_key(&value){
            let apperances = map.get_key_value(&value);
            match apperances {
                Some(x) => {
                    let updated_apperances = x.1 + 1;
                    map.insert(value, updated_apperances);
                },
                None => println!("Error getting the key for: {value}.\n"),
            }
            continue;
        }
        map.insert(value, 1);
    }

    // Return the value that occurs the most and the times it occurs
    let mut result : (i32, i32) = (0, 0);
    for (key, value) in map.iter_mut(){
        if *value > result.1 {
            result.0 = *key;
            result.1 = *value;
        }
    }
    
    return result;

}

fn check_sorted(vector : &Vec<i32>) -> bool {
    let mut index = 1;

    while index < vector.len() {
        if vector[index] < vector[index-1]{
            return false;
        }
        index = index + 1;
    }

    return true;
}

/*
 / It creates a vector, sorts it, and print the median and the most frecuent value
 */
fn main() {
    
    println!("Creating unsorted vector...\n");
    let unsorted_vector = generate_random_list(4, -999, 999);

    // println!("The unsorted vector is: [ \n");
    // for element in &unsorted_vector {
    //     println!("{element}");
    // }
    // println!("]. \n");
    
    println!("Sorting...");

    let mut sorted_vector : Vec<i32> = Vec::new();
    sorted_vector = sort_vector(&unsorted_vector);
    
    if check_sorted(&sorted_vector){
        println!("Sorted! \n");
    }

    // println!("The sorted vector is: [ \n");
    // for element in &sorted_vector {
    //     println!("{element}");
    // }
    // println!("]. \n");

    let median_value : i32 = median_value(&sorted_vector);
    println!("The median value (the one in the middle position) of the sorted vector is: {median_value}\n");

    let mode : (i32, i32) = mode_value(sorted_vector);
    println!("The value that appears the most is {:?} with {:?} apperances!", mode.0, mode.1);

}
