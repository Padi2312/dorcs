use std::fs;
use std::io::{self, Write};

use serde_json::json;

pub fn setup() {
    if docs_directory_exists() {
        return;
    }

    println!("#########################");
    println!("### Welcome to Dorcs! ###");
    println!("#########################");
    println!();
    println!("🧙 Welcome to your first adventure with Dorcs! Allow me to be your guide.");
    println!();

    let mut name = ask_for_source_dir();
    if name.is_empty() {
        name = "docs".to_owned();
        create_directory("docs");
        create_index_file("./docs/index.md");
    } else {
        println!(
            "Ah, great choice! I will create a directory named '{}' for you. 📁",
            &name
        );
        create_directory(&name);
        println!("I will also create an 'index.md' file being the starting point for a great documentation. 📄");
        create_index_file(&format!("{}/index.md", &name));
    }

    if !dorcs_config_exists() {
        println!();
        println!("I will create a 'dorcs.config.json' file containing a magic spell giving you autocompletion powers❗");
        write_dorcs_config(&name);
    }

    println!("🎉 Dorcs is now set up and ready to use! 🚀");
    println!();
    println!("I will now create your very first documentation with Dorcs.📄");
    println!("Have a look at the 'output' directory. 📂");
    println!();
    println!(
        "For hosting your documentation, you can use any service able to host static websites. 🌐"
    );
    println!();
}

fn docs_directory_exists() -> bool {
    fs::metadata("docs").is_ok()
}

fn dorcs_config_exists() -> bool {
    fs::metadata("dorcs.config.json").is_ok()
}

fn ask_for_source_dir() -> String {
    print!("Enter a name for the source directory (or press Enter to use 'docs'): ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    name.trim().to_string()
}

fn create_directory(directory: &str) {
    fs::create_dir(directory).expect("Failed to create directory");
}

fn create_index_file(path: &str) {
    let content = r#"# Hello Dorcs👋
This is the starting point of your documentation. You can start writing here. 🚀

## 🎯Getting Started
You can start by creating a new file in this directory. You can also create a new directory and start writing there. \
There are no limitations, you can create as many files and directories as you want. 📂

Create another file and see how Dorcs generates a sidebar navigation. 📄

## 📝 Markdown
Dorcs uses Markdown to generate the documentation. You can use Markdown to style your documentation.
You can write **bold text**, *italic text*, `code blocks`, and much more. 

---

Have a look at the [Markdown guide](https://www.markdownguide.org) to get started with Markdown. 📖

## ❓Help 
If you need help, you can check the [Dorcs documentation](https://dorcs.allthing.eu). (Of course generated with Dorcs)

## Have fun!
Now it's your turn! Have fun writing your documentation. ✍️


(You can delete this content and start writing your own content.) \
(And yes i think with emojis everything looks more comfortable and not so boring 😄)
    "#;
    fs::write(path, content).expect("Failed to create index file");
}

fn write_dorcs_config(source_dir: &str) {
    let config = json!({
        "$schema": "https://dorcs.allthing.eu/dorcs.config.schema.json",
        "source": source_dir
    });
    let result = serde_json::to_writer_pretty(
        fs::File::create("dorcs.config.json").expect("Failed to create Dorcs config file"),
        &config,
    )
    .expect("Failed to write Dorcs config");
    // fs::write("./dorcs.config.json", config).expect("Failed to write Dorcs config");
}
