# Unzip Rust Algorithm

This project is a script written in Rust that extracts the contents of a ZIP archive to the local file system. It reads a ZIP file supplied as a command line argument and extracts all the files and directories contained in it to the current or specified directory.


## Prerequisites
<ul>
    <li><b><a href="https://www.rust-lang.org/tools/install">Cargo:</a></b> 1.78.0</li>
    <li><b><a href="https://www.git-scm.com/downloads">Git:</a></b> 2.39.2</li>
</ul>

##### Compile the Project

```bash
    cargo build --release
```

##### Build and Run the project

```bash
    cargo run {target} 
```

###### Example 

```bash
    cargo run README.zip
```
