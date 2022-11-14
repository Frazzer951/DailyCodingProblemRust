/* HARD
Suppose we represent our file system by a string in the following manner:

The string "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext" represents:

dir
    subdir1
    subdir2
        file.ext


The directory dir contains an empty sub-directory subdir1 and a sub-directory
subdir2 containing a file file.ext.

The string
"dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
represents:

dir
    subdir1
        file1.ext
        subsubdir1
    subdir2
        subsubdir2
            file2.ext


The directory dir contains two sub-directories subdir1 and subdir2. subdir1
contains a file file1.extand an empty second-level sub-directory subsubdir1.
subdir2 contains a second-level sub-directorysubsubdir2 containing a file
file2.ext.

We are interested in finding the longest (number of characters) absolute path to
a file within our file system. For example, in the second example above, the
longest absolute path is "dir/subdir2/subsubdir2/file2.ext", and its length is
32 (not including the double quotes).

Given a string representing the file system in the above format, return the
length of the longest absolute path to a file in the abstracted file system. If
there is no file in the system, return 0.

Note:

The name of a file contains at least a period and an extension.

The name of a directory or sub-directory will not contain a period.
*/

use std::collections::HashMap;

fn is_file(f: &str) -> bool {
    f.contains('.')
}

fn find_longest_filepath(filesystem: String) -> String {
    let file_paths = filesystem.split('\n');

    let mut depths: HashMap<usize, &str> = HashMap::new();
    let mut filepaths: Vec<String> = Vec::new();

    for path in file_paths {
        let depth = path.matches('\t').count();
        if is_file(path) {
            let mut filepath = String::new();
            for i in 0..depth {
                filepath.push_str(depths[&i]);
                filepath.push('/');
            }
            filepath.push_str(path.trim_matches('\t'));
            filepaths.push(filepath);
        } else {
            depths.insert(depth, path.trim_matches('\t'));
        }
    }
    let longest_filepath = filepaths.iter().max_by_key(|x| x.len()).unwrap();
    longest_filepath.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_017() {
        assert_eq!(
            find_longest_filepath(String::from(
                "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
            )),
            String::from("dir/subdir2/subsubdir2/file2.ext")
        );
    }
}
