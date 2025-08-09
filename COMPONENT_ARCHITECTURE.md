# Component Architecture Guide

## Overview
This document outlines the component organization strategy implemented for long-term maintainability and growth.

## Folder Structure

```
app/src/components/
├── common/           # Shared, reusable components
│   └── modals/      # Generic modal components
├── features/        # Feature-specific components  
│   └── vocabulary/  # Vocabulary management components
├── layout/          # Layout components (future)
└── [legacy]         # Original flat components (to be migrated)
```

## Refactoring Completed

### 1. KnownWordsModal Decomposition
**Before:** Single 241-line component with inline styles
**After:** Modular structure with:
- `known_words_modal.rs` - Main component (78 lines)
- `known_words_modal_styles.rs` - Extracted styles
- `modal_header.rs` - Header sub-component
- `search_bar.rs` - Search functionality
- `word_grid.rs` - Word display grid
- `empty_state.rs` - Empty state component

### 2. Reusable Modal Component
Created `common/modals/modal.rs` providing:
- Generic modal wrapper with overlay
- Configurable width
- Click-outside-to-close functionality
- Theme integration

### 3. Custom Hooks
Created specialized hooks for business logic:
- `use_word_tracking.rs` - Word encounter tracking
- Enhanced error boundary with circuit breaker pattern

## Best Practices Applied

### 1. Component Size
- Keep components under 150 lines
- Extract complex logic into hooks
- Split large components into sub-components

### 2. Style Management
- Extract inline styles to dedicated modules
- Use style structs for type-safe styling
- Centralize theme-dependent styles

### 3. Composition Pattern
- Build complex UIs from smaller components
- Use wrapper components for common patterns
- Separate presentation from business logic

### 4. State Management
- Use custom hooks for complex state logic
- Keep component state minimal
- Prefer composition over prop drilling

## Migration Roadmap

### Phase 1: High Priority (Completed)
- [x] KnownWordsModal refactoring
- [x] Create reusable Modal wrapper
- [x] Establish folder structure

### Phase 2: Medium Priority (Completed)
- [x] Refactor App component (171 lines → 80 lines)
  - Extracted modal management to ModalManager
  - Created AppLayout component for layout concerns
  - Separated keyboard event handling into KeyboardHandler
  
- [x] Refactor MainContent component (147 lines → 95 lines)
  - Extracted loading states to LoadingState component
  - Created ContentDisplay for content rendering
  - Added SentenceProcessor for side effects
  - Simplified effect handling with dedicated hooks

- [x] Split styles.rs into feature modules
  - WordMeaningsStyles → features/vocabulary/
  - ImageGalleryStyles → features/gallery/

### Phase 3: Component Migration (Next Steps)
- [ ] Move remaining reading components to `features/reading/`
- [ ] Move settings components to `features/settings/`
- [ ] Create common button components
- [ ] Create common form components
- [ ] Migrate TextInputModal to use Modal wrapper

## Component Guidelines

### Creating New Components
1. Place in appropriate feature folder
2. Extract styles to separate module
3. Keep focused on single responsibility
4. Use composition for complex UIs
5. Write accompanying tests

### Refactoring Existing Components
1. Identify components over 150 lines
2. Extract business logic to hooks
3. Split into logical sub-components
4. Move inline styles to style modules
5. Update imports and tests

## Benefits Achieved

1. **Better Organization** - Components grouped by feature
2. **Improved Readability** - Smaller, focused components
3. **Enhanced Reusability** - Generic components like Modal
4. **Easier Testing** - Isolated component logic
5. **Scalable Structure** - Clear patterns for growth

## Technical Debt Remaining

1. **App Component** - Still managing too many responsibilities
2. **Inline Styles** - Many components still have inline styles
3. **Prop Drilling** - Some areas could benefit from context
4. **Test Coverage** - New components need tests