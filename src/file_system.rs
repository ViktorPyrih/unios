use crate::shell::{MAX_ARRAY_SIZE};
use crate::{print};

const MAX_DIRS : usize = 20;
const MAX_CHILDREN : usize = 10;
pub const NAME_SIZE : usize = 16;
pub const LINE_END : u8 = '^' as u8;

pub struct FileSystem {
    dirs_count: usize,
    dirs: [Directory; MAX_DIRS],
    curr_dir: usize,
    last_index: usize
}

#[derive(Debug, Clone, Copy)]
pub struct Directory {
    index: usize,
    name: [u8; NAME_SIZE],
    parent_index: usize,
    child_count: usize,
    child_indexes: [usize; MAX_CHILDREN]
}

impl FileSystem {

    pub fn new(array: [u8; MAX_ARRAY_SIZE]) -> FileSystem {
        let directories_count : usize = array[0] as usize;
        if directories_count == 0 {
            FileSystem::new_empty()
        } else {
            deserialize(array)
        }
    }

    pub fn new_empty() -> FileSystem {
        let directories = [Directory::new_root(); MAX_DIRS];

        FileSystem {
            dirs_count: 1,
            dirs: directories,
            curr_dir: 0,
            last_index: 0
        }
    }

    pub fn get_arr(self) -> [u8; MAX_ARRAY_SIZE] {
        serialize(self)
    }

    pub fn execute_command(&mut self,
        text_left: [u8; NAME_SIZE], text_right: [u8; NAME_SIZE]) {

        if compare_text_arrs(text_left, str_name_to_arr("pwd")) {
            self.cmd_curr_dir();
        } else if compare_text_arrs(text_left, str_name_to_arr("mkdir")) {
            self.cmd_make_dir(text_right);
        } else if compare_text_arrs(text_left, str_name_to_arr("cd")) {
            if compare_text_arrs(text_right, str_name_to_arr("..")) {
                self.cmd_dir_back();
            } else {
                self.cmd_change_dir(text_right);
            }
        } else if compare_text_arrs(text_left, str_name_to_arr("rm")) || compare_text_arrs(text_left, str_name_to_arr("del")) {
            self.cmd_remove_dir(text_right);
        } else if compare_text_arrs(text_left, str_name_to_arr("tree")) {
            self.cmd_get_dir_tree();
        } else {
            print!("[Failed] Command: '");
            print_name(text_left, false, false);
            print!("' is not supported\n");
        }
    }

    // commands

    fn cmd_remove_dir(&mut self, name: [u8; NAME_SIZE]) {

        let (name_found_result, delete_dir_index) = self.find_child_by_name(name);
        if name_found_result {

            let delete_dir_name = self.dirs[self.find_dir_index(delete_dir_index)].name;
            self.cascade_dir_delete(delete_dir_index);

            print!("[OK] Deleted directory: '");
            print_name(delete_dir_name, false, false);
            print!("'\n");
        } else {
            print!("[Failed] Cannot find directory '");
            print_name(name, false, false);
            print!("'\n");
        }
    }

    fn cmd_change_dir(&mut self, name: [u8; NAME_SIZE]) {

        let (name_found_result, new_current_dir_index) = self.find_child_by_name(name);
        if name_found_result {

            self.curr_dir = new_current_dir_index;
            let curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];

            print!("[OK] Changed current directory to: '");
            print_name(curr_dir.name, false, false);
            print!("'\n");
        } else {
            print!("[Failed] Cannot find directory '");
            print_name(name, false, false);
            print!("'\n");
        }
    }

    fn cmd_dir_back(&mut self) {

        let mut curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];

        if curr_dir.index != curr_dir.parent_index {

            self.curr_dir = curr_dir.parent_index;
            curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];

            print!("[OK] Changed current directory to: '");
            print_name(curr_dir.name, false, false);
            print!("'\n");
        } else {
            print!("[Failed] No parent found for directory: '");
            print_name(curr_dir.name, false, false);
            print!("'\n");
        }
    }

    fn cmd_make_dir(&mut self, name: [u8; NAME_SIZE]) {

        if is_name_empty(name) {
            print!("[Failed] Cannot create directory with the empty name\n");
            return;
        }

        let (name_found_result, _dir_with_same_name_index) = self.find_child_by_name(name);
        if name_found_result {
            print!("[Failed] Directory with the same name already exists\n");
            return;
        }

        let curr_dir_index = self.find_dir_index(self.curr_dir);
        let curr_dir = self.dirs[curr_dir_index];
        if curr_dir.child_count == MAX_CHILDREN {
            print!("[Failed] Cannot create more than {} entries in the directory\n", MAX_CHILDREN);
            return;
        }
        if self.dirs_count == MAX_DIRS {
            print!("[Failed] Cannot create more than {} directories\n", MAX_DIRS);
            return;
        }

        self.last_index += 1;
        self.dirs_count += 1;
        self.dirs[self.dirs_count - 1] = self.dirs[curr_dir_index].new_child(self.last_index, name);

        print!("[OK] Created new directory: '");
        print_name(self.dirs[self.dirs_count - 1].name, false, false);
        print!("'\n");
    }

    fn cmd_curr_dir(&mut self) {
        let curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];
        self.read_dirs_back_recursively(curr_dir);
    }

    fn cmd_get_dir_tree(&mut self) {
        let curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];
        print_name(curr_dir.name, true, true);
        self.read_dirs_front_recursively(curr_dir, 1);
    }

    fn read_dirs_front_recursively(&mut self, parent_dir: Directory, nesting: usize) {

        for i in 0..parent_dir.child_count {
            let dir_index = self.find_dir_index(parent_dir.child_indexes[i]);
            let child = self.dirs[dir_index];

            for _i in 0..nesting {
                let backspaces_count = 4;
                for _j in 0..backspaces_count {
                    print!(" ");
                }
            }
            print_name(child.name, true, true);
            self.read_dirs_front_recursively(child, nesting + 1);
        }
    }

    fn read_dirs_back_recursively(&mut self, child: Directory) -> usize {
        let parent = self.get_parent(child);
        let mut nesting : usize = 0;
        if parent.index != child.index {
            nesting = self.read_dirs_back_recursively(parent);
        }

        for _i in 0..nesting {
            let backspaces_count = 4;
            for _j in 0..backspaces_count {
                print!(" ");
            }
        }
        print_name(child.name, true, true);
        return nesting + 1;
    }

    fn get_parent(&mut self, child: Directory) -> Directory {
        return self.dirs[self.find_dir_index(child.parent_index)];
    }

    fn find_dir_index(&mut self, index: usize) -> usize {
        for i in 0..self.dirs_count {
            if self.dirs[i].index == index {
                return i;
            }
        }
        return 0;
    }

    fn find_child_by_name(&mut self, name: [u8; NAME_SIZE]) -> (bool, usize) {

        let curr_dir = self.dirs[self.find_dir_index(self.curr_dir)];
        for i in 0..curr_dir.child_count {
            let index = curr_dir.child_indexes[i];
            let dir_index = self.find_dir_index(index);
            let child = self.dirs[dir_index];

            if compare_text_arrs(child.name, name) {
                return (true, index);
            }
        }
        return (false, 0);
    }

    fn cascade_dir_delete(&mut self, dir_index: usize) {

        let mut dir = self.dirs[self.find_dir_index(dir_index)];

        for i in (0..dir.child_count).rev() {
            let child = self.dirs[self.find_dir_index(dir.child_indexes[i])];
            self.cascade_dir_delete(child.index);
        }

        // remove directory
        let index = self.find_dir_index(dir_index);
        dir = self.dirs[index];
        self.dirs[self.find_dir_index(dir.parent_index)]
            .remove_child_index_from_list(dir_index);

        self.dirs_count -= 1;
        for i in index..self.dirs_count {
            self.dirs[i] = self.dirs[i + 1];
        }
        self.dirs[self.dirs_count] = Directory::new_root();
    }
}

impl Directory {

    pub fn new_root() -> Directory {
        return Directory {
            index: 0,
            name: str_name_to_arr("root"),
            parent_index: 0,
            child_count: 0,
            child_indexes: [0; MAX_CHILDREN]
        };
    }

    pub fn new_child(&mut self, dir_index: usize, dir_name: [u8; NAME_SIZE]) -> Directory {
        self.child_indexes[self.child_count] = dir_index;
        self.child_count += 1;

        return Directory {
            index: dir_index,
            name: dir_name,
            parent_index: self.index,
            child_count: 0,
            child_indexes: [0; MAX_CHILDREN]
        };
    }

    pub fn remove_child_index_from_list(&mut self, child_index: usize) {
        let mut flag : bool = false;
        for i in 0..self.child_count {
            if self.child_indexes[i] == child_index {
                flag = true;
            } else if flag {
                self.child_indexes[i - 1] = self.child_indexes[i];
            }
        }
        if flag {
            self.child_indexes[self.child_count - 1] = 0;
            self.child_count -= 1;
        }
    }
}

fn serialize(file_system : FileSystem) -> [u8; MAX_ARRAY_SIZE] {
    let mut array : [u8; MAX_ARRAY_SIZE] = [0; MAX_ARRAY_SIZE];

    array[0] = file_system.dirs_count as u8;
    array[1] = file_system.curr_dir as u8;
    array[2] = file_system.last_index as u8;

    let mut arr_index = 3;
    for i in 0..file_system.dirs_count {
        array[arr_index] = file_system.dirs[i].index as u8;
        arr_index += 1;

        for j in 0..NAME_SIZE {
            array[arr_index] = file_system.dirs[i].name[j];
            arr_index += 1;
        }

        array[arr_index] = file_system.dirs[i].parent_index as u8;
        arr_index += 1;
        array[arr_index] = file_system.dirs[i].child_count as u8;
        arr_index += 1;

        for j in 0..MAX_CHILDREN {
            array[arr_index] = file_system.dirs[i].child_indexes[j] as u8;
            arr_index += 1;
        }
    }
    return array;
}

fn deserialize(array: [u8; MAX_ARRAY_SIZE]) -> FileSystem {
    let dirs_count = array[0] as usize;
    let curr_dir = array[1] as usize;
    let last_index = array[2] as usize;
    let mut dirs = [Directory::new_root(); MAX_DIRS];

    let mut arr_index = 3;
    for i in 0..dirs_count {
        let dir_index = array[arr_index] as usize;
        arr_index += 1;

        let mut dir_name = [0; NAME_SIZE];
        for j in 0..NAME_SIZE {
            dir_name[j] = array[arr_index];
            arr_index += 1;
        }

        let dir_parent_index = array[arr_index] as usize;
        arr_index += 1;
        let dir_child_count = array[arr_index] as usize;
        arr_index += 1;

        let mut dir_children = [0; MAX_CHILDREN];
        for j in 0..MAX_CHILDREN {
            dir_children[j] = array[arr_index] as usize;
            arr_index += 1;
        }

        dirs[i] = Directory {
            index: dir_index,
            name: dir_name,
            parent_index: dir_parent_index,
            child_count: dir_child_count,
            child_indexes: dir_children
        };
    }

    return FileSystem { dirs_count, dirs, curr_dir, last_index };
}

pub fn str_name_to_arr(text: &str) -> [u8; NAME_SIZE] {
    let mut array : [u8; NAME_SIZE] = [0; NAME_SIZE];

    let mut index : usize = 0;
    for (i, byte) in text.bytes().enumerate() {
        array[i] = byte;
        index += 1;
    }
    if index < NAME_SIZE {
        array[index] = LINE_END;
    }
    return array;
}

pub fn compare_text_arrs(text1 : [u8; NAME_SIZE], text2: [u8; NAME_SIZE]) -> bool {
    for i in 0..NAME_SIZE {
        if text1[i] != text2[i] {
            return false;
        }
        if text1[i] == text2[i] && text1[i] == LINE_END {
            return true;
        }
    }
    return true;
}

pub fn is_name_empty(name : [u8; NAME_SIZE]) -> bool {
    for i in 0..NAME_SIZE {
        if name[i] == LINE_END {
            return true;
        }
        if name[i] != (b' ' as u8) {
            return false;
        }
    }
    return true;
}

fn print_name(name: [u8; NAME_SIZE], add_backslash: bool, add_new_line: bool) {

    if add_backslash {
        print!("/");
    }

    for i in 0..NAME_SIZE {
        if name[i] == LINE_END {
            break;
        }

        print!("{}", name[i] as char);
    }

    if add_new_line {
        print!("\n");
    }
}
