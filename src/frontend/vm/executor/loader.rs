use std::collections::HashMap;


pub(super) fn deserialize(
    data: Vec<u8>,
) -> (
    Vec<u8>,
    Vec<u64>,
    Vec<String>,
    HashMap<String, usize>,
    usize,
) {
    let mut pos = 0;
    let const_count = (data[pos] as usize) | ((data[pos + 1] as usize) << 8);
    pos += 2;

    let mut constants = Vec::with_capacity(const_count);
    let mut string_map: HashMap<String, usize> = HashMap::new();
    let mut strings_vector: Vec<String> = Vec::new();

    for _ in 0..const_count {
        let tag = data[pos];
        pos += 1;

        match tag {
            0x01 | 0x02 => {
                // Int и Float
                let bytes: [u8; 8] = data[pos..pos + 8].try_into().unwrap();
                constants.push(u64::from_le_bytes(bytes));
                pos += 8;
            }
            0x03 => {
                // Bool
                let val = data[pos] != 0;
                constants.push(val as u64);
                pos += 1;
            }
            0x04 => {
                // Text
                let length = (data[pos] as usize) | ((data[pos + 1] as usize) << 8);
                pos += 2;
                let bytes = &data[pos..pos + length];
                pos += length;
                let string = String::from_utf8(bytes.to_vec())
                    .unwrap_or_else(|_| "Error in string converting".to_string());

                // We check if there is a string in our hashmap
                // If yes we push id to constants, creating a pointer to our string
                if let Some(id) = string_map.get(&string) {
                    constants.push(*id as u64);
                // If not, we create a unique id, by measuring the current length of the vector
                // push string.clone() to vector
                // push same string as key with value as its id
                // and push the same id to constants
                } else {
                    let new_id = strings_vector.len();
                    strings_vector.push(string.clone());
                    string_map.insert(string, new_id);
                    constants.push(new_id as u64);
                }
            }
            _ => panic!("Unknown constant tag 0x{:02X}", tag),
        }
    }

    let var_count = (data[pos] as usize) | ((data[pos + 1] as usize) << 8);
    pos += 2;
    let bytecode = data[pos..].to_vec();

    (bytecode, constants, strings_vector, string_map, var_count)
}
