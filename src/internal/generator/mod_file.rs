use crate::internal::generator::file_generator::write_to_file;
use crate::internal::generator::types::TypeGenerator;
use crate::internal::model::Model;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

struct ZserioModuleTreeNode {
    children: HashMap<String, Rc<RefCell<ZserioModuleTreeNode>>>,
}

fn add_package_to_tree(
    type_generator: &mut TypeGenerator,
    tree_node: &Rc<RefCell<ZserioModuleTreeNode>>,
    pkg_name: &str,
) {
    let mut current_node: Rc<RefCell<ZserioModuleTreeNode>> = tree_node.clone();
    //let mut current_node = tree_node;
    for module_path in pkg_name.split('.') {
        let rust_module_name = type_generator.to_rust_module_name(module_path);

        if let Some(child_node) = current_node
            .clone()
            .borrow()
            .children
            .get(&rust_module_name)
        {
            // The module is already known.
            current_node = child_node.clone();
            continue;
        }
        // The module is not known yet. Need to create it.
        current_node.borrow_mut().children.insert(
            rust_module_name.clone(),
            Rc::from(RefCell::from(ZserioModuleTreeNode {
                children: HashMap::new(),
            })),
        );
        let new_node = current_node
            .borrow()
            .children
            .get(&rust_module_name)
            .unwrap()
            .clone();
        current_node = new_node;
    }
}

fn generate_mod_section(
    file_content: &mut String,
    mod_name: &str,
    module_tree_node: &Rc<RefCell<ZserioModuleTreeNode>>,
) {
    if module_tree_node.borrow().children.is_empty() {
        *file_content += format!("pub mod {};\n", mod_name).as_str();
        return;
    }

    if !mod_name.is_empty() {
        *file_content += format!("pub mod {} {{\n", mod_name).as_str();
    }
    // Generate the "pub mod X;" for all children.
    for (child_name, child_node) in module_tree_node.borrow().children.iter() {
        generate_mod_section(file_content, child_name, child_node);
    }

    if !mod_name.is_empty() {
        *file_content += "}\n";
    }
}

/// This function generates the top-level mod file, which adds all generated packages
/// to the known packages.
pub fn generate_top_level_mod_file(
    type_generator: &mut TypeGenerator,
    model: &Model,
    package_directory: &Path,
    root_package: &str,
) {
    // Reorganize the packages into a tree structure, categorized by module.
    let module_tree_root = Rc::from(RefCell::from(ZserioModuleTreeNode {
        children: HashMap::new(),
    }));

    for package in model.packages.values() {
        add_package_to_tree(type_generator, &module_tree_root, &package.name);
    }

    // Generate the mod.rs file content.
    let mut mod_file_content = String::from("");
    // https://github.com/Danaozhong/rust-zserio/issues/28
    // The generated code does not pass clippy format checks.
    // Fixing these is moved to later, once the generated code works correctly.
    mod_file_content += "#![allow(clippy::all)]\n";
    mod_file_content += "#![allow(warnings)]\n";

    generate_mod_section(&mut mod_file_content, "", &module_tree_root);

    // Generate a "mod.rs" file, if the zserio libraries are generated into a
    // subdirectory. If the library is generated on the top level, the "lib.rs"
    // name is used.
    let mut filename = "mod";
    if root_package.is_empty() {
        filename = "lib";
    }

    write_to_file(
        type_generator,
        &mod_file_content,
        package_directory,
        "",
        filename,
    );
}
