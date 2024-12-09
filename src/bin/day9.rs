use aoc2024::fetch_or_load_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 9;
    let input = fetch_or_load_input(day)?;
    let (files, frees) = parse_input(&input)?;

    {
        let initial_array = build_initial_array(&files, &frees);
        let total_length: usize = files.iter().map(|v| *v as usize).sum();

        let final_array = fill_free_spaces_from_back(&initial_array, total_length);
        let checksum = compute_checksum(&final_array);
        println!("Checksum part one: {}", checksum);
    }

    {
        let mut final_array = build_initial_array(&files, &frees);
        move_files_left(&files, &mut final_array);
        let checksum_part_two = compute_checksum(&final_array);
        println!("Checksum part two: {}", checksum_part_two);
    }

    Ok(())
}

fn parse_input(line: &str) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let bytes = line.trim().as_bytes();

    let mut files = Vec::new();
    let mut frees = Vec::new();

    let length = bytes.len();
    if length == 0 {
        return Ok((files, frees)); // empty input means no files
    }

    let mut i = 0;
    while i < length {
        let file_count = (bytes[i] - b'0') as u8;
        i += 1;

        let free_count = if i < length {
            // We have a free count available
            let fc = (bytes[i] - b'0') as u8;
            i += 1;
            fc
        } else {
            // No free count provided after the last file, assume 0
            0
        };

        files.push(file_count);
        frees.push(free_count);
    }

    Ok((files, frees))
}

fn build_initial_array(files: &[u8], frees: &[u8]) -> Vec<Option<usize>> {
    let total_length: usize = files
        .iter()
        .zip(frees.iter())
        .map(|(f, fr)| (*f as usize + *fr as usize))
        .sum();
    let mut initial_array = Vec::with_capacity(total_length);

    for (file_id, (&fcount, &frcount)) in files.iter().zip(frees.iter()).enumerate() {
        // file blocks
        for _ in 0..fcount {
            initial_array.push(Some(file_id));
        }
        // free blocks as None
        for _ in 0..frcount {
            initial_array.push(None);
        }
    }

    initial_array
}

fn fill_free_spaces_from_back(
    initial_array: &[Option<usize>],
    total_length: usize,
) -> Vec<Option<usize>> {
    fn get_next_non_empty_block_from_back(array: &[Option<usize>], start: usize) -> (usize, usize) {
        let mut idx = start - 1;
        loop {
            if let Some(value) = array[idx] {
                return (value, idx);
            }
            idx -= 1;
        }
    }

    let mut final_array = Vec::new();
    let mut back_idx = initial_array.len();

    for i in 0..total_length {
        if let Some(file_id) = initial_array[i] {
            final_array.push(Some(file_id))
        } else {
            let (file_id, next_idx) = get_next_non_empty_block_from_back(&initial_array, back_idx);
            final_array.push(Some(file_id));
            back_idx = next_idx;
        }
    }

    final_array
}

/// Move files to the left starting from the last file.
/// A file is identified by its file_id and occurs in one contiguous segment.
/// We try to find a contiguous free space at the left of the array large enough to hold that file.
/// If found, move the file there and mark old position as free.
fn move_files_left(files: &[u8], final_array: &mut [Option<usize>]) {
    let num_files = files.len();

    // Process files in reverse order: last file first
    for fid in (0..num_files).rev() {
        let file_size = files[fid] as usize;
        if file_size == 0 {
            continue; // no blocks for this file, skip
        }

        // Find the file's segment in final_array
        if let Some(start) = find_file_segment_start(final_array, fid) {
            if let Some(free_start) = find_free_segment(final_array, file_size, start) {
                // Move the file to [free_start .. free_start+file_size]
                let end = start + file_size;
                move_file_segment(final_array, start, end, free_start);
            }
        }
    }
}

/// Find the file segment for a given file_id in array.
fn find_file_segment_start(final_array: &[Option<usize>], file_id: usize) -> Option<usize> {
    // find first occurrence of file_id
    Some(final_array.iter().position(|&b| b == Some(file_id))?)
}

/// Find a contiguous free segment of at least `size` blocks to the left of `limit_index`.
/// We search from the beginning to limit_index for a run of free blocks large enough.
fn find_free_segment(
    final_array: &[Option<usize>],
    size: usize,
    limit_index: usize,
) -> Option<usize> {
    let mut count = 0;
    let mut start = 0;

    for i in 0..limit_index {
        if final_array[i].is_none() {
            count += 1;
            if count == 1 {
                start = i;
            }
            if count >= size {
                return Some(start);
            }
        } else {
            count = 0;
        }
    }

    None
}

/// Move file segment [file_start..file_end) to [target_start..target_start+(file_end-file_start)).
/// We'll overwrite the target range with the file's blocks and mark the old range as free.
fn move_file_segment(
    final_array: &mut [Option<usize>],
    file_start: usize,
    file_end: usize,
    target_start: usize,
) {
    let length = file_end - file_start;

    let mut file_blocks = Vec::with_capacity(length);
    file_blocks.extend_from_slice(&final_array[file_start..file_end]);

    for (i, &b) in file_blocks.iter().enumerate() {
        final_array[target_start + i] = b;
    }

    for i in file_start..file_end {
        final_array[i] = None;
    }
}

/// Compute checksum ignoring free blocks.
fn compute_checksum(array: &[Option<usize>]) -> i64 {
    let mut checksum = 0i64;
    for (i, &val) in array.iter().enumerate() {
        if let Some(file_id) = val {
            checksum += i as i64 * file_id as i64;
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "2333133121414131402";

    #[test]
    fn test_part_one() {
        let (files, frees) = parse_input(TEST_DATA).unwrap();
        let initial_array = build_initial_array(&files, &frees);
        let total_length: usize = files.iter().map(|v| *v as usize).sum();
        let final_array = fill_free_spaces_from_back(&initial_array, total_length);
        let checksum = compute_checksum(&final_array);

        let expected_checksum = 1928;
        assert_eq!(checksum, expected_checksum);
    }

    #[test]
    fn test_part_two() {
        let (files, frees) = parse_input(TEST_DATA).unwrap();
        let mut final_array = build_initial_array(&files, &frees);
        move_files_left(&files, &mut final_array);
        let checksum = compute_checksum(&final_array);

        let expected_checksum = 2858;
        assert_eq!(checksum, expected_checksum);
    }
}
