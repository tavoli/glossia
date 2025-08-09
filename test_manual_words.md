# Test Plan for Manual Words Fix

## Test Scenario
1. Load text with multiple sentences
2. Navigate to a sentence containing word "apple"
3. Click on "apple" to add it to manual words list
4. Navigate to next sentence that does NOT contain "apple"
5. Verify "apple" does not appear in the word meanings list

## Expected Behavior
- Manual words should only appear in the meanings list when they are present in the current sentence
- When navigating to a sentence without the manual word, it should not be displayed

## Test Text
```
The apple is red and sweet. 
The car drives fast on the highway.
I found an apple in the basket.
```

## Test Steps
1. Load the test text
2. On sentence 1: Click "apple" - it should appear in meanings
3. Navigate to sentence 2: "apple" should NOT appear (no "apple" in this sentence)
4. Navigate to sentence 3: "apple" should appear again (has "apple" in it)