
use crate::structs::*;

// ---------- Handlers ----------

fn root_handler() {
    println!("Running myapp...");
}

fn greet_handler() {
    println!("Greeting user...");
}

fn build_handler() {
    println!("Building project...");
}

fn release_handler() {
    println!("Building release...");
}

fn test_handler() {
    println!("Running tests...");
}

fn calculate_handler() {
    println!("Calculating result...");
}

fn user_handler() {
    println!("Showing user...");
}

fn user_create_handler() {
    println!("Creating user...");
}

fn user_delete_handler() {
    println!("Deleting user...");
}

fn deploy_handler() {
    println!("Deploying application...");
}

// ---------- Tree 1: Basic commands ----------
// Tests: root handler, multiple commands,
// positional arguments, strings and missing args config.

pub fn tree_one() -> Tree {
    let mut tree = Tree::new(
        String::from("myapp"),
        Some(String::from("A basic CLI application")),
    );

    tree.handle(root_handler);

    let mut greet = Command::new(
        String::from("greet"),
        Some(greet_handler),
    );

    greet.description(String::from("Greet a user"));

    greet.add_arg(Args::new(
        1,
        vec![OptionType::Str],
    ));

    let mut build = Command::new(
        String::from("build"),
        Some(build_handler),
    );

    build.description(String::from("Build the project"));

    tree.add_command(greet);
    tree.add_command(build);

    tree
}

// ---------- Tree 2: Flags and options ----------
// Tests: flags, short names, descriptions,
// typed options, and a command with no positional args.

pub fn tree_two() -> Tree {
    let mut tree = Tree::new(
        String::from("myapp"),
        Some(String::from("Build and test projects")),
    );

    tree.version(String::from("1.0.0"));

    let mut build = Command::new(
        String::from("build"),
        Some(build_handler),
    );

    let mut release = Flag::new(String::from("release"));
    release.short(String::from("r"));
    release.description(String::from("Build in release mode"));

    let mut verbose = Flag::new(String::from("verbose"));
    verbose.short(String::from("v"));
    verbose.description(String::from("Show detailed output"));

    let mut jobs = OptionSpec::new(
        String::from("jobs"),
        OptionType::Int,
    );

    jobs.short(String::from("j"));
    jobs.description(String::from("Number of build jobs"));

    build.add_flag(release);
    build.add_flag(verbose);
    build.add_option(jobs);

    let mut test = Command::new(
        String::from("test"),
        Some(test_handler),
    );

    test.description(String::from("Run project tests"));

    tree.add_command(build);
    tree.add_command(test);

    tree
}

// ---------- Tree 3: Nested subcommands ----------
// Tests: multiple nesting levels, arguments,
// handlers at parent and child levels.

pub fn tree_three() -> Tree {
    let mut tree = Tree::new(
        String::from("myapp"),
        Some(String::from("Manage users")),
    );

    let mut user = Command::new(
        String::from("user"),
        Some(user_handler),
    );

    user.description(String::from("Manage user accounts"));

    let mut create = Command::new(
        String::from("create"),
        Some(user_create_handler),
    );

    create.description(String::from("Create a user"));

    create.add_arg(Args::new(
        1,
        vec![OptionType::Str],
    ));

    let mut delete = Command::new(
        String::from("delete"),
        Some(user_delete_handler),
    );

    delete.description(String::from("Delete a user"));

    delete.add_arg(Args::new(
        1,
        vec![OptionType::Int],
    ));

    user.add_subcommand(create);
    user.add_subcommand(delete);

    tree.add_command(user);

    tree
}

// ---------- Tree 4: Mixed configuration ----------
// Tests: multiple positional types, flags,
// options of different types, and subcommands.

pub fn tree_four() -> Tree {
    let mut tree = Tree::new(
        String::from("myapp"),
        Some(String::from("Application deployment tool")),
    );

    tree.version(String::from("2.0.0"));

    let mut deploy = Command::new(
        String::from("deploy"),
        Some(deploy_handler),
    );

    deploy.description(String::from("Deploy an application"));

    deploy.add_arg(Args::new(
        2,
        vec![
            OptionType::Str,
            OptionType::Int,
        ],
    ));

    let mut force = Flag::new(String::from("force"));
    force.short(String::from("f"));

    let mut timeout = OptionSpec::new(
        String::from("timeout"),
        OptionType::Float,
    );

    timeout.short(String::from("t"));

    deploy.add_flag(force);
    deploy.add_option(timeout);

    let mut release = Command::new(
        String::from("release"),
        Some(release_handler),
    );

    release.description(String::from("Create a release"));

    release.add_arg(Args::new(
        1,
        vec![OptionType::Str],
    ));

    deploy.add_subcommand(release);

    tree.add_command(deploy);

    tree
}
