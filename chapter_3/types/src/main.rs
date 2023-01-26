fn main() {
    let _number_int: i32 = -1000; // integer
    let _number_uint: u32 = 1_000; // unsigned integer _ can be used for better readability; 1_000 == 1000
    let _number_float: f32 = 3.2; // float number

    let _bool_variable: bool = true;

    let _char_variable: char = 'a';

    // tuples

    let tuple: (i32, f32) = (10, 10.5); // tuples - general way of combining multiple variables
                                        // with different types;

    let _tuple_first = tuple.0; // 10; returns variables from tuple by index;
    let (_a, _b) = tuple; // destructuring variables from tuples;

    // arrays

    let _array_variable: [i32; 5] = [1, 2, 3, 4, 5]; // list of elements with same type; have a fixed length;
    let _array_same_variable = [3; 5]; // generates array of the same elements [3, 3, 3, 3, 3];

    let _array_first_element = _array_variable[0]; // 1; get element by index;
}
