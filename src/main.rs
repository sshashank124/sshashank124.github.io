mod data;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use data::{About, Config, DataSource};

static DATA_DIR: &str = "data";
static TEMPLATES_DIR: &str = "templates";
static STYLESHEETS_DIR: &str = "stylesheets";
static STATIC_DIR: &str = "static";
static OUTPUT_DIR: &str = "docs";

fn create_dir_if_nonexistent(dir: impl AsRef<Path>) -> Result<()> {
    let dir = dir.as_ref();
    if !fs::exists(dir).with_context(|| {
        format!(
            "Failed to check if directory already exists: {}",
            dir.display()
        )
    })? {
        fs::create_dir(dir)
            .with_context(|| format!("Failed to create directory: {}", dir.display()))?;
    }
    Ok(())
}

fn recreate_output_dir() -> Result<()> {
    println!("Recreating output dir ...");

    if fs::exists(OUTPUT_DIR)
        .with_context(|| format!("Failed to check if directory already exists: {OUTPUT_DIR}"))?
    {
        fs::remove_dir_all(OUTPUT_DIR).with_context(|| "Failed to delete output directory")?;
    }

    create_dir_if_nonexistent(OUTPUT_DIR)
        .with_context(|| format!("Failed to create output directory: {OUTPUT_DIR}"))?;

    println!("Recreated output dir");
    Ok(())
}

pub fn load_data<D: DataSource>() -> Result<D> {
    println!("Loading '{}' data ...", D::NAME);

    let file_path = PathBuf::from(DATA_DIR).join(D::NAME).with_extension("json");

    let json_data = fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read {}", file_path.display()))?;

    let data = serde_json::from_str(&json_data)
        .with_context(|| format!("Failed to parse {}", file_path.display()))?;

    println!("Loaded '{}' data", D::NAME);
    Ok(data)
}

fn render_templates(about: &About, config: &Config) -> Result<()> {
    println!("Rendering templates ...");

    let tera = {
        let templates_glob = PathBuf::from(TEMPLATES_DIR).join("**").join("*");
        tera::Tera::new(&templates_glob.to_string_lossy()).with_context(|| {
            format!(
                "Failed to load templates under glob: {}",
                templates_glob.display()
            )
        })?
    };

    let context = {
        fn add_data_to_context<D: DataSource>(context: &mut tera::Context, d: &D) -> Result<()> {
            context.try_insert(D::NAME, d).with_context(|| {
                format!(
                    "Failed to add '{}' to template rendering context",
                    About::NAME
                )
            })
        }

        let mut context = tera::Context::new();
        add_data_to_context(&mut context, about)?;
        add_data_to_context(&mut context, config)?;
        context
    };

    for page in &config.pages {
        let dst_path = PathBuf::from(OUTPUT_DIR).join(&page.link);

        let rendered_page = tera
            .render(&page.link, &context)
            .with_context(|| format!("Failed to render template: {}", &page.link))?;

        fs::write(&dst_path, rendered_page)
            .with_context(|| format!("Failed to write rendered page to {}", dst_path.display()))?;
    }

    println!("Rendered templates");
    Ok(())
}

fn compile_stylesheets() -> Result<()> {
    println!("Compiling stylesheets ...");

    let stylesheets_to_compile: Vec<_> = fs::read_dir(STYLESHEETS_DIR)
        .with_context(|| format!("Failed to read directory contents: {STYLESHEETS_DIR}"))?
        .map(|e| {
            Ok(e.with_context(|| "Failed to read directory entry")?
                .file_name()
                .to_string_lossy()
                .into_owned())
        })
        .filter(|name| name.as_ref().is_ok_and(|v| !v.starts_with('_')))
        .collect::<Result<_>>()
        .with_context(|| "Failed to get list of stylesheets to compile")?;

    let output_dir = PathBuf::from(OUTPUT_DIR).join(STYLESHEETS_DIR);
    create_dir_if_nonexistent(&output_dir).with_context(|| {
        format!(
            "Failed to create stylesheets output directory: {}",
            output_dir.display()
        )
    })?;

    let compilation_options = grass::Options::default();
    for stylesheet_name in stylesheets_to_compile {
        let src_path = PathBuf::from(STYLESHEETS_DIR).join(&stylesheet_name);
        let dst_path = output_dir.join(&stylesheet_name).with_extension("css");

        let compiled_stylesheet = grass::from_path(&src_path, &compilation_options)
            .with_context(|| format!("Failed to compile stylesheet: {}", src_path.display()))?;

        fs::write(&dst_path, &compiled_stylesheet).with_context(|| {
            format!(
                "Failed to write compiled stylesheet to {}",
                dst_path.display()
            )
        })?;
    }

    println!("Compiled stylesheets");
    Ok(())
}

fn copy_over_static_files() -> Result<()> {
    fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
        create_dir_if_nonexistent(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    println!("Copying over static files ...");

    copy_dir_all(STATIC_DIR, OUTPUT_DIR)
        .with_context(|| "Failed to copy static files to output")?;

    println!("Copied over static files");
    Ok(())
}

fn main() -> Result<()> {
    println!("Generating website ...");

    let about: About = load_data().with_context(|| "Failed to load About data")?;
    let config: Config = load_data().with_context(|| "Failed to load Config data")?;

    recreate_output_dir().with_context(|| "Failed to recreate output directory")?;
    render_templates(&about, &config).with_context(|| "Failed to render templates")?;
    compile_stylesheets().with_context(|| "Failed to compile stylesheets")?;
    copy_over_static_files().with_context(|| "Failed to copy over static files")?;

    println!("Generated website");
    Ok(())
}
