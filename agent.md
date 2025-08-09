# Glossia Development Guide

## 🎯 Purpose
This guide ensures consistent, maintainable code contributions to Glossia, whether from developers or AI agents. Follow these patterns to maintain our clean architecture.

## 📁 Project Structure

```
glossia/
├── app/                      # Main application
│   ├── src/
│   │   ├── components/       # UI components (feature-based organization)
│   │   ├── hooks/           # Custom React/Dioxus hooks
│   │   ├── services/        # External service integrations
│   │   ├── utils/           # Utility functions
│   │   ├── theme.rs         # Theme system
│   │   └── main.rs          # Application entry point
│   └── assets/              # Static assets (fonts, images)
├── crates/                  # Modular Rust libraries
│   ├── http-client/         # HTTP client with retry & circuit breaker
│   ├── image-client/        # Image service abstraction
│   ├── llm-client/          # LLM service abstraction
│   ├── logging/             # Centralized logging
│   ├── navigation-service/  # Text navigation logic
│   ├── reading-engine/      # Core reading functionality
│   ├── shared/              # Shared types and errors
│   ├── text-parser/         # Text processing
│   └── vocabulary-manager/  # Vocabulary tracking
└── logs/                    # Application logs

```

## 🏗️ Architecture Principles

### 1. Feature-Based Component Organization
Components are organized by feature, not by type:

```
components/
├── common/          # Reusable, generic components
├── features/        # Feature-specific components
│   ├── vocabulary/  # Word management features
│   ├── reading/     # Reading experience
│   ├── gallery/     # Image gallery
│   ├── modals/      # Modal management
│   └── navigation/  # Navigation handlers
└── layout/          # Layout components
```

### 2. Component Size Guidelines
- **Maximum lines**: ~150 lines per component
- **When to split**: 
  - Component exceeds 150 lines
  - Multiple responsibilities evident
  - Complex state management needed
  - Reusable sub-components identified

### 3. State Management Pattern
```rust
// ✅ Good: Use custom hooks for complex state
pub fn use_app_state() -> AppState {
    // Centralized state management
}

// ❌ Bad: Complex state logic in components
#[component]
pub fn MyComponent() -> Element {
    // Don't put complex state logic here
}
```

## 📝 Adding New Features

### Step 1: Determine Feature Category

Ask yourself:
- Is this a **reusable** component? → `components/common/`
- Is this **feature-specific**? → `components/features/<feature-name>/`
- Is this a **layout** component? → `components/layout/`
- Is this a **hook**? → `hooks/`
- Is this a **utility**? → `utils/`
- Is this a **service**? → `services/`

### Step 2: Create Component Structure

For a new feature component:

```bash
# Create feature directory
mkdir -p app/src/components/features/my-feature

# Create component files
touch app/src/components/features/my-feature/mod.rs
touch app/src/components/features/my-feature/my_component.rs
touch app/src/components/features/my-feature/my_component_styles.rs
```

### Step 3: Implement Component

```rust
// my_component.rs
use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn MyComponent(
    // Props should be simple types
    theme: Theme,
    on_action: EventHandler<String>,
) -> Element {
    // Use hooks for state
    let state = use_signal(|| InitialState);
    
    rsx! {
        div {
            class: "my-component",
            // Component content
        }
    }
}
```

### Step 4: Extract Styles

```rust
// my_component_styles.rs
use crate::theme::Theme;

pub struct MyComponentStyles<'a> {
    pub theme: &'a Theme,
}

impl<'a> MyComponentStyles<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }
    
    pub fn container(&self) -> String {
        format!("background: {}; padding: 20px;", self.theme.surface)
    }
}
```

### Step 5: Update Module Exports

```rust
// mod.rs
pub mod my_component;
pub mod my_component_styles;

pub use my_component::MyComponent;
```

### Step 6: Wire Up to Main Module

```rust
// In components/features/mod.rs
pub mod my_feature;

// In components/mod.rs
pub use features::my_feature::MyComponent;
```

## 🪝 Creating Custom Hooks

### Pattern for Hooks

```rust
// use_my_feature.rs
use dioxus::prelude::*;

pub fn use_my_feature(dependency: Signal<Type>) -> FeatureState {
    let state = use_signal(|| initial_state);
    
    use_effect(move || {
        // Side effects here
    });
    
    FeatureState { state }
}
```

### Hook Guidelines
- Prefix with `use_`
- Return structured state
- Handle side effects internally
- Keep focused on single responsibility

## 🎨 Theme Integration

Always use the theme system for colors and styling:

```rust
// ✅ Good
let theme = Theme::from_mode(*theme_mode.read());
style: "color: {theme.text_primary};"

// ❌ Bad
style: "color: #000000;"
```

## 🔧 Utility Functions

Place in appropriate utils module:

```rust
// utils/text_utils.rs
pub fn process_text(text: &str) -> ProcessedText {
    // Utility logic
}
```

## 🚦 Service Integration

For external services, use the abstraction pattern:

```rust
// services/my_service.rs
pub trait MyServiceTrait {
    async fn perform_action(&self) -> Result<Output, AppError>;
}

pub struct MyService {
    client: HttpClient,
}

impl MyServiceTrait for MyService {
    // Implementation
}
```

## ✅ Checklist for New Features

Before committing new code, ensure:

- [ ] Component is under 150 lines
- [ ] Styles are extracted to separate module
- [ ] Complex logic is in custom hooks
- [ ] Theme system is used for all colors
- [ ] File is in correct feature directory
- [ ] Module exports are updated
- [ ] No inline styles (extract to style modules)
- [ ] Props are simple types, not complex objects
- [ ] Event handlers use `EventHandler<T>`
- [ ] State management uses signals/hooks

## 🚫 Anti-Patterns to Avoid

### ❌ Don't: Inline Styles
```rust
// Bad
style: "background: white; color: black;"
```

### ✅ Do: Use Theme System
```rust
// Good
style: "background: {theme.surface}; color: {theme.text_primary};"
```

### ❌ Don't: Large Components
```rust
// Bad: 300+ line component with everything
pub fn GiantComponent() -> Element {
    // Too much code here
}
```

### ✅ Do: Split into Sub-Components
```rust
// Good: Focused components
pub fn ParentComponent() -> Element {
    rsx! {
        Header { ... }
        Content { ... }
        Footer { ... }
    }
}
```

### ❌ Don't: Business Logic in Components
```rust
// Bad
#[component]
pub fn MyComponent() -> Element {
    // Complex calculations and logic here
}
```

### ✅ Do: Extract to Hooks/Utils
```rust
// Good
#[component]
pub fn MyComponent() -> Element {
    let processed_data = use_processed_data();
    // Simple rendering
}
```

## 🔍 Finding Existing Code

### By Feature
- **Vocabulary/Words**: `components/features/vocabulary/`
- **Reading**: `components/features/reading/`
- **Modals**: `components/features/modals/`
- **Image Gallery**: `components/features/gallery/`

### By Type
- **Hooks**: `hooks/use_*.rs`
- **Services**: `services/*_service.rs`
- **Utils**: `utils/*_utils.rs`
- **Types**: `crates/shared/src/types.rs`

## 🧹 Maintaining Code Quality

### Regular Maintenance Tasks

1. **Check component sizes**: 
   ```bash
   find app/src/components -name "*.rs" -exec wc -l {} \; | sort -rn
   ```

2. **Remove unused code**:
   ```bash
   cargo build 2>&1 | grep "warning:"
   ```

3. **Format code**:
   ```bash
   cargo fmt
   ```

4. **Run lints**:
   ```bash
   cargo clippy
   ```

## 📊 Current Feature Map

### Core Features
- **Text Input**: Modal for adding text to read
- **Reading Engine**: Sentence navigation and display
- **Vocabulary Manager**: Track known/unknown words
- **Word Meanings**: Display word definitions
- **Image Gallery**: Show images for words
- **Theme System**: Light/dark mode support

### UI Components
- **Modal**: Reusable modal wrapper
- **LoadingState**: Loading spinner component
- **ErrorDisplay**: Error message display
- **ProgressBar**: Reading progress indicator
- **FloatingButton**: Floating action button

### Hooks
- **use_app_state**: Central application state
- **use_simplification**: Text simplification
- **use_vocabulary**: Vocabulary management
- **use_word_meanings**: Fetch word definitions
- **use_image_cache**: Image caching

## 🆕 Example: Adding a New Feature

Let's say we want to add a "Notes" feature:

1. **Create feature structure**:
   ```bash
   mkdir -p app/src/components/features/notes
   ```

2. **Create note editor component**:
   ```rust
   // app/src/components/features/notes/note_editor.rs
   use dioxus::prelude::*;
   use crate::theme::Theme;
   
   #[component]
   pub fn NoteEditor(
       note: String,
       theme: Theme,
       on_save: EventHandler<String>,
   ) -> Element {
       let mut content = use_signal(|| note);
       
       rsx! {
           div {
               class: "note-editor",
               textarea {
                   value: "{content}",
                   oninput: move |e| content.set(e.value()),
               }
               button {
                   onclick: move |_| on_save.call(content()),
                   "Save Note"
               }
           }
       }
   }
   ```

3. **Create styles module**:
   ```rust
   // app/src/components/features/notes/note_editor_styles.rs
   use crate::theme::Theme;
   
   pub struct NoteEditorStyles<'a> {
       pub theme: &'a Theme,
   }
   
   impl<'a> NoteEditorStyles<'a> {
       pub fn container(&self) -> String {
           format!("padding: 20px; background: {};", self.theme.surface)
       }
   }
   ```

4. **Create hook for notes state**:
   ```rust
   // app/src/hooks/use_notes.rs
   use dioxus::prelude::*;
   
   pub fn use_notes() -> Signal<Vec<Note>> {
       use_signal(|| Vec::new())
   }
   ```

5. **Update module exports**:
   ```rust
   // app/src/components/features/notes/mod.rs
   pub mod note_editor;
   pub mod note_editor_styles;
   
   pub use note_editor::NoteEditor;
   ```

## 🤝 Contributing Guidelines

1. **Before starting**: Check this guide
2. **During development**: Follow the patterns
3. **Before committing**: Run the checklist
4. **After merging**: Update this guide if needed

## 🔄 Keeping This Guide Updated

When you:
- Add a new pattern → Document it here
- Find an anti-pattern → Add it to the avoid list
- Create a new feature → Update the feature map
- Discover a better way → Update the guidelines

---

*This guide is a living document. Keep it updated as the project evolves.*