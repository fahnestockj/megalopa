
# Megalopa

A static site generator built from scratch!

Including:
- A Markdown -> HTML converter
- A web server with live reload
- A CLI (using clap)

WIP:
- A Mustache 🥸 compliant HTML templating engine


### Installation
TODO

### Usage

```bash
# Initialize a new project in the current directory
megalopa init <project-name>

# Build your pages
megalopa build

# Create new md page
megalopa new <page-name>

# Host a local development server on port 3000
megalopa dev

```

### Project Directory Structure
```bash
Project-Name/
├── larvae.yaml # Config file
├── content/  # Where you write your content
│   ├── post1.md
│   └── ...
│
├── public/  # Where the generated site is stored
│   ├── index.html
│   ├── post1.html
│   └── ...

```

### Templates
TODO: overrides not implemented yet
