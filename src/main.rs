mod api;
mod cli;
mod git;
mod makepkg;
mod pacman;

use std::path::Component;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

fn install_packages(packages: Vec<String>) -> Result<()> {
    if packages.is_empty() {
        println!("No package specified");
        return Ok(());
    }

    println!("{}", "Checking packages...".bold());

    let response = api::get_packages_info(&packages[..]).unwrap();

    let found_packages = response
        .results
        .iter()
        .map(|p| p.name.as_str())
        .collect::<Vec<_>>();

    let mut invalid_packages = false;
    for wanted_package in packages {
        if !found_packages.contains(&wanted_package.as_str()) {
            println!("Package '{}' not found", wanted_package);
            invalid_packages = true;
        }
    }

    if invalid_packages {
        return Ok(());
    }

    println!("{}", "--- Recap ---".bold());
    for package in &response.results {
        println!("{} - {}", package.name, package.version);
    }

    // Install missing dependencies

    let needed_deps = response
        .results
        .iter()
        .flat_map(|package| &package.depends)
        .map(|package| pacman::strip_package_version(package))
        .filter(|package| !pacman::is_package_installed(package).unwrap_or(false))
        .collect::<Vec<_>>();

    if !needed_deps.is_empty() {
        println!();
        println!("{}", "Installing missing dependencies...".bold());

        pacman::install_packages(&needed_deps[..])?;
    }

    // Cloning
    for package in &response.results {
        println!();
        println!("-- Cloning {}...", package.name);
        git::clone(&format!("https://aur.archlinux.org/{}.git", package.name))?;
    }

    let mut pkg_files = Vec::with_capacity(response.resultcount as usize);

    for package in &response.results {
        println!();
        println!("-- Building {}...", package.name);

        makepkg::build(&package.name)?;

        // Find .pkg.tar.zst files
        let pkgs = makepkg::find_pkg_files(&package.name)?;
        pkg_files.extend(pkgs);
    }

    println!();
    println!("{}", "Installing built packages...".bold());
    pacman::install_local_pacakges(&pkg_files)?;

    println!();
    for package in &response.results {
        println!("-- Cleaning up {}...", package.name);
        git::clean(&package.name)?;
    }

    Ok(())
}

fn update_packages() -> Result<()> {
    println!("{}", "Finding packages to update...".bold());

    let paths = std::fs::read_dir(".")?
        .filter_map(|dir_entry| dir_entry.ok())
        .filter(|dir_entry| match dir_entry.file_type() {
            Ok(file_type) => file_type.is_dir(),
            Err(_) => false,
        })
        .map(|dir_entry| dir_entry.path())
        .filter(|path| path.join("PKGBUILD").is_file())
        .filter(|path| git::pull(path).ok().unwrap_or(false))
        .collect::<Vec<_>>();

    let packages_to_update = paths
        .iter()
        .filter_map(|path| path.components().next_back())
        .filter_map(|component| match component {
            Component::Normal(os_str) => Some(os_str),
            _ => None,
        })
        .filter_map(|os_str| os_str.to_str())
        .collect::<Vec<_>>();

    if packages_to_update.is_empty() {
        println!("No updates");
        return Ok(());
    }

    println!();
    println!("Packages needing update: {}", packages_to_update.join(", "));

    let mut pkg_files = Vec::with_capacity(packages_to_update.len());

    for package in &packages_to_update {
        println!();
        println!("-- Building {}...", package);

        makepkg::build(package)?;

        // Find .pkg.tar.zst files
        let pkgs = makepkg::find_pkg_files(package)?;
        pkg_files.extend(pkgs);
    }

    println!();
    println!("{}", "Installing built packages...".bold());
    pacman::install_local_pacakges(&pkg_files)?;

    for package in &packages_to_update {
        println!();
        println!("-- Cleaning up {}...", package);
        git::clean(package)?;
    }

    Ok(())
}

fn search_package(term: &str) -> Result<()> {
    let mut response = api::search_package(term).unwrap();

    if let Some(error_mesage) = response.error {
        println!("{}", error_mesage);
        return Ok(());
    }

    response
        .results
        .sort_by(|a, b| a.popularity.total_cmp(&b.popularity));

    println!("{}", "Matches:".bold());
    for package in &response.results {
        let description = package.description.as_deref().unwrap_or("<No description>");

        println!(
            "{} - {}\n  {}",
            package.name.bold(),
            package.version.bold().blue(),
            description,
        );
    }

    Ok(())
}

fn main() {
    let command = cli::Command::parse();

    if let Err(error) = match command {
        cli::Command::Update => update_packages(),
        cli::Command::Install { packages } => install_packages(packages),
        cli::Command::Search { term } => search_package(&term),
    } {
        println!("error occured: {}", error);
    }
}
