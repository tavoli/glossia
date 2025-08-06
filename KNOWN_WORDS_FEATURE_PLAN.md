# Known Words Feature Implementation Plan

## Overview

This feature implements a smart vocabulary learning system that tracks user's known words and automatically adapts the difficulty highlighting based on their learning progress.

## Core Requirements

- Save known words/phrases locally to `~/.glossia/known_words.json`
- Filter known words from difficulty highlighting in the UI
- Add words to known list via double-click on difficult words
- Auto-promote words to known after 12 encounters
- Display known words count with modal management interface

## Data Structures

### KnownWords
```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KnownWords {
    pub words: HashSet<String>,
}
```

### WordEncounters
```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WordEncounters {
    pub encounters: HashMap<String, u32>,
}
```

## File System

- `~/.glossia/known_words.json` - Set of known words/phrases
- `~/.glossia/word_encounters.json` - Map of word → encounter count

## Implementation Tasks

### Phase 1: Core Infrastructure (High Priority)

1. **Create KnownWords data structure and JSON serialization**
   - Define KnownWords struct with HashSet
   - Implement Serialize/Deserialize traits
   - Add validation and helper methods

2. **Create WordEncounters data structure to track word frequency (12 times threshold)**
   - Define WordEncounters struct with HashMap
   - Implement encounter increment logic
   - Add threshold checking (12 encounters)

3. **Implement KnownWordsManager for file operations (~/.glossia/known_words.json)**
   - Create/read/write known_words.json
   - Handle file creation if doesn't exist
   - Ensure atomic file operations

4. **Implement WordEncountersManager for file operations (~/.glossia/word_encounters.json)**
   - Create/read/write word_encounters.json
   - Handle file creation if doesn't exist
   - Ensure atomic file operations

5. **Add logic to increment word encounters when words are displayed**
   - Track when words are shown in ReadingContainer
   - Increment encounter count for each displayed word
   - Persist changes to file system

6. **Add automatic promotion logic (12 encounters -> known words)**
   - Check threshold on each encounter increment
   - Move words from encounters to known words
   - Clean up encounters data after promotion

7. **Add known words filtering logic in word highlighting**
   - Filter known words from WordMeaning list before highlighting
   - Maintain original word detection for encounter tracking
   - Preserve color consistency for remaining words

8. **Implement double-click handler to add words to known list**
   - Extend existing double-click functionality
   - Add immediate promotion option
   - Update both known words and remove from encounters

### Phase 2: UI Components (High Priority)

9. **Create KnownWordsCounter component with circular badge**
   - Circular design matching theme toggle
   - Real-time count display
   - Click handler to open modal

10. **Create KnownWordsModal component to display word list**
    - Modal overlay with word grid/list
    - Search and filter functionality
    - Remove word functionality

11. **Position KnownWordsCounter below theme toggle button**
    - Fixed position: top: 80px, right: 20px
    - Consistent styling with theme toggle
    - Z-index management

### Phase 3: Integration (Medium Priority)

12. **Update ReadingContainer to use known words filtering and encounter tracking**
    - Integrate known words filtering in word highlighting
    - Add encounter tracking on text display
    - Handle auto-promotion notifications

13. **Add progress indicators in UI (e.g., 'seen 8/12 times')**
    - Show progress on hover or in tooltips
    - Visual indicators for words close to promotion
    - Progress bar or text indicators

14. **Add remove word functionality in modal**
    - Remove button for each word
    - Confirmation dialog for removal
    - Update counter after removal

15. **Add search/filter functionality in modal**
    - Real-time search through known words
    - Filter by word length, date added, etc.
    - Sort options (alphabetical, recent, etc.)

### Phase 4: Enhancements (Low Priority)

16. **Add notification when word is auto-promoted to known**
    - Toast/snackbar notification
    - Visual feedback for auto-promotion
    - Optional sound notification

17. **Add import/export functionality for known words and encounters**
    - Export to JSON/CSV formats
    - Import from other vocabulary apps
    - Backup and restore functionality

## Data Flow

### Word Display Flow
```
Text Input → Word Detection → Check Known Words → Filter Highlighting → Display
     ↓
Increment Encounters → Check Threshold → Auto-Promote if 12+ → Update Counter
```

### Manual Learning Flow
```
Double-Click Word → Add to Known Words → Remove from Encounters → Update Counter
```

### Modal Interaction Flow
```
Click Counter → Open Modal → Display Words → Search/Filter → Remove Words → Update Files
```

## UI Layout

### Fixed Position Elements
- **Theme Toggle**: `position: fixed; top: 20px; right: 20px;`
- **Known Words Counter**: `position: fixed; top: 80px; right: 20px;`

### Counter Design
- Circular badge similar to theme toggle
- Background color indicating status
- Number display with overflow handling (999+)
- Hover effects and click animation

### Modal Design
- Full-screen overlay with backdrop
- Centered content area with max-width
- Header with title and close button
- Search bar at top
- Scrollable word grid/list
- Footer with action buttons

## File Operations

### Directory Structure
```
~/.glossia/
├── known_words.json      # Set of known words
└── word_encounters.json  # Encounter count map
```

### Error Handling
- Graceful degradation if files don't exist
- Atomic file operations to prevent corruption
- Backup creation before major updates
- Logging for debugging file operations

## Testing Strategy

### Unit Tests
- Test known words filtering logic
- Test encounter counting and threshold detection
- Test file operations and error handling
- Test auto-promotion logic

### Integration Tests
- Test word highlighting with known words
- Test modal interactions
- Test counter updates
- Test file persistence

### User Acceptance Tests
- Verify double-click adds words to known list
- Verify auto-promotion after 12 encounters
- Verify modal displays and manages words correctly
- Verify counter shows accurate count

## Performance Considerations

- Use HashSet for O(1) known word lookups
- Batch file operations to reduce I/O
- Debounce encounter updates to prevent excessive writes
- Lazy load modal content for large word lists
- Use efficient data structures for search/filter

## Security Considerations

- Validate file paths to prevent directory traversal
- Sanitize word inputs to prevent injection
- Use atomic file operations to prevent corruption
- Implement proper error handling for file operations

## Future Enhancements

- Spaced repetition algorithm for word review
- Integration with external vocabulary services
- Statistics and learning analytics
- Multi-language support
- Cloud synchronization
- Difficulty level customization
- Word categories and tagging system
