pub fn remove_trailing_zeros(mut vec: Vec<u8>) -> Vec<u8> {
    let last_non_zero_index = vec
        .iter()
        .rfind(|&&x| x != 0)
        .map(|x| std::ptr::from_ref::<u8>(x) as usize - vec.as_ptr() as usize);
    if let Some(index) = last_non_zero_index {
        vec.truncate(index / std::mem::size_of::<u8>() + 1);
    } else {
        vec.clear();
    }
    vec
}