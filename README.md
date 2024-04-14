
# Megalopa

Megalopa is an accessible framework for crafting static sites. 🦀
Organize and grow your larva sized brain in the vast sea of technical knowledge. Use your abdominal appendage to propel yourself forward. Serve it in a website or something idk you'll figure it out.


## Installation
TODO

## Usage

```bash
# Initialize a new project in the current directory
megalopa init <project-name>

# Build the html pages
megalopa

# Create new md page
megalopa new <page-name>

# Host a local development server on port 3000
megalopa dev

```

## Project Directory Structure
```bash
Project-Name/
├── larvae.yaml # Config file
├── content/  # Where you write your content
│   ├── post1.md
│   ├── post2.md 
│   └── ...
│
├── public/  # Where the generated site is stored
│   ├── index.html
│   ├── post1.html
│   ├── post2.html
│   └── ...

```

## Templates
Megalopa uses the [Tera](https://keats.github.io/tera/) templating engine. You can overwrite the default templates by creating a `templates` directory in the root of your project.

Megalopa comes with default templates defined as follows:

base.html - The base template that all other templates extend from.
index.html - The template for the index pages. Including a modifiable list of content.
content.html - The template for content pages.
homepage.html - The template for the homepage.

