# flutter_wizard

## Product Overview

### Problem Statement

In Flutter app development, after creating a project with `flutter create`, developers must manually configure the same architecture setup, add packages, and organize folder structures every time — a process that is time-consuming and error-prone. In team development, different members often create projects with inconsistent structures, making it difficult to maintain a unified codebase.

`flutter_wizard` solves these problems by providing an interactive wizard that allows developers to select their architecture, state management library, and additional options to automatically generate a unified Flutter project from a template.

### Usage Patterns

**Basic Usage Flow**

Running `flutter_wizard` launches an interactive wizard that guides the user through the following steps to generate a project:

1. Enter project name and org name
2. Select target platforms (Android / iOS / Web / Windows / macOS / Linux)
3. Select architecture (Clean Architecture / MVVM / MVC / Layered Architecture)
4. Select package categories (State Management / Database / DI / Data Model / Code Generation / Networking / Routing / Logger, etc.)
5. Select packages within each chosen category (multiple selection supported)
6. Configure options for selected packages (e.g., Riverpod → use code generation / use flutter_hooks)
7. Configure environment separation (develop / staging / production, environment names are customizable)
8. Select additional options (CI/CD file generation / License file, etc.)
9. Preview selections on a confirmation screen, then execute generation

**Saving and Reusing Settings**

After project generation completes, the user is prompted to choose whether to save the current configuration as default settings or export it as a YAML file. Saved settings can be used as default values in future wizard sessions, with per-item confirmation at each step.

**Generating from a Config File**

By specifying a YAML config file with `flutter_wizard --config <file>`, the user only needs to enter the project name to generate a project. Teams can share config files to ensure consistent project structures.

**Environment Separation**

Generates boilerplate files with environment-specific settings (app name, Bundle ID, app icon, etc.) for each environment such as develop / staging / production. Environment names are customizable in the wizard. Generates directory structures compatible with Flutter flavors (`--flavor`) and `.env.<environment>` files for `dart-define`.

**Plugin-based Package Extension**

Package definitions are designed to be externally extensible via plugins (YAML files). New packages or categories can be added by placing a plugin file in `~/.config/flutter_wizard/plugins/`, and they will automatically appear as options in the wizard.

### Objectives

- **Improve Development Efficiency** — Automate project initial setup through an interactive wizard, significantly reducing time spent on manual configuration
- **Unify Project Structure** — Eliminate inconsistencies in architecture and package configurations within a team, maintaining a consistent codebase
- **Share Best Practices** — Define and share the team's standard project structure through templates and config files, reducing onboarding costs for new members

## Functional Requirements

### 1. Project Generation

- Running `flutter_wizard` launches an interactive wizard to generate a project
- The wizard accepts project name and org name as input
- Project name is validated against Dart package naming rules (lowercase letters, digits, and underscores only); invalid names display an error message and prompt re-entry
- Org name is validated as a reverse domain format (e.g., `com.example`)
- Projects are generated in the current working directory
- The `--config <file>` option accepts a YAML config file, requiring only the project name to generate a project
- A preview of all selections is displayed before generation; the user confirms before proceeding
- When the target directory already exists, a confirmation prompt is shown (`--force` to override without prompt)
- `flutter pub get` is automatically executed after generation completes

### 2. Architecture Selection

- The wizard allows selection of one of the following architectures:
  - Clean Architecture (`domain / data / presentation` layer structure)
  - MVVM (`model / view / viewmodel` structure)
  - MVC (`model / view / controller` structure)
  - Layered Architecture (simple `ui / logic / data` structure)
- Generates directory structure and empty boilerplate files (class definitions only) for the selected architecture
- Architecture templates are managed in an external directory (`~/.config/flutter_wizard/templates/`)
- No restrictions are imposed on architecture and state management library combinations

### 3. Package Management

- The wizard allows multiple package categories to be selected (skipping all is allowed)
- Multiple packages can be selected within each chosen category
- If a selected package has options, additional configuration is prompted
- Initial categories and packages provided are as follows:

| Category | Package Options |
|---|---|
| State Management | Riverpod / Bloc / GetX |
| Database | drift / isar / sqflite |
| DI | get_it / injectable |
| Data Model | freezed / built_value |
| Code Generation | build_runner / flutter_gen |
| Networking | dio / http |
| Routing | go_router / auto_route |
| Logger | logger / talker |

- Riverpod options: use code generation / use flutter_hooks
- Selected packages are automatically added to `pubspec.yaml`
- All package categories can be skipped to generate a minimal project
- Categories, packages, and options can be externally added and extended via plugins

### 4. Environment Separation

- The wizard allows customization of environment names (defaults: develop / staging / production)
- Generates `.env.<environment>` files with the following default settings per environment:
  - App name (`APP_NAME`)
  - Bundle ID / Application ID (`BUNDLE_ID`)
- `flutter_launcher_icons` is included as a required package, generating per-environment icon config files (`flutter_launcher_icons-<environment>.yaml`)
- Generates directory structures compatible with Flutter flavors (`--flavor`)
- Generates environment variable files in `.env.<environment>` format compatible with `dart-define`
- Environment separation can be skipped (when skipped, `flutter_launcher_icons` becomes optional)

### 5. Additional Options

- The wizard allows each of the following options to be individually selected or skipped

**CI/CD (GitHub Actions)**
- The following workflows can be individually selected and generated:
  - Format and lint check (`flutter analyze` / `dart format`)
  - Test execution (`flutter test`)
  - Release creation (tagged release to GitHub Releases)
  - Store distribution (Google Play / App Store deployment)
- Generates `.github/workflows/` files for selected workflows

**License**
- Generates a `LICENSE` file (selectable from MIT / Apache 2.0 / GPL, etc.)

### 6. Configuration Management

- After project generation completes, the following options are presented:
  - Save current configuration as default settings
  - Export current configuration as a YAML file
  - Do nothing
- Items included in default settings and exported files:
  - Org name
  - Architecture
  - Target platforms
  - Package configuration
  - Environment separation settings
  - CI/CD settings
  - License settings
- Default settings are saved to `~/.config/flutter_wizard/config.yaml`
- Exported config files are output to the current directory as `flutter_wizard.yaml`
- When `--config <file>` is specified, only the project name is required to generate a project
- When a config file is specified, the wizard confirms per item whether to use the config file value (Y/n)
  - e.g., "Use config file settings for CI/CD? (Y/n)"
- When `~/.config/flutter_wizard/config.yaml` exists at wizard startup, per-item confirmation is prompted in the same manner (Y/n)
- `flutter_wizard config --show` displays the current default configuration
- `flutter_wizard config --reset` resets the default configuration
- All config files use YAML format

### 7. Plugins

- Categories, packages, and options can be externally added and extended via plugins
- Plugins are defined as YAML files and auto-discovered when placed in `~/.config/flutter_wizard/plugins/`
- Plugins can define the following:
  - New categories
  - Additional packages for existing categories
  - Package option definitions
  - Dependencies to add to `pubspec.yaml` when a package is selected
  - Template file paths for boilerplate files generated when a package is selected

### 8. Template Management

- Template files are managed in `~/.config/flutter_wizard/templates/`
- Templates are organized into directories per architecture:
  - e.g., `templates/clean_architecture/` / `templates/mvvm/` / `templates/mvc/` / `templates/layered/`
- Tera syntax (`{{ project_name }}`, etc.) is used for variable substitution in template files
- `flutter_wizard template --list` displays available templates
- `flutter_wizard template --init` expands default templates into `~/.config/flutter_wizard/templates/`

## Tech Stack

### Development Environment

- Language: Rust (latest stable)
- Supported OS: No restrictions (Windows / macOS / Linux)

### Key Libraries and Frameworks

| Crate | Purpose |
|---|---|
| clap v4 | CLI command and argument parsing |
| tera | Template engine (variable substitution) |
| serde / serde_yaml | Reading and writing YAML config files |
| TBD (inquire / dialoguer) | Interactive wizard |

## Notes

None
